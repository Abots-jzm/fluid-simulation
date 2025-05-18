use macroquad::prelude::*;

use crate::{config::Config, particle::Particle};

pub enum FluidSpawnMode {
    Random,
    Grid,
}

pub struct Fluid {
    pub particles: Vec<Particle>,
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
            FluidSpawnMode::Grid => {
                let cols = config.particle_columns;
                let rows = config.particle_count / cols;

                let total_width = cols as f32 * config.particle_spacing;
                let left_offset = (screen_width() - total_width) / 2.0;

                for i in 0..rows {
                    for j in 0..cols {
                        let x = left_offset + j as f32 * config.particle_spacing;
                        let y =
                            i as f32 * config.particle_spacing + (config.boundary_padding * 2.0);
                        particles.push(Particle::new(Vec2::new(x, y), config.particle_radius));
                    }
                }
            }
        }
        Self { particles }
    }

    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw();
        }
    }

    pub fn update(&mut self, delta_time: f32, gravity: Vec2) {
        for particle in &mut self.particles {
            particle.update(delta_time, gravity);
        }
    }
}
