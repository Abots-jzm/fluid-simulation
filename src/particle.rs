use macroquad::prelude::*;

use crate::simulation::DISTANCE_ZOOM;

pub struct Particle {
    pub position: Vec2,
    pub predicted_position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub density: f32,
    pub near_density: f32,
    pub is_ghost: bool,
}

impl Particle {
    pub fn new(position: Vec2, radius: f32, is_ghost: bool) -> Self {
        Self {
            position,
            predicted_position: position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            radius,
            density: 0.0,
            near_density: 0.0,
            is_ghost,
        }
    }

    pub fn draw(&self, max_speed: f32) {
        if self.is_ghost {
            return;
        }

        let speed = self.velocity.length();

        let normalized_speed = (speed / max_speed).min(1.0);

        // Color transitions: Blue (0,0,1) -> Cyan (0,1,1) -> Yellow (1,1,0) -> Red (1,0,0)
        let color = if normalized_speed < 0.2 {
            // Blue to Cyan (0.0 - 0.2)
            let t = normalized_speed / 0.2;
            Color::new(0.0, t, 1.0, 1.0)
        } else if normalized_speed < 0.5 {
            // Cyan to Yellow (0.2 - 0.5)
            let t = (normalized_speed - 0.2) / 0.3;
            Color::new(t, 1.0, 1.0 - t, 1.0)
        } else {
            // Yellow to Red (0.5 - 1.0)
            let t = (normalized_speed - 0.5) / 0.5;
            Color::new(1.0, 1.0 - t, 0.0, 1.0)
        };

        draw_circle(self.position.x, self.position.y, self.radius, color);
    }

    pub fn predict_position(&mut self) {
        self.predicted_position = self.position + self.velocity * 1. / 30.;
    }

    pub fn update(&mut self, delta_time: f32, gravity: Vec2) {
        if self.is_ghost {
            return;
        }

        self.acceleration += gravity * DISTANCE_ZOOM;
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;

        self.acceleration = Vec2::ZERO;
    }
}
