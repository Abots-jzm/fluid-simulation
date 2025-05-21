use macroquad::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum FluidType {
    Gas,
    Liquid,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum FluidSpawnMode {
    Grid,
    Flow,
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
    pub fluid_type: FluidType,
    pub fluid_spawn_mode: FluidSpawnMode,
    pub flow_spawn_rate: f32,
    pub flow_spawn_width: f32,
}

impl Config {
    pub fn new() -> Self {
        let fluid_type = FluidType::Liquid;
        let fluid_spawn_mode = FluidSpawnMode::Flow;

        let gravity = match fluid_type {
            FluidType::Liquid => Vec2::new(0.0, 1.),
            FluidType::Gas => Vec2::new(0.0, 0.0),
        };

        let target_density = match fluid_type {
            FluidType::Liquid => 5000.,
            FluidType::Gas => 150.0,
        };

        let pressure_multiplier = match fluid_type {
            FluidType::Liquid => 750.0,
            FluidType::Gas => 150.0,
        };

        let viscosity_strength = match fluid_type {
            FluidType::Liquid => 3.,
            FluidType::Gas => 5.,
        };

        let near_pressure_multiplier = match fluid_type {
            FluidType::Liquid => 100.0,
            FluidType::Gas => 0.0,
        };

        let interaction_strength = match fluid_type {
            FluidType::Liquid => 2500.0,
            FluidType::Gas => 5000.0,
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
            fluid_type,
            fluid_spawn_mode,
            flow_spawn_rate: 100.,
            flow_spawn_width: 120.,
        }
    }
}
