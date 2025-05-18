use macroquad::prelude::*;

use crate::{boundary::Boundary, config::Config, fluid::Fluid};

pub struct Simulation {
    is_running: bool,
    is_paused: bool,
    // config: Config,
    fluid: Fluid,
    boundary: Boundary,
}

impl Simulation {
    pub fn new() -> Self {
        let config = Config::new();

        let fluid = Fluid::from_config(&config);
        let boundary = Boundary::new(config.boundary_padding);

        Self {
            is_running: true,
            is_paused: true,
            // config,
            fluid,
            boundary,
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

        self.boundary.check_collision();
    }

    pub fn render(&self) {
        self.boundary.draw();
        self.fluid.draw();
    }
}
