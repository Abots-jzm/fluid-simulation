use macroquad::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum FluidSpawnMode {
    Gas,
    Liquid,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InteractionType {
    Pull,
    Push,
}

pub struct Config {
    pub particle_radius: f32,
    pub particle_count: u32,
    pub boundary_damping: f32,
    pub gravity: Vec2,
    pub mass: f32,
    pub smoothing_radius: f32,
    pub target_density: f32,
    pub pressure_multiplier: f32,
    pub near_pressure_multiplier: f32,
    pub interaction_strength: f32,
    pub interaction_radius: f32,
    pub viscosity_strength: f32,
    pub target_ghost_spacing: f32,
    pub start_ghost_spacing_multiplier: f32,
    pub ghost_wall_start_percent: f32,
    pub fluid_spawn_mode: FluidSpawnMode,
}

impl Config {
    pub fn new() -> Self {
        let fluid_spawn_mode = FluidSpawnMode::Liquid;

        let gravity = match fluid_spawn_mode {
            FluidSpawnMode::Liquid => Vec2::new(0.0, 1.),
            FluidSpawnMode::Gas => Vec2::new(0.0, 0.0),
        };

        let target_density = match fluid_spawn_mode {
            FluidSpawnMode::Liquid => 5000.,
            FluidSpawnMode::Gas => 150.0,
        };

        let pressure_multiplier = match fluid_spawn_mode {
            FluidSpawnMode::Liquid => 750.0,
            FluidSpawnMode::Gas => 150.0,
        };

        let viscosity_strength = match fluid_spawn_mode {
            FluidSpawnMode::Liquid => 3.,
            FluidSpawnMode::Gas => 5.,
        };

        let near_pressure_multiplier = match fluid_spawn_mode {
            FluidSpawnMode::Liquid => 100.0,
            FluidSpawnMode::Gas => 0.0,
        };

        let interaction_strength = match fluid_spawn_mode {
            FluidSpawnMode::Liquid => 3500.0,
            FluidSpawnMode::Gas => 5000.0,
        };

        Self {
            particle_radius: 3.,
            particle_count: 3000,
            boundary_damping: 0.7,
            gravity,
            mass: 1.0,
            smoothing_radius: 40.0,
            target_density,
            pressure_multiplier,
            near_pressure_multiplier,
            interaction_strength,
            interaction_radius: 200.0,
            viscosity_strength,
            target_ghost_spacing: 3.,
            start_ghost_spacing_multiplier: 2.6,
            ghost_wall_start_percent: 0.6,
            fluid_spawn_mode,
        }
    }
}
