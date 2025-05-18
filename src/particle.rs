use macroquad::prelude::*;

use crate::simulation::DISTANCE_ZOOM;
use uuid::Uuid;

#[derive(Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub density: f32,
    pub id: Uuid,
}

impl Particle {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            radius,
            density: 0.0,
            id: Uuid::new_v4(),
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, BLUE);
    }

    pub fn update(&mut self, delta_time: f32, gravity: Vec2) {
        self.acceleration += gravity * DISTANCE_ZOOM;
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;

        self.acceleration = Vec2::ZERO;
    }
}
