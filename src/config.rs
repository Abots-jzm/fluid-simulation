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
}

impl Config {
    pub fn new() -> Self {
        Self {
            particle_radius: 10.0,
            particle_count: 100,
            particle_spacing: 50.0,
            particle_columns: 10,
            fluid_spawn_mode: FluidSpawnMode::Grid,
            boundary_padding: 25.0,
            boundary_damping: 0.7,
            gravity: Vec2::new(0.0, 981.),
        }
    }
}
