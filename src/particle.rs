use macroquad::prelude::*;

pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    radius: f32,
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
}
