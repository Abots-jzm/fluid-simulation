use macroquad::prelude::*;
use rayon::prelude::*;

use crate::{
    config::{Config, FluidSpawnMode, InteractionType},
    grid::GridBox,
    particle::Particle,
    physics::Physics,
};

pub struct Fluid {
    pub grid: Vec<GridBox>,
    grid_cols: usize,
    grid_rows: usize,
}

impl Fluid {
    pub fn from_config(config: &Config) -> Self {
        let mut particles = Vec::new();
        match config.fluid_spawn_mode {
            FluidSpawnMode::Random => {
                for _ in 0..config.particle_count {
                    let x = rand::gen_range(
                        config.boundary_padding + config.particle_radius,
                        screen_width() - config.boundary_padding - config.particle_radius,
                    );
                    let y = rand::gen_range(
                        config.boundary_padding + config.particle_radius,
                        screen_height() - config.boundary_padding - config.particle_radius,
                    );
                    particles.push(Particle::new(Vec2::new(x, y), config.particle_radius));
                }
            }
            _ => {
                let target_aspect_ratio = screen_width() / screen_height();

                let rows: u32;
                let cols: u32;

                let particle_count_f = config.particle_count as f32;

                let rows_f = (particle_count_f / target_aspect_ratio).sqrt();
                let cols_f = (particle_count_f * target_aspect_ratio).sqrt();
                rows = rows_f.round() as u32;
                cols = cols_f.round() as u32;

                let spacing = config.particle_radius * 2.5;

                let total_width = cols as f32 * spacing;
                let left_offset = (screen_width() - total_width) / 2.0;
                let total_height = rows as f32 * spacing;
                let top_offset = (screen_height() - total_height) / 2.0;

                for i in 0..rows {
                    for j in 0..cols {
                        let x = left_offset + j as f32 * spacing;
                        let y = i as f32 * spacing + top_offset;
                        particles.push(Particle::new(Vec2::new(x, y), config.particle_radius));
                    }
                }
            }
        }

        let grid_size = config.smoothing_radius;
        let grid_cols = (screen_width() / grid_size).ceil() as usize;
        let grid_rows = (screen_height() / grid_size).ceil() as usize;

        let mut grid = Vec::with_capacity(grid_cols * grid_rows);
        for i in 0..grid_rows {
            for j in 0..grid_cols {
                let x = j as f32 * grid_size;
                let y = i as f32 * grid_size;
                grid.push(GridBox::new(grid_size, Vec2::new(x, y)));
            }
        }

        for particle in &particles {
            let grid_x = (particle.position.x / grid_size).floor() as usize;
            let grid_y = (particle.position.y / grid_size).floor() as usize;
            let index = grid_y * grid_cols + grid_x;

            if index < grid.len() {
                grid[index].add_particle(particle.clone());
            }
        }

        Self {
            grid,
            grid_cols,
            grid_rows,
        }
    }

    pub fn draw(&self) {
        for particle in self.grid.iter().flat_map(|grid_box| &grid_box.particles) {
            particle.draw(750.);
        }

        for grid_box in &self.grid {
            grid_box.draw();
        }
    }

    fn get_grid_dimensions(&self, grid_size: f32) -> (usize, usize) {
        let grid_cols = (screen_width() / grid_size).ceil() as usize;
        let grid_rows = (screen_height() / grid_size).ceil() as usize;
        (grid_cols, grid_rows)
    }

    fn get_grid_coords(&self, position: Vec2, grid_size: f32) -> (usize, usize) {
        let mut grid_x = (position.x / grid_size).floor() as usize;
        let mut grid_y = (position.y / grid_size).floor() as usize;

        grid_x = grid_x.min(self.grid_cols.saturating_sub(1));
        grid_y = grid_y.min(self.grid_rows.saturating_sub(1));

        (grid_x, grid_y)
    }

    fn get_grid_index(&self, grid_x: usize, grid_y: usize) -> usize {
        grid_y * self.grid_cols + grid_x
    }

    fn get_neighbor_particles(&self, grid_x: usize, grid_y: usize) -> Vec<Particle> {
        let mut neighbors = Vec::new();
        for ny_offset in -1..=1 {
            for nx_offset in -1..=1 {
                let ngx = grid_x as i32 + nx_offset;
                let ngy = grid_y as i32 + ny_offset;

                if ngx >= 0
                    && ngx < self.grid_cols as i32
                    && ngy >= 0
                    && ngy < self.grid_rows as i32
                {
                    let neighbor_grid_index = (ngy as usize) * self.grid_cols + (ngx as usize);
                    if neighbor_grid_index < self.grid.len() {
                        for p_neighbor in &self.grid[neighbor_grid_index].particles {
                            neighbors.push(p_neighbor.clone());
                        }
                    }
                }
            }
        }
        neighbors
    }

    pub fn update(&mut self, delta_time: f32, gravity: Vec2, config: &Config) {
        // predict particle positions
        for grid_box in &mut self.grid {
            for particle in &mut grid_box.particles {
                particle.predict_position();
            }
        }

        self.update_spatial_grid(config); // Grid now updated based on predicted_position
        self.update_density(config);

        (self.grid_cols, self.grid_rows) = self.get_grid_dimensions(config.smoothing_radius);

        // Storing particle state and grid location for force calculation
        let particle_info_for_force_calc: Vec<_> = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(grid_idx, grid_box)| {
                let gx = grid_idx % self.grid_cols;
                let gy = grid_idx / self.grid_cols;
                grid_box.particles.iter().map(move |p| {
                    (
                        p.position,
                        p.predicted_position,
                        p.density,
                        p.near_density,
                        p.id,
                        gx,
                        gy,
                        p.radius,
                        p.velocity,
                    )
                })
            })
            .collect();

        let forces: Vec<Vec2> = particle_info_for_force_calc
            .par_iter()
            .map(
                |(
                    current_pos,
                    predicted_position,
                    current_density,
                    near_density,
                    current_id,
                    gx,
                    gy,
                    p_radius,
                    p_velocity, // Added particle velocity
                )| {
                    let neighbor_particles = self.get_neighbor_particles(*gx, *gy);

                    let temp_current_particle = Particle {
                        position: *current_pos,
                        predicted_position: *predicted_position,
                        density: *current_density,
                        near_density: *near_density,
                        id: *current_id,
                        radius: *p_radius,
                        velocity: *p_velocity, // Use actual particle velocity
                        acceleration: Vec2::ZERO,
                    };

                    let pressure_force = Physics::calculate_pressure_force_on_particle(
                        &temp_current_particle,
                        &neighbor_particles,
                        config.mass,
                        config.smoothing_radius,
                        config,
                    );

                    let viscosity_force = Physics::calculate_viscosity_from_neighbors(
                        &temp_current_particle,
                        &neighbor_particles,
                        config.mass,
                        config.smoothing_radius,
                        config.viscosity_strength,
                    );

                    pressure_force + viscosity_force
                },
            )
            .collect();

        // Apply forces in parallel
        let mut flat_mut_particles: Vec<&mut Particle> = self
            .grid
            .iter_mut()
            .flat_map(|gb| gb.particles.iter_mut())
            .collect();

        flat_mut_particles
            .par_iter_mut()
            .zip(forces.par_iter())
            .for_each(|(particle, force)| {
                if particle.density > 0.0 {
                    // Keeping this check, can be useful
                    particle.acceleration += *force / particle.density;
                }
            });

        // Update particle positions
        for particle in &mut self
            .grid
            .iter_mut()
            .flat_map(|grid_box| &mut grid_box.particles)
        {
            particle.update(delta_time, gravity);
        }
    }

    pub fn update_spatial_grid(&mut self, config: &Config) {
        let all_particles: Vec<Particle> = self
            .grid
            .iter()
            .flat_map(|grid_box| grid_box.particles.clone())
            .collect();

        for grid_box in &mut self.grid {
            grid_box.clear();
        }

        let grid_size = config.smoothing_radius;
        (self.grid_cols, self.grid_rows) = self.get_grid_dimensions(grid_size);

        let expected_grid_len = self.grid_cols * self.grid_rows;
        if self.grid.len() != expected_grid_len {
            self.grid.clear();
            for i in 0..self.grid_rows {
                for j in 0..self.grid_cols {
                    let x = j as f32 * grid_size;
                    let y = i as f32 * grid_size;
                    self.grid.push(GridBox::new(grid_size, Vec2::new(x, y)));
                }
            }
        }

        for particle in all_particles {
            let (grid_x, grid_y) = self.get_grid_coords(particle.predicted_position, grid_size);
            let index = self.get_grid_index(grid_x, grid_y);

            if index < self.grid.len() {
                self.grid[index].add_particle(particle);
            } else if !self.grid.is_empty() {
                let fallback_idx = self
                    .get_grid_index(
                        grid_x.min(self.grid_cols.saturating_sub(1)),
                        grid_y.min(self.grid_rows.saturating_sub(1)),
                    )
                    .min(self.grid.len().saturating_sub(1)); // Ensure fallback_idx is valid

                if fallback_idx < self.grid.len() {
                    self.grid[fallback_idx].add_particle(particle);
                }
            }
        }
    }

    pub fn update_density(&mut self, config: &Config) {
        // Update grid dimensions
        (self.grid_cols, self.grid_rows) = self.get_grid_dimensions(config.smoothing_radius);

        // Storing particle positions and grid coordinates for density calculation
        let particle_infos: Vec<(Vec2, usize, usize)> = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(grid_idx, grid_box)| {
                let gx = grid_idx % self.grid_cols;
                let gy = grid_idx / self.grid_cols;
                grid_box
                    .particles
                    .iter()
                    .map(move |p| (p.predicted_position, gx, gy))
            })
            .collect();

        let densities: Vec<(f32, f32)> = particle_infos
            .par_iter()
            .map(|(predicted_position, gx, gy)| {
                let neighbor_particles = self.get_neighbor_particles(*gx, *gy);
                (
                    Physics::calculate_density_from_neighbors(
                        *predicted_position,
                        &neighbor_particles,
                        config.mass,
                        config.smoothing_radius,
                    ),
                    Physics::calculate_near_density_from_neighbors(
                        *predicted_position,
                        &neighbor_particles,
                        config.mass,
                        config.smoothing_radius,
                    ),
                )
            })
            .collect();

        let mut density_iter = densities.iter();
        for grid_box in &mut self.grid {
            for particle in &mut grid_box.particles {
                if let Some(&density) = density_iter.next() {
                    particle.density = density.0;
                    particle.near_density = density.1;
                } else {
                    eprintln!("Error: Mismatch during density assignment.");
                }
            }
        }
    }

    pub fn handle_interaction(
        &mut self,
        click_point: Vec2,
        interaction_type: InteractionType,
        config: &Config,
    ) {
        let radius = config.interaction_radius;
        let strength = config.interaction_strength;

        for grid_box in &mut self.grid {
            for particle in &mut grid_box.particles {
                let offset = click_point - particle.position;
                let sqr_dist = offset.length_squared();

                if sqr_dist < radius * radius && sqr_dist > 1e-6 {
                    let dist = sqr_dist.sqrt();

                    let dir_to_input_point = offset / dist;

                    let centre_t = 1.0 - dist / radius;

                    let base_force_dir = match interaction_type {
                        InteractionType::Pull => dir_to_input_point,
                        InteractionType::Push => -dir_to_input_point,
                    };

                    let acc_change = (base_force_dir * strength - particle.velocity) * centre_t;
                    particle.acceleration += acc_change;
                }
            }
        }
    }
}
