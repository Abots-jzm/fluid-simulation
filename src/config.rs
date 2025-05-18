use macroquad::prelude::*;

use crate::fluid::FluidSpawnMode;

pub struct Config {
    pub particle_radius: f32,
    pub particle_count: u32,
    pub particle_spacing: f32,
    pub particle_columns: u32,
    pub fluid_spawn_mode: FluidSpawnMode,
    pub boundary_padding: f32,
    pub boundary_damping: f32,
    pub gravity: Vec2,
    pub mass: f32,
    pub smoothing_radius: f32,
}

impl Config {
    pub fn new() -> Self {
        let fluid_spawn_mode = FluidSpawnMode::Random;
        let gravity = match fluid_spawn_mode {
            FluidSpawnMode::Random => Vec2::new(0.0, 0.0),
            FluidSpawnMode::Grid => Vec2::new(0.0, 1.),
        };

        Self {
            particle_radius: 10.0,
            particle_count: 400,
            particle_spacing: 50.0,
            particle_columns: 10,
            boundary_padding: 25.0,
            boundary_damping: 0.7,
            gravity,
            mass: 1.0,
            smoothing_radius: 100.0,
            fluid_spawn_mode,
        }
    }
}
