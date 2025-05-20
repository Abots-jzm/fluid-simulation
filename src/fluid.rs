use macroquad::prelude::*;
use rayon::prelude::*;

use crate::{
    config::{Config, FluidSpawnMode, InteractionType},
    grid::GridCell,
    particle::Particle,
    physics::Physics,
};

pub struct Fluid {
    pub grid: Vec<GridCell>,
    pub particles: Vec<Particle>,
    grid_cols: usize,
    grid_rows: usize,
    grid_size: f32,
}

impl Fluid {
    pub fn from_config(config: &Config) -> Self {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let mut particles = Vec::with_capacity(config.particle_count as usize);

        match config.fluid_spawn_mode {
            FluidSpawnMode::Random => {
                for _ in 0..config.particle_count {
                    let x = rand::gen_range(
                        config.boundary_padding + config.particle_radius,
                        screen_width - config.boundary_padding - config.particle_radius,
                    );
                    let y = rand::gen_range(
                        config.boundary_padding + config.particle_radius,
                        screen_height - config.boundary_padding - config.particle_radius,
                    );
                    particles.push(Particle::new(Vec2::new(x, y), config.particle_radius));
                }
            }
            _ => {
                let target_aspect_ratio = screen_width / screen_height;

                let rows: u32;
                let cols: u32;

                let particle_count_f = config.particle_count as f32;

                let rows_f = (particle_count_f / target_aspect_ratio).sqrt();
                let cols_f = (particle_count_f * target_aspect_ratio).sqrt();
                rows = rows_f.round() as u32;
                cols = cols_f.round() as u32;

                let spacing = config.particle_radius * 2.5;

                let total_width = cols as f32 * spacing;
                let left_offset = (screen_width - total_width) / 2.0;
                let total_height = rows as f32 * spacing;
                let top_offset = (screen_height - total_height) / 2.0;

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
        let grid_cols = (screen_width / grid_size).ceil() as usize;
        let grid_rows = (screen_height / grid_size).ceil() as usize;

        let mut grid = Vec::with_capacity(grid_cols * grid_rows);
        for i in 0..grid_rows {
            for j in 0..grid_cols {
                let x = j as f32 * grid_size;
                let y = i as f32 * grid_size;
                grid.push(GridCell::new(grid_size, Vec2::new(x, y)));
            }
        }

        for (index, particle) in particles.iter().enumerate() {
            let grid_x = (particle.position.x / grid_size).floor() as usize;
            let grid_y = (particle.position.y / grid_size).floor() as usize;
            let grid_index = grid_y * grid_cols + grid_x;

            if grid_index < grid.len() {
                grid[grid_index].add_particle(index);
            }
        }

        Self {
            grid,
            particles,
            grid_cols,
            grid_rows,
            grid_size,
        }
    }

    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw(750.);
        }

        for grid_box in &self.grid {
            grid_box.draw();
        }
    }

    fn get_grid_coords(&self, position: Vec2) -> (usize, usize) {
        let mut grid_x = (position.x / self.grid_size).floor() as usize;
        let mut grid_y = (position.y / self.grid_size).floor() as usize;

        grid_x = grid_x.min(self.grid_cols.saturating_sub(1));
        grid_y = grid_y.min(self.grid_rows.saturating_sub(1));

        (grid_x, grid_y)
    }

    fn get_grid_index(&self, grid_x: usize, grid_y: usize) -> usize {
        grid_y * self.grid_cols + grid_x
    }

    fn get_neighbor_particle_indices(&self, grid_x: usize, grid_y: usize) -> Vec<usize> {
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
                    let neighbor_grid_index = self.get_grid_index(ngx as usize, ngy as usize);
                    if neighbor_grid_index < self.grid.len() {
                        for &particle_index in &self.grid[neighbor_grid_index].particles {
                            neighbors.push(particle_index);
                        }
                    }
                }
            }
        }

        neighbors
    }

    pub fn update(&mut self, delta_time: f32, gravity: Vec2, config: &Config) {
        self.particles
            .par_iter_mut()
            .for_each(|particle| particle.predict_position());

        self.update_spatial_grid();
        self.update_density(config);

        let forces: Vec<Vec2> = self
            .particles
            .par_iter()
            .enumerate()
            .map(|(index, particle)| {
                let (grid_x, grid_y) = self.get_grid_coords(particle.predicted_position);
                let neighbor_particle_indices = self.get_neighbor_particle_indices(grid_x, grid_y);

                let pressure_force = Physics::calculate_pressure_force_on_particle(
                    index,
                    &neighbor_particle_indices,
                    &self.particles,
                    config.mass,
                    config.smoothing_radius,
                    config,
                );

                let viscosity_force = Physics::calculate_viscosity_from_neighbors(
                    index,
                    &neighbor_particle_indices,
                    &self.particles,
                    config.mass,
                    config.smoothing_radius,
                    config.viscosity_strength,
                );

                pressure_force + viscosity_force
            })
            .collect();

        self.particles
            .par_iter_mut()
            .zip(forces.par_iter())
            .for_each(|(particle, &force)| {
                if particle.density > 0.0 {
                    particle.acceleration += force / particle.density;
                }

                particle.update(delta_time, gravity);
            });
    }

    pub fn update_spatial_grid(&mut self) {
        for grid_box in &mut self.grid {
            grid_box.clear();
        }

        for (index, particle) in self.particles.iter().enumerate() {
            let (grid_x, grid_y) = self.get_grid_coords(particle.predicted_position);
            let grid_index = self.get_grid_index(grid_x, grid_y);

            if grid_index < self.grid.len() {
                self.grid[grid_index].add_particle(index);
            } else if !self.grid.is_empty() {
                let fallback_idx = self
                    .get_grid_index(
                        grid_x.min(self.grid_cols.saturating_sub(1)),
                        grid_y.min(self.grid_rows.saturating_sub(1)),
                    )
                    .min(self.grid.len().saturating_sub(1));

                if fallback_idx < self.grid.len() {
                    self.grid[fallback_idx].add_particle(index);
                }
            }
        }
    }

    pub fn update_density(&mut self, config: &Config) {
        let densities: Vec<(f32, f32)> = self
            .particles
            .par_iter()
            .map(|particle| {
                let (grid_x, grid_y) = self.get_grid_coords(particle.predicted_position);
                let neighbor_indices = self.get_neighbor_particle_indices(grid_x, grid_y);
                (
                    Physics::calculate_density_from_neighbors(
                        particle.predicted_position,
                        &neighbor_indices,
                        &self.particles,
                        config.mass,
                        config.smoothing_radius,
                    ),
                    Physics::calculate_near_density_from_neighbors(
                        particle.predicted_position,
                        &neighbor_indices,
                        &self.particles,
                        config.mass,
                        config.smoothing_radius,
                    ),
                )
            })
            .collect();

        self.particles
            .par_iter_mut()
            .zip(densities.par_iter())
            .for_each(|(particle, &density)| {
                particle.density = density.0;
                particle.near_density = density.1;
            });
    }

    pub fn handle_interaction(
        &mut self,
        click_point: Vec2,
        interaction_type: InteractionType,
        config: &Config,
    ) {
        let radius = config.interaction_radius;
        let strength = config.interaction_strength;

        self.particles.par_iter_mut().for_each(|particle| {
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
        });
    }
}
