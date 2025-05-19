use macroquad::prelude::*;

#[allow(dead_code)]
pub enum FluidSpawnMode {
    Random, // kinda like gas
    Grid,
    Gravity, // kinda like liquid
}

#[derive(Clone, Copy, PartialEq)]
pub enum InteractionType {
    Pull,
    Push,
}

pub struct Config {
    pub particle_radius: f32,
    pub particle_count: u32,
    pub fluid_spawn_mode: FluidSpawnMode,
    pub boundary_padding: f32,
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
}

impl Config {
    pub fn new() -> Self {
        //Benchmark values
        // let fluid_spawn_mode = FluidSpawnMode::Gravity;
        // let gravity = match fluid_spawn_mode {
        //     FluidSpawnMode::Gravity => Vec2::new(0.0, 0.75),
        //     _ => Vec2::new(0.0, 0.0),
        // };

        // let target_density = match fluid_spawn_mode {
        //     FluidSpawnMode::Gravity => 15_000.0,
        //     _ => 150.0,
        // };

        // let pressure_multiplier = match fluid_spawn_mode {
        //     FluidSpawnMode::Gravity => 250.0,
        //     _ => 150.0,
        // };

        // let viscosity_strength = match fluid_spawn_mode {
        //     FluidSpawnMode::Gravity => 10.,
        //     _ => 10.,
        // };

        // let near_pressure_multiplier = match fluid_spawn_mode {
        //     FluidSpawnMode::Gravity => 0.0,
        //     _ => 0.0,
        // };

        // let interaction_strength = match fluid_spawn_mode {
        //     FluidSpawnMode::Gravity => 3500.0,
        //     _ => 5000.0,
        // };
        // Self {
        //     particle_radius: 2.,
        //     particle_count: 10_000,
        //     boundary_padding: 100.,
        //     boundary_damping: 0.7,
        //     gravity,
        //     mass: 1.0,
        //     smoothing_radius: 40.0,
        //     fluid_spawn_mode,
        //     target_density,
        //     pressure_multiplier,
        //     near_pressure_multiplier,
        //     interaction_strength,
        //     interaction_radius: 200.0,
        //     viscosity_strength,
        // }

        //REALISTIC VALUES
        let fluid_spawn_mode = FluidSpawnMode::Gravity;
        let gravity = match fluid_spawn_mode {
            FluidSpawnMode::Gravity => Vec2::new(0.0, 0.75),
            _ => Vec2::new(0.0, 0.0),
        };

        let target_density = match fluid_spawn_mode {
            FluidSpawnMode::Gravity => 3000.,
            _ => 150.0,
        };

        let pressure_multiplier = match fluid_spawn_mode {
            FluidSpawnMode::Gravity => 750.0,
            _ => 150.0,
        };

        let viscosity_strength = match fluid_spawn_mode {
            FluidSpawnMode::Gravity => 5.,
            _ => 10.,
        };

        let near_pressure_multiplier = match fluid_spawn_mode {
            FluidSpawnMode::Gravity => 100.0,
            _ => 0.0,
        };

        let interaction_strength = match fluid_spawn_mode {
            FluidSpawnMode::Gravity => 3500.0,
            _ => 5000.0,
        };

        Self {
            particle_radius: 4.,
            particle_count: 1_500,
            boundary_padding: 100.,
            boundary_damping: 0.7,
            gravity,
            mass: 1.0,
            smoothing_radius: 40.0,
            fluid_spawn_mode,
            target_density,
            pressure_multiplier,
            near_pressure_multiplier,
            interaction_strength,
            interaction_radius: 200.0,
            viscosity_strength,
        }
    }
}
