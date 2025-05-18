use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{config::Config, particle::Particle, simulation::DISTANCE_ZOOM};

#[allow(dead_code)]
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

    pub fn update(&mut self, delta_time: f32, gravity: Vec2, config: &Config) {
        self.update_density(config);

        for particle in &mut self.particles {
            particle.update(delta_time, gravity);
        }

        for i in 0..self.particles.len() {
            let pressure_force =
                self.calculate_pressure_force(i, config.mass, config.smoothing_radius, config);
            let particle = &mut self.particles[i];
            particle.acceleration = pressure_force / particle.density;
        }
    }

    pub fn update_density(&mut self, config: &Config) {
        let mut densities = Vec::with_capacity(self.particles.len());

        for particle in &self.particles {
            let density =
                self.calculate_density(particle.position, config.mass, config.smoothing_radius);
            densities.push(density);
        }

        for (i, particle) in self.particles.iter_mut().enumerate() {
            particle.density = densities[i];
        }
    }

    pub fn calculate_density(&self, point: Vec2, mass: f32, smoothing_radius: f32) -> f32 {
        let mut density = 0.0;

        for particle in &self.particles {
            let distance = point.distance(particle.position);
            density += mass * self.smoothing_kernel(smoothing_radius, distance);
        }
        density
    }

    fn smoothing_kernel(&self, radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let distance = distance / DISTANCE_ZOOM;
        let radius = radius / DISTANCE_ZOOM;

        let volume = PI * radius.powi(4) / 6.0;

        (radius - distance) * (radius - distance) / volume
    }

    fn smoothing_kernel_derivative(&self, radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let distance = distance / DISTANCE_ZOOM;
        let radius = radius / DISTANCE_ZOOM;

        let scale = 12.0 / (PI * radius.powi(4));

        (distance - radius) * scale
    }

    pub fn density_to_pressure(
        &self,
        density: f32,
        target_density: f32,
        pressure_multiplier: f32,
    ) -> f32 {
        let density_diff = density - target_density;
        pressure_multiplier * density_diff
    }

    pub fn calculate_pressure_force(
        &self,
        index: usize,
        mass: f32,
        radius: f32,
        config: &Config,
    ) -> Vec2 {
        let mut pressure_force = Vec2::ZERO;

        for (i, particle) in self.particles.iter().enumerate() {
            if i == index {
                continue;
            }

            let distance = (self.particles[index].position).distance(particle.position);
            let mut direction = (particle.position - self.particles[index].position).normalize();

            if direction == Vec2::ZERO {
                let angle = rand::gen_range(0.0, 2.0 * PI);
                direction = Vec2::new(angle.cos(), angle.sin());
            }

            let slope = self.smoothing_kernel_derivative(radius, distance);
            let shared_pressure = self.calculate_shared_pressure(
                particle.density,
                self.particles[index].density,
                config,
            );
            pressure_force += shared_pressure * direction * slope * mass / particle.density;
        }

        pressure_force
    }

    pub fn calculate_shared_pressure(
        &self,
        density_a: f32,
        density_b: f32,
        config: &Config,
    ) -> f32 {
        let pressure_a =
            self.density_to_pressure(density_a, config.target_density, config.pressure_multiplier);
        let pressure_b =
            self.density_to_pressure(density_b, config.target_density, config.pressure_multiplier);
        let shared_pressure = (pressure_a + pressure_b) / 2.0;
        shared_pressure
    }

    #[allow(dead_code)]
    pub fn calculate_property(&self, point: Vec2, mass: f32, radius: f32) -> f32 {
        let mut property = 0.0;

        for particle in &self.particles {
            let distance = point.distance(particle.position);
            let influcence = self.smoothing_kernel(radius, distance);
            property += influcence * mass / particle.density // * property to calculate
        }

        property
    }

    #[allow(dead_code)]
    pub fn calculate_property_gradient(&self, point: Vec2, mass: f32, radius: f32) -> Vec2 {
        let mut gradient = Vec2::ZERO;

        for particle in &self.particles {
            let distance = point.distance(particle.position);
            let direction = (particle.position - point).normalize();
            let slope = self.smoothing_kernel_derivative(radius, distance);
            let density = self.calculate_density(particle.position, mass, radius);
            gradient += direction * slope * mass / density; // * property to calculate
        }

        gradient
    }
}
