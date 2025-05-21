use clap::ValueEnum;
use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct SerializableVec2 {
    pub x: f32,
    pub y: f32,
}

impl From<SerializableVec2> for Vec2 {
    fn from(sv: SerializableVec2) -> Self {
        Vec2::new(sv.x, sv.y)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum FluidType {
    Gas,
    Liquid,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum FluidSpawnMode {
    Grid,
    Flow,
}

#[derive(Clone, Copy, PartialEq, Deserialize, Debug)]
pub enum InteractionType {
    Pull,
    Push,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FluidTypeSpecifics {
    pub gravity: SerializableVec2,
    pub target_density: f32,
    pub pressure_multiplier: f32,
    pub near_pressure_multiplier: f32,
    pub viscosity_strength: f32,
    pub interaction_strength: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub particle_radius: f32,
    pub particle_count: u32,
    pub boundary_damping: f32,
    pub mass: f32,
    pub smoothing_radius: f32,
    pub interaction_radius: f32,

    // Active parameters (populated by adapt_to_fluid_type)
    #[serde(skip)]
    pub gravity: Vec2,
    #[serde(skip)]
    pub target_density: f32,
    #[serde(skip)]
    pub pressure_multiplier: f32,
    #[serde(skip)]
    pub near_pressure_multiplier: f32,
    #[serde(skip)]
    pub viscosity_strength: f32,
    #[serde(skip)]
    pub interaction_strength: f32,

    pub target_ghost_spacing: f32,
    pub start_ghost_spacing_multiplier: f32,
    pub ghost_wall_start_percent: f32,
    pub fluid_type: FluidType,
    pub fluid_spawn_mode: FluidSpawnMode,
    pub flow_spawn_rate: f32,
    pub flow_spawn_width: f32,
    pub liquid: FluidTypeSpecifics,
    pub gas: FluidTypeSpecifics,
}

impl Config {
    pub fn adapt_to_fluid_type(&mut self) {
        let specifics = match self.fluid_type {
            FluidType::Liquid => &self.liquid,
            FluidType::Gas => &self.gas,
        };

        self.gravity = specifics.gravity.into();
        self.target_density = specifics.target_density;
        self.pressure_multiplier = specifics.pressure_multiplier;
        self.near_pressure_multiplier = specifics.near_pressure_multiplier;
        self.viscosity_strength = specifics.viscosity_strength;
        self.interaction_strength = specifics.interaction_strength;
    }
}
