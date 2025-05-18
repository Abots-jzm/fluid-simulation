use macroquad::prelude::*;

#[allow(dead_code)]
pub enum FluidSpawnMode {
    Random,
    Grid,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InteractionType {
    Pull,
    Push,
}

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
    pub target_density: f32,
    pub pressure_multiplier: f32,
    pub interaction_strength: f32,
    pub interaction_radius: f32,
}

impl Config {
    pub fn new() -> Self {
        let fluid_spawn_mode = FluidSpawnMode::Grid;
        let gravity = match fluid_spawn_mode {
            FluidSpawnMode::Random => Vec2::new(0.0, 0.0),
            FluidSpawnMode::Grid => Vec2::new(0.0, 0.),
        };

        Self {
            particle_radius: 5.0,
            particle_count: 1500,
            particle_spacing: 15.,
            particle_columns: 50,
            boundary_padding: 25.0,
            boundary_damping: 0.7,
            gravity,
            mass: 1.0,
            smoothing_radius: 40.0,
            fluid_spawn_mode,
            target_density: 150.0,
            pressure_multiplier: 250.,
            interaction_strength: 5000.,
            interaction_radius: 200.0,
        }
    }
}
