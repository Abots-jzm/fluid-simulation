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

    pub fn density_kernel(radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        let d_scaled = distance / DISTANCE_ZOOM;

        let volume = PI * r_scaled.powi(4) / 6.0;

        (r_scaled - d_scaled).powi(2) / volume
    }

    pub fn near_density_kernel(radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        let d_scaled = distance / DISTANCE_ZOOM;

        let volume = PI * r_scaled.powi(5) / 10.0;

        (r_scaled - d_scaled).powi(3) / volume
    }

    pub fn density_kernel_derivative(radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        let denominator = PI * r_scaled.powi(4);

        let scale = 12.0 / denominator;

        (distance / DISTANCE_ZOOM - r_scaled) * scale
    }

    pub fn near_density_kernel_derivative(radius: f32, distance: f32) -> f32 {
        if distance > radius {
            return 0.0;
        }

        let r_scaled = radius / DISTANCE_ZOOM;
        let d_scaled = distance / DISTANCE_ZOOM;

        let denominator = PI * r_scaled.powi(5);

        let scale_factor = -30.0 / denominator;
        let term = r_scaled - d_scaled;

        scale_factor * term * term
    }

    pub fn density_to_pressure(
        density: f32,
        near_density: f32,
        target_density: f32,
        pressure_multiplier: f32,
        near_pressure_multiplier: f32,
    ) -> (f32, f32) {
        let density_diff = density - target_density;
        let pressure = pressure_multiplier * density_diff;

        let near_pressure = near_pressure_multiplier * near_density;
        (pressure, near_pressure)
    }
    pub fn calculate_shared_pressure(
        density_a: f32,
        density_b: f32,
        near_density_a: f32,
        near_density_b: f32,
        config: &Config,
    ) -> (f32, f32) {
        let pressure_a = Self::density_to_pressure(
            density_a,
            near_density_a,
            config.target_density,
            config.pressure_multiplier,
            config.near_pressure_multiplier,
        );
        let pressure_b = Self::density_to_pressure(
            density_b,
            near_density_b,
            config.target_density,
            config.pressure_multiplier,
            config.near_pressure_multiplier,
        );

        let shared_pressure = (pressure_a.0 + pressure_b.0) / 2.0;
        let shared_near_pressure = (pressure_a.1 + pressure_b.1) / 2.0;

        (shared_pressure, shared_near_pressure)
    }

    pub fn calculate_density_from_neighbors(
        point: Vec2,
        neighbor_particle_indices: &[usize],
        particles: &[Particle],
        mass: f32,
        smoothing_radius: f32,
    ) -> f32 {
        neighbor_particle_indices
            .iter()
            .fold(0.0, |density, &neighbor_index| {
                let distance = point.distance(particles[neighbor_index].predicted_position);
                density + mass * Self::density_kernel(smoothing_radius, distance)
            })
    }

    pub fn calculate_near_density_from_neighbors(
        point: Vec2,
        neighbor_particle_indices: &[usize],
        particles: &[Particle],
        mass: f32,
        smoothing_radius: f32,
    ) -> f32 {
        neighbor_particle_indices
            .iter()
            .fold(0.0, |near_density, &neighbor_index| {
                let distance = point.distance(particles[neighbor_index].predicted_position);
                near_density + mass * Self::near_density_kernel(smoothing_radius, distance)
            })
    }

    pub fn calculate_viscosity_from_neighbors(
        current_index: usize,
        neighbor_indices: &[usize],
        particles: &[Particle],
        mass: f32,
        smoothing_radius: f32,
        viscosity_strength: f32,
    ) -> Vec2 {
        let mut viscosity_force = Vec2::ZERO;
        let current_particle = &particles[current_index];
        let current_particle_pos = current_particle.predicted_position;

        for &other_particle in neighbor_indices {
            if other_particle == current_index {
                continue;
            }

            let other_particle = &particles[other_particle];
            let distance = current_particle_pos.distance(other_particle.predicted_position);
            let influence = Self::viscosity_kernel(smoothing_radius, distance) * mass;
            viscosity_force += (other_particle.velocity - current_particle.velocity) * influence;
        }

        viscosity_force * viscosity_strength
    }

    pub fn calculate_pressure_force_on_particle(
        current_index: usize,
        neighbor_indices: &[usize],
        particles: &[Particle],
        mass: f32,
        radius: f32,
        config: &Config,
    ) -> Vec2 {
        let mut pressure_force = Vec2::ZERO;
        let current_particle = &particles[current_index];
        let current_particle_pos = current_particle.predicted_position;
        let current_particle_density = current_particle.density;

        for &other_particle in neighbor_indices {
            if other_particle == current_index {
                continue;
            }

            let other_particle = &particles[other_particle];
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
            let density_slope = Self::density_kernel_derivative(radius, distance);
            let near_density_slope = Self::near_density_kernel_derivative(radius, distance);

            let (shared_pressure, shared_near_pressure) = Self::calculate_shared_pressure(
                other_particle.density,
                current_particle_density,
                other_particle.near_density,
                current_particle.near_density,
                config,
            );

            if other_particle.density > 0.0 && other_particle.near_density > 0.0 {
                let regular_pressure_force =
                    shared_pressure * direction * density_slope * mass / other_particle.density;

                let near_pressure_force =
                    shared_near_pressure * direction * near_density_slope * mass
                        / other_particle.near_density;

                pressure_force += regular_pressure_force + near_pressure_force;
            }
        }

        pressure_force
    }
}
