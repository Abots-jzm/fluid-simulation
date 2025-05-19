use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::{config::Config, particle::Particle, simulation::DISTANCE_ZOOM};

pub struct Physics;

impl Physics {
    pub fn viscosity_kernel(radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        let d_scaled = distance / DISTANCE_ZOOM;

        let volume = PI * r_scaled.powi(8) / 4.0;
        let value = r_scaled.powi(2) - d_scaled.powi(2);

        value * value * value / volume
    }

    pub fn smoothing_kernel(radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        let d_scaled = distance / DISTANCE_ZOOM;

        let volume = PI * r_scaled.powi(4) / 6.0;

        (r_scaled - d_scaled).powi(2) / volume
    }

    pub fn smoothing_kernel_derivative(radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        let denominator = PI * r_scaled.powi(4);

        let scale = 12.0 / denominator;

        (distance / DISTANCE_ZOOM - r_scaled) * scale
    }

    pub fn density_to_pressure(density: f32, target_density: f32, pressure_multiplier: f32) -> f32 {
        let density_diff = density - target_density;
        pressure_multiplier * density_diff
    }

    pub fn calculate_shared_pressure(density_a: f32, density_b: f32, config: &Config) -> f32 {
        let pressure_a =
            Self::density_to_pressure(density_a, config.target_density, config.pressure_multiplier);
        let pressure_b =
            Self::density_to_pressure(density_b, config.target_density, config.pressure_multiplier);
        (pressure_a + pressure_b) / 2.0
    }

    pub fn calculate_density_from_neighbors(
        point: Vec2,
        neighbor_particles: &[Particle],
        mass: f32,
        smoothing_radius: f32,
    ) -> f32 {
        neighbor_particles.iter().fold(0.0, |density, particle| {
            let distance = point.distance(particle.predicted_position);
            density + mass * Self::smoothing_kernel(smoothing_radius, distance)
        })
    }

    pub fn calculate_viscosity_from_neighbors(
        particle: &Particle,
        neighbor_particles: &[Particle],
        mass: f32,
        smoothing_radius: f32,
        viscosity_strength: f32,
    ) -> Vec2 {
        let mut viscosity_force = Vec2::ZERO;
        let current_particle_pos = particle.predicted_position;

        for other_particle in neighbor_particles {
            let distance = current_particle_pos.distance(other_particle.predicted_position);
            let influence = Self::viscosity_kernel(smoothing_radius, distance) * mass;
            viscosity_force += (other_particle.velocity - particle.velocity) * influence;
        }

        viscosity_force * viscosity_strength
    }

    pub fn calculate_pressure_force_on_particle(
        current_particle: &Particle,
        neighbor_particles: &[Particle],
        mass: f32,
        radius: f32,
        config: &Config,
    ) -> Vec2 {
        let mut pressure_force = Vec2::ZERO;
        let current_particle_pos = current_particle.predicted_position;
        let current_particle_density = current_particle.density;

        for other_particle in neighbor_particles {
            if other_particle.id == current_particle.id {
                continue;
            }

            let distance = current_particle_pos.distance(other_particle.predicted_position);
            if distance == 0.0 || distance > radius {
                continue;
            }

            let mut direction =
                (other_particle.predicted_position - current_particle_pos).normalize_or_zero();

            if direction == Vec2::ZERO {
                let angle = rand::gen_range(0.0, 2.0 * PI);
                direction = Vec2::new(angle.cos(), angle.sin());
            }

            let slope = Self::smoothing_kernel_derivative(radius, distance);
            let shared_pressure = Self::calculate_shared_pressure(
                other_particle.density,
                current_particle_density,
                config,
            );

            if other_particle.density > 0.0 {
                pressure_force +=
                    shared_pressure * direction * slope * mass / other_particle.density;
            }
        }

        pressure_force
    }
}
