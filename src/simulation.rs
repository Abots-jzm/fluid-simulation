use macroquad::prelude::*;

use crate::{boundary::Boundary, config::Config, fluid::Fluid};

pub const DISTANCE_ZOOM: f32 = 1000.0;

pub struct Simulation {
    is_running: bool,
    is_paused: bool,
    config: Config,
    fluid: Fluid,
    boundary: Boundary,
    sample_point: Option<Vec2>,
    density_at_sample_point: Option<f32>,
}

impl Simulation {
    pub fn new() -> Self {
        let config = Config::new();

        let fluid = Fluid::from_config(&config);
        let boundary = Boundary::new(config.boundary_padding, config.boundary_damping);

        Self {
            is_running: true,
            is_paused: false,
            config,
            fluid,
            boundary,
            sample_point: None,
            density_at_sample_point: None,
        }
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.is_running = false;
        }
        if is_key_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            self.sample_point = Some(Vec2::new(mouse_pos.0, mouse_pos.1));
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.is_running {
            std::process::exit(0);
        }
        if self.is_paused {
            return;
        }

        self.fluid.update(delta_time, self.config.gravity);
        self.boundary.check_collision(&mut self.fluid.particles);

        if let Some(point) = self.sample_point {
            self.density_at_sample_point = Some(self.fluid.calculate_density(
                point,
                self.config.mass,
                self.config.smoothing_radius,
            ));
        }
    }

    pub fn render(&self) {
        self.boundary.draw();
        self.fluid.draw();
        self.draw_sample_point();
        self.draw_smoothing_kernel();
    }

    pub fn draw_sample_point(&self) {
        if let Some(point) = self.sample_point {
            draw_circle(point.x, point.y, 5.0, RED);
        }
        if let Some(density) = self.density_at_sample_point {
            let text = format!("Density: {:.2}", density);
            draw_text(&text, 10.0, 20.0, 20.0, WHITE);
        }
    }

    pub fn draw_smoothing_kernel(&self) {
        if let Some(point) = self.sample_point {
            let radius = self.config.smoothing_radius;
            draw_circle_lines(point.x, point.y, radius, 1., GREEN);
        }
    }
}
