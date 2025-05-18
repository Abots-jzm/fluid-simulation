use macroquad::prelude::*;

use crate::fluid::FluidSpawnMode;

pub struct Config {
    pub top_padding: f32,
    pub particle_radius: f32,
    pub particle_count: u32,
    pub particle_spacing: f32,
    pub fluid_spawn_mode: FluidSpawnMode,
}

impl Config {
    pub fn new() -> Self {
        Self {
            top_padding: 50.0,
            particle_radius: 5.0,
            particle_count: 100,
            particle_spacing: 50.0,
            fluid_spawn_mode: FluidSpawnMode::Grid,
        }
    }
}
