use macroquad::prelude::*;

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
}

impl Particle {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            radius,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, BLUE);
    }

    pub fn update(&mut self, delta_time: f32, gravity: Vec2) {
        self.acceleration += gravity;
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
        // Reset acceleration after applying it
        self.acceleration = Vec2::ZERO;
    }
}
