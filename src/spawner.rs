use macroquad::prelude::*;
use macroquad::rand as macroquad_rand;

use crate::{
    boundary::Boundary,
    config::{Config, FluidSpawnMode},
    particle::Particle,
};

pub struct ParticleSpawner {
    time_to_next_spawn: f32,
    spawn_x_min: f32,
    spawn_x_max: f32,
    spawn_y: f32,
    flow_rate_interval: f32,
}

impl ParticleSpawner {
    pub fn new(config: &Config, boundary: &Boundary) -> Self {
        let padding = config.particle_radius * 1.1;
        let spawn_y = boundary.pos.y + padding;

        let max_possible_spawn_width = boundary.width - 2.0 * padding;
        let effective_spawn_width = config
            .flow_spawn_width
            .min(max_possible_spawn_width)
            .max(0.0);

        let center_x = boundary.pos.x + boundary.width / 2.0;
        let spawn_x_start = center_x - effective_spawn_width / 2.0;
        let spawn_x_end = center_x + effective_spawn_width / 2.0;

        let min_boundary_x = boundary.pos.x + padding;
        let max_boundary_x = boundary.pos.x + boundary.width - padding;

        Self {
            time_to_next_spawn: 0.0,
            spawn_x_min: spawn_x_start.max(min_boundary_x),
            spawn_x_max: spawn_x_end.min(max_boundary_x),
            spawn_y,
            flow_rate_interval: if config.flow_spawn_rate > 0.0 {
                1.0 / config.flow_spawn_rate
            } else {
                f32::MAX
            },
        }
    }

    pub fn update_flow_spawn(
        &mut self,
        particles: &mut Vec<Particle>,
        config: &Config,
        delta_time: f32,
    ) {
        if config.fluid_spawn_mode != FluidSpawnMode::Flow
            || self.flow_rate_interval == f32::MAX
            || self.spawn_x_min >= self.spawn_x_max
            || particles.len() >= config.particle_count as usize
        {
            return;
        }

        self.time_to_next_spawn -= delta_time;

        while self.time_to_next_spawn <= 0.0 {
            if particles.len() >= config.particle_count as usize {
                break;
            }
            let spawn_x = macroquad_rand::gen_range(self.spawn_x_min, self.spawn_x_max);
            let spawn_position = Vec2::new(spawn_x, self.spawn_y);

            let new_particle = Particle::new(spawn_position, config.particle_radius, false);
            particles.push(new_particle);

            self.time_to_next_spawn += self.flow_rate_interval;
        }
    }
}

pub fn spawn_particles_grid(config: &Config, map_width: f32, map_height: f32) -> Vec<Particle> {
    if config.particle_count == 0 {
        return Vec::new();
    }

    let mut particles = Vec::with_capacity(config.particle_count as usize);
    let particle_count_f = config.particle_count as f32;
    let target_aspect_ratio = if map_height > 0.0 {
        map_width / map_height
    } else {
        1.0
    };

    let rows_f = (particle_count_f / target_aspect_ratio).sqrt();
    let cols_f = particle_count_f / rows_f;

    let mut rows = rows_f.round() as u32;
    let mut cols = cols_f.round() as u32;

    if rows == 0 && cols == 0 && config.particle_count > 0 {
        rows = 1;
        cols = 1;
    } else if rows == 0 && config.particle_count > 0 {
        rows = 1;
        cols = (particle_count_f / rows as f32).round().max(1.0) as u32;
    } else if cols == 0 && config.particle_count > 0 {
        cols = 1;
        rows = (particle_count_f / cols as f32).round().max(1.0) as u32;
    }

    let spacing = config.particle_radius * 2.5;
    let conceptual_total_width = cols as f32 * spacing;
    let left_offset = (map_width - conceptual_total_width) / 2.0;
    let conceptual_total_height = rows as f32 * spacing;
    let top_offset = (map_height - conceptual_total_height) / 2.0;

    for i in 0..rows {
        for j in 0..cols {
            if particles.len() >= config.particle_count as usize {
                break;
            }
            let x = left_offset + j as f32 * spacing;
            let y = top_offset + i as f32 * spacing;

            particles.push(Particle::new(
                Vec2::new(x, y),
                config.particle_radius,
                false,
            ));
        }
        if particles.len() >= config.particle_count as usize {
            break;
        }
    }

    particles
}
