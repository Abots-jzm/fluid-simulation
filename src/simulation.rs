use macroquad::prelude::*;

use crate::{config::Config, fluid::Fluid};

pub struct Simulation {
    is_running: bool,
    is_paused: bool,
    config: Config,
    fluid: Fluid,
}

impl Simulation {
    pub fn new() -> Self {
        let config = Config::new();

        let fluid = Fluid::from_config(&config);

        Self {
            is_running: true,
            is_paused: true,
            config,
            fluid,
        }
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.is_running = false;
        }
        if is_key_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
        }
    }

    pub fn update(&mut self, _delta_time: f32) {
        if !self.is_running {
            std::process::exit(0);
        }
        if self.is_paused {
            return;
        }
    }

    pub fn render(&self) {
        self.fluid.draw();
    }
}
