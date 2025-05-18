use std::f32::consts::PI;

use macroquad::prelude::*;
use rayon::prelude::*;

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

        let forces: Vec<Vec2> = (0..self.particles.len())
            .into_par_iter()
            .map(|i| self.calculate_pressure_force(i, config.mass, config.smoothing_radius, config))
            .collect();

        self.particles
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, particle)| {
                if particle.density != 0.0 {
                    particle.acceleration += forces[i] / particle.density; // Changed from = to += to accumulate gravity
                } else {
                    // Avoid division by zero, particle might be isolated or an error occurred
                    // particle.acceleration is already set by particle.update (gravity)
                    // Or, if pressure force should dominate, set to Vec2::ZERO
                    // For now, let's assume gravity is already applied and we don't overwrite if density is zero.
                }
            });
    }

    pub fn update_density(&mut self, config: &Config) {
        let densities: Vec<f32> = self
            .particles
            .par_iter()
            .map(|particle| {
                self.calculate_density(particle.position, config.mass, config.smoothing_radius)
            })
            .collect();

        self.particles
            .iter_mut()
            .zip(densities)
            .for_each(|(particle, density)| {
                particle.density = density;
            });
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

        let r_scaled = radius / DISTANCE_ZOOM;
        let d_scaled = distance / DISTANCE_ZOOM;

        let volume = PI * r_scaled.powi(4) / 6.0;

        (r_scaled - d_scaled).powi(2) / volume
    }

    fn smoothing_kernel_derivative(&self, radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        // let d_scaled = distance / DISTANCE_ZOOM; // d_scaled is not used with current formula, but good to keep if formula changes

        let denominator = PI * r_scaled.powi(4);

        let scale = 12.0 / denominator;

        (distance / DISTANCE_ZOOM - r_scaled) * scale // Ensure distance is also scaled consistently
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
        let current_particle_pos = self.particles[index].position;
        let current_particle_density = self.particles[index].density;

        for (i, other_particle) in self.particles.iter().enumerate() {
            if i == index {
                continue;
            }

            let distance = current_particle_pos.distance(other_particle.position);
            if distance == 0.0 || distance > radius {
                // also check distance > radius here
                continue;
            }

            let mut direction =
                (other_particle.position - current_particle_pos).normalize_or_zero();

            if direction == Vec2::ZERO {
                // This case (distance != 0 but normalize_or_zero gives ZERO) should not happen with standard Vec2.
                // If it could, or if distance was zero and we didn't continue, then assign random.
                let angle = rand::gen_range(0.0, 2.0 * PI);
                direction = Vec2::new(angle.cos(), angle.sin());
            }

            let slope = self.smoothing_kernel_derivative(radius, distance);
            let shared_pressure = self.calculate_shared_pressure(
                other_particle.density,
                current_particle_density,
                config,
            );

            if other_particle.density == 0.0 {
                continue; // Avoid division by zero
            }
            pressure_force += shared_pressure * direction * slope * mass / other_particle.density;
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
            if distance > radius {
                // Optimization: skip particles outside radius
                continue;
            }
            let influcence = self.smoothing_kernel(radius, distance);
            property += influcence * mass / particle.density // * property to calculate
        }

        property
    }

    // #[allow(dead_code)]
    // pub fn calculate_property_gradient(&self, point: Vec2, mass: f32, radius: f32) -> Vec2 {
    //     // let mut gradient = Vec2::ZERO;

    //     // for particle in &self.particles {
    //     //     let distance = point.distance(particle.position);
    //     //     let direction = (particle.position - point).normalize();
    //     //     let slope = self.smoothing_kernel_derivative(radius, distance);
    //     //     let density = self.calculate_density(particle.position, mass, radius);
    //     //     gradient += direction * slope * mass / density; // * property to calculate
    //     // }

    //     // gradient

    //     self.particles
    //         .par_iter()
    //         .map(|particle| {
    //             let distance = point.distance(particle.position);
    //             if distance == 0.0 || distance > radius {
    //                 // Avoid division by zero if point is particle.position, and skip if too far
    //                 return Vec2::ZERO;
    //             }
    //             let mut direction = (particle.position - point).normalize_or_zero(); // normalize_or_zero handles zero vector case

    //             // If direction is still zero (e.g. point == particle.position after all checks, though distance check should prevent)
    //             // a random direction might be needed, or skip. For now, normalize_or_zero handles it.
    //             if direction == Vec2::ZERO {
    //                 // This case should ideally be handled by the distance check or specific logic
    //                 // For SPH, a particle doesn't exert force/gradient on itself in this manner.
    //                 // If point can be exactly a particle's position, this needs careful thought.
    //                 // Assuming distance > 0 from the check above.
    //             }

    //             let slope = self.smoothing_kernel_derivative(radius, distance);

    //             // PERFORMANCE NOTE: This calculate_density call is inside a loop.
    //             // If particle.density (the stored density) is appropriate for this property calculation,
    //             // using it directly would be a significant optimization, changing this part from O(N) to O(1).
    //             // The current implementation makes calculate_property_gradient O(N^2) per call.
    //             let density = self.calculate_density(particle.position, mass, radius);

    //             if density == 0.0 {
    //                 return Vec2::ZERO; // Avoid division by zero
    //             }
    //             direction * slope * mass / density // * property value of the particle (if property varies per particle)
    //         })
    //         .reduce(|| Vec2::ZERO, |a, b| a + b)
    // }
}
