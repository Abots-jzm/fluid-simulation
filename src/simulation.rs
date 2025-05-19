use macroquad::prelude::*;

use crate::{
    boundary::Boundary,
    config::{Config, InteractionType},
    fluid::Fluid,
};

pub const DISTANCE_ZOOM: f32 = 1000.0;

pub struct Simulation {
    is_running: bool,
    is_paused: bool,
    config: Config,
    fluid: Fluid,
    boundary: Boundary,
    click_point: Option<Vec2>,
    interaction_type: Option<InteractionType>,
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
            click_point: None,
            interaction_type: None,
        }
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.is_running = false;
        }
        if is_key_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            self.click_point = Some(Vec2::new(mouse_pos.0, mouse_pos.1));
            self.interaction_type = Some(InteractionType::Pull);
        }

        if is_mouse_button_down(MouseButton::Right) {
            let mouse_pos = mouse_position();
            self.click_point = Some(Vec2::new(mouse_pos.0, mouse_pos.1));
            self.interaction_type = Some(InteractionType::Push);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.click_point = None;
            self.interaction_type = None;
        }

        if is_mouse_button_released(MouseButton::Right) {
            self.click_point = None;
            self.interaction_type = None;
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.is_running {
            std::process::exit(0);
        }
        if self.is_paused {
            return;
        }

        self.fluid
            .update(delta_time, self.config.gravity, &self.config);

        // Handle interaction before collecting particles
        if let Some(click_point) = self.click_point {
            if let Some(interaction_type) = &self.interaction_type {
                self.fluid
                    .handle_interaction(click_point, *interaction_type, &self.config);
            }
        }

        let mut particles = Vec::new();
        for grid in &mut self.fluid.grid {
            for particle in &mut grid.particles {
                particles.push(particle);
            }
        }

        self.boundary.check_collision(&mut particles);
    }

    pub fn render(&self) {
        self.boundary.draw();
        self.fluid.draw();
        self.draw_interaction_radius();
    }

    pub fn draw_interaction_radius(&self) {
        if let Some(point) = self.click_point {
            let radius = self.config.interaction_radius;
            draw_circle_lines(point.x, point.y, radius, 1., GREEN);
        }
    }
}
