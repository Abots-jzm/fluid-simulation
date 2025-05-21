use macroquad::prelude::*;
use rayon::prelude::*;

use crate::{
    boundary::Boundary,
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
    map_width: f32,
    map_height: f32,
    screen_width: f32,
    screen_height: f32,
}

impl Fluid {
    pub fn from_config(config: &Config, boundary: &Boundary) -> Self {
        let map_width = boundary.width + config.smoothing_radius * 2.;
        let map_height = boundary.height + config.smoothing_radius * 2.;

        let mut particles = Vec::with_capacity(config.particle_count as usize);
        let target_aspect_ratio = map_width / map_height;

        let rows: u32;
        let cols: u32;

        let particle_count_f = config.particle_count as f32;

        let rows_f = (particle_count_f / target_aspect_ratio).sqrt();
        let cols_f = (particle_count_f * target_aspect_ratio).sqrt();
        rows = rows_f.round() as u32;
        cols = cols_f.round() as u32;

        let spacing = config.particle_radius * 2.5;

        let total_width = cols as f32 * spacing;
        let left_offset = (map_width - total_width) / 2.0;
        let total_height = rows as f32 * spacing;
        let top_offset = (map_height - total_height) / 2.0;

        for i in 0..rows {
            for j in 0..cols {
                let x = left_offset + j as f32 * spacing;
                let y = i as f32 * spacing + top_offset;
                particles.push(Particle::new(
                    Vec2::new(x, y),
                    config.particle_radius,
                    false,
                ));
            }
        }

        let grid_size = config.smoothing_radius;
        let mut grid_cols = (boundary.width / grid_size).ceil() as usize;
        let mut grid_rows = (boundary.height / grid_size).ceil() as usize;

        let mut grid: Vec<GridCell> = Vec::with_capacity(grid_cols * grid_rows);
        for i in 0..grid_rows {
            for j in 0..grid_cols {
                let x = j as f32 * grid_size + boundary.pos.x;
                let y = i as f32 * grid_size + boundary.pos.y;
                grid.push(GridCell::new(grid_size, Vec2::new(x, y)));
            }
        }

        if config.fluid_spawn_mode == FluidSpawnMode::Liquid {
            // Create ghost grid cells
            let mut ghost_grid = Vec::new();
            let extended_grid_cols = grid_cols + 2;
            let extended_grid_rows = grid_rows + 2;
            let ghost_layer_origin_x = boundary.pos.x - grid_size;
            let ghost_layer_origin_y = boundary.pos.y - grid_size;

            for r in 0..extended_grid_rows {
                for c in 0..extended_grid_cols {
                    // Only add cells that are on the outer-most layer
                    if r == 0
                        || r == extended_grid_rows - 1
                        || c == 0
                        || c == extended_grid_cols - 1
                    {
                        let x = c as f32 * grid_size + ghost_layer_origin_x;
                        let y = r as f32 * grid_size + ghost_layer_origin_y;
                        ghost_grid.push(GridCell::new(grid_size, Vec2::new(x, y)));
                    }
                }
            }

            // Ghost particles
            let mut ghost_particles: Vec<Particle> = Vec::new();

            let left_boundary_x = boundary.pos.x - grid_size;
            let right_boundary_x = boundary.pos.x + (grid_cols as f32 * grid_size);
            let top_boundary_y = boundary.pos.y - grid_size;
            let bottom_boundary_y = boundary.pos.y + (grid_rows as f32 * grid_size);

            let extended_grid_rows_f = extended_grid_rows as f32;

            for ghost_cell in &ghost_grid {
                let cell_x = ghost_cell.position.x;
                let cell_y = ghost_cell.position.y;

                let is_on_top_boundary = (cell_y - top_boundary_y).abs() < f32::EPSILON;
                if is_on_top_boundary {
                    continue;
                }

                let is_on_left_boundary = (cell_x - left_boundary_x).abs() < f32::EPSILON;
                let is_on_right_boundary = (cell_x - right_boundary_x).abs() < f32::EPSILON;

                // skip particles before target percent
                if is_on_left_boundary || is_on_right_boundary {
                    // Normalized so 0 at top and 1 at bottom
                    let norm_y_in_col =
                        ((cell_y - top_boundary_y) / grid_size) / (extended_grid_rows_f - 1.0);
                    if norm_y_in_col < config.ghost_wall_start_percent {
                        continue;
                    }
                }

                let spacing_factor = {
                    let is_on_bottom_boundary = (cell_y - bottom_boundary_y).abs() < f32::EPSILON;
                    if is_on_bottom_boundary {
                        1.0
                    } else if is_on_left_boundary || is_on_right_boundary {
                        let norm_y_in_col =
                            ((cell_y - top_boundary_y) / grid_size) / (extended_grid_rows_f - 1.0);
                        let interp_t = (norm_y_in_col - 0.5).max(0.0) * 2.0;
                        config.start_ghost_spacing_multiplier * (1.0 - interp_t) + interp_t
                    } else {
                        // Should not be reachable
                        1.0
                    }
                };

                let current_spacing =
                    config.particle_radius * config.target_ghost_spacing * spacing_factor;

                let count_x = (grid_size / current_spacing).floor() as u32;
                let count_y = (grid_size / current_spacing).floor() as u32;

                if count_x == 0 || count_y == 0 {
                    continue;
                }

                let margin_x = (grid_size - (count_x as f32 * current_spacing)) / 2.0;
                let margin_y = (grid_size - (count_y as f32 * current_spacing)) / 2.0;

                for i in 0..count_y {
                    for j in 0..count_x {
                        let px =
                            ghost_cell.position.x + margin_x + (j as f32 + 0.5) * current_spacing;
                        let py =
                            ghost_cell.position.y + margin_y + (i as f32 + 0.5) * current_spacing;
                        ghost_particles.push(Particle::new(
                            Vec2::new(px, py),
                            config.particle_radius,
                            true,
                        ));
                    }
                }
            }

            grid.extend(ghost_grid);
            grid_cols += 2;
            grid_rows += 2;
            particles.extend(ghost_particles);

            grid.sort_by(|a, b| {
                if a.position.y == b.position.y {
                    a.position.x.partial_cmp(&b.position.x).unwrap()
                } else {
                    a.position.y.partial_cmp(&b.position.y).unwrap()
                }
            });
        }

        for (index, particle) in particles.iter().enumerate() {
            let offset_x = (screen_width() - map_width) / 2.;
            let offset_y = (screen_height() - map_height) / 2.;
            let grid_x = ((particle.position.x - offset_x) / grid_size).floor() as usize;
            let grid_y = ((particle.position.y - offset_y) / grid_size).floor() as usize;
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
            map_width,
            map_height,
            screen_width: screen_width(),
            screen_height: screen_height(),
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
        let offset_x = (self.screen_width - self.map_width) / 2.;
        let offset_y = (self.screen_height - self.map_height) / 2.;
        let mut grid_x = ((position.x - offset_x) / self.grid_size).floor() as usize;
        let mut grid_y = ((position.y - offset_y) / self.grid_size).floor() as usize;

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

                let centre_t = if config.fluid_spawn_mode == FluidSpawnMode::Liquid {
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
