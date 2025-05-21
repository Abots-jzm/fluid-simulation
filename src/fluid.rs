use macroquad::prelude::*;
use rayon::prelude::*;

use crate::{
    boundary::Boundary,
    config::{Config, FluidSpawnMode, FluidType, InteractionType},
    grid::GridCell,
    particle::Particle,
    physics::Physics,
    spawner::{ParticleSpawner, spawn_particles_grid},
};

pub struct Fluid {
    pub grid: Vec<GridCell>,
    pub particles: Vec<Particle>,
    grid_cols: usize,
    grid_rows: usize,
    grid_size: f32,
    particle_spawner: Option<ParticleSpawner>,
    world_grid_origin: Vec2,
}

impl Fluid {
    pub fn from_config(config: &Config, boundary: &Boundary) -> Self {
        let initial_spawn_area_width = boundary.width;
        let initial_spawn_area_height = boundary.height;

        let mut particles;
        let mut particle_spawner = None;

        match config.fluid_spawn_mode {
            FluidSpawnMode::Grid => {
                particles = spawn_particles_grid(
                    config,
                    initial_spawn_area_width,
                    initial_spawn_area_height,
                );
                for p in particles.iter_mut() {
                    p.position += boundary.pos;
                }
            }
            FluidSpawnMode::Flow => {
                particles = Vec::new();
                particle_spawner = Some(ParticleSpawner::new(config, boundary));
            }
        }

        let grid_size = config.smoothing_radius;
        let mut world_grid_origin = boundary.pos;

        let base_grid_cols = (boundary.width / grid_size).ceil().max(1.0) as usize;
        let base_grid_rows = (boundary.height / grid_size).ceil().max(1.0) as usize;

        let mut current_grid_cols = base_grid_cols;
        let mut current_grid_rows = base_grid_rows;

        if config.fluid_type == FluidType::Liquid {
            let extended_grid_cols = base_grid_cols + 2;
            let extended_grid_rows = base_grid_rows + 2;

            world_grid_origin = boundary.pos - Vec2::new(grid_size, grid_size);
            current_grid_cols = extended_grid_cols;
            current_grid_rows = extended_grid_rows;

            let ghost_layer_origin_x = world_grid_origin.x;
            let ghost_layer_origin_y = world_grid_origin.y;

            // Ghost particles
            let mut ghost_particles_vec: Vec<Particle> = Vec::new();
            let extended_grid_rows_f = extended_grid_rows as f32;

            for r_idx in 0..extended_grid_rows {
                for c_idx in 0..extended_grid_cols {
                    if !(r_idx == 0
                        || r_idx == extended_grid_rows - 1
                        || c_idx == 0
                        || c_idx == extended_grid_cols - 1)
                    {
                        continue;
                    }

                    let is_on_top_boundary = r_idx == 0;
                    if is_on_top_boundary {
                        continue;
                    }

                    let is_on_left_boundary = c_idx == 0;
                    let is_on_right_boundary = c_idx == extended_grid_cols - 1;
                    let is_on_bottom_boundary = r_idx == extended_grid_rows - 1;

                    if is_on_left_boundary || is_on_right_boundary {
                        let norm_y_in_col = (r_idx as f32) / (extended_grid_rows_f - 1.0);
                        if norm_y_in_col < config.ghost_wall_start_percent {
                            continue;
                        }
                    }

                    let spacing_factor = {
                        if is_on_bottom_boundary {
                            1.0
                        } else if is_on_left_boundary || is_on_right_boundary {
                            let norm_y_in_col = (r_idx as f32) / (extended_grid_rows_f - 1.0);
                            let range_norm_y = (norm_y_in_col - config.ghost_wall_start_percent)
                                / (1.0 - config.ghost_wall_start_percent);
                            let interp_t = range_norm_y.max(0.0).min(1.0);
                            config.start_ghost_spacing_multiplier * (1.0 - interp_t)
                                + 1.0 * interp_t
                        } else {
                            1.0
                        }
                    };

                    let current_spacing =
                        config.particle_radius * config.target_ghost_spacing * spacing_factor;

                    if current_spacing <= 1e-3 {
                        continue;
                    }

                    let count_x = (grid_size / current_spacing).floor().max(1.0) as u32;
                    let count_y = (grid_size / current_spacing).floor().max(1.0) as u32;

                    let margin_x = (grid_size - (count_x as f32 * current_spacing)) / 2.0;
                    let margin_y = (grid_size - (count_y as f32 * current_spacing)) / 2.0;

                    let cell_origin_x = ghost_layer_origin_x + c_idx as f32 * grid_size;
                    let cell_origin_y = ghost_layer_origin_y + r_idx as f32 * grid_size;

                    for i in 0..count_y {
                        for j in 0..count_x {
                            let px = cell_origin_x
                                + margin_x
                                + (j as f32 * current_spacing)
                                + current_spacing * 0.5;
                            let py = cell_origin_y
                                + margin_y
                                + (i as f32 * current_spacing)
                                + current_spacing * 0.5;
                            ghost_particles_vec.push(Particle::new(
                                Vec2::new(px, py),
                                config.particle_radius,
                                true,
                            ));
                        }
                    }
                }
            }
            particles.extend(ghost_particles_vec);
        }

        let mut grid: Vec<GridCell> = Vec::with_capacity(current_grid_cols * current_grid_rows);
        for i in 0..current_grid_rows {
            for j in 0..current_grid_cols {
                grid.push(GridCell::new(
                    grid_size,
                    Vec2::new(
                        world_grid_origin.x + j as f32 * grid_size,
                        world_grid_origin.y + i as f32 * grid_size,
                    ),
                ));
            }
        }

        for (index, particle) in particles.iter().enumerate() {
            let (grid_x, grid_y) = Self::get_grid_coords_internal(
                particle.position,
                world_grid_origin,
                grid_size,
                current_grid_cols,
                current_grid_rows,
            );
            let grid_index = Self::get_grid_index_internal(grid_x, grid_y, current_grid_cols);

            if grid_index < grid.len() {
                grid[grid_index].add_particle(index);
            } else {
                eprintln!(
                    "Particle out of initial grid bounds: index {}, pos {:?}, grid_xy ({}, {}), grid_idx {}, grid_len {}",
                    index,
                    particle.position,
                    grid_x,
                    grid_y,
                    grid_index,
                    grid.len()
                );
            }
        }

        Self {
            grid,
            particles,
            grid_cols: current_grid_cols,
            grid_rows: current_grid_rows,
            grid_size,
            particle_spawner,
            world_grid_origin,
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

    fn get_grid_coords_internal(
        position: Vec2,
        world_grid_origin: Vec2,
        grid_size: f32,
        grid_cols: usize,
        grid_rows: usize,
    ) -> (usize, usize) {
        let mut grid_x = ((position.x - world_grid_origin.x) / grid_size).floor() as i32;
        let mut grid_y = ((position.y - world_grid_origin.y) / grid_size).floor() as i32;

        // Clamp to grid boundaries
        grid_x = grid_x.max(0).min(grid_cols.saturating_sub(1) as i32);
        grid_y = grid_y.max(0).min(grid_rows.saturating_sub(1) as i32);

        (grid_x as usize, grid_y as usize)
    }

    fn get_grid_index_internal(grid_x: usize, grid_y: usize, grid_cols: usize) -> usize {
        grid_y * grid_cols + grid_x
    }

    fn get_grid_coords(&self, position: Vec2) -> (usize, usize) {
        Self::get_grid_coords_internal(
            position,
            self.world_grid_origin,
            self.grid_size,
            self.grid_cols,
            self.grid_rows,
        )
    }

    fn get_grid_index(&self, grid_x: usize, grid_y: usize) -> usize {
        Self::get_grid_index_internal(grid_x, grid_y, self.grid_cols)
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
        if let Some(spawner) = &mut self.particle_spawner {
            spawner.update_flow_spawn(&mut self.particles, config, delta_time);
        }

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
            } else {
                let clamped_gx = grid_x.min(self.grid_cols.saturating_sub(1));
                let clamped_gy = grid_y.min(self.grid_rows.saturating_sub(1));
                let clamped_idx = self.get_grid_index(clamped_gx, clamped_gy);
                if clamped_idx < self.grid.len() {
                    self.grid[clamped_idx].add_particle(index);
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
                let neighbor_particle_indices = self.get_neighbor_particle_indices(grid_x, grid_y);
                let density = Physics::calculate_density_from_neighbors(
                    particle.predicted_position,
                    &neighbor_particle_indices,
                    &self.particles,
                    config.mass,
                    config.smoothing_radius,
                );
                let near_density = Physics::calculate_near_density_from_neighbors(
                    particle.predicted_position,
                    &neighbor_particle_indices,
                    &self.particles,
                    config.mass,
                    config.smoothing_radius,
                );
                (density, near_density)
            })
            .collect();

        self.particles
            .par_iter_mut()
            .zip(densities.par_iter())
            .for_each(|(particle, &(density, near_density))| {
                particle.density = density;
                particle.near_density = near_density;
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

                let centre_t = if config.fluid_type == FluidType::Liquid {
                    let normalized_dist = dist / radius;
                    const FALLOFF_EXPONENT: f32 = 6.0;
                    let centre_t = 1.0 - normalized_dist.powf(FALLOFF_EXPONENT);
                    centre_t.max(0.0)
                } else {
                    1.0 - dist / radius
                };

                let dir_to_input_point = offset / dist;

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
