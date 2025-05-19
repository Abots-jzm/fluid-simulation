use macroquad::prelude::*;

use crate::particle::Particle;

pub struct GridBox {
    pub particles: Vec<Particle>,
    pub grid_size: f32,
    pub position: Vec2,
}

impl GridBox {
    pub fn new(grid_size: f32, position: Vec2) -> Self {
        Self {
            particles: Vec::new(),
            grid_size,
            position,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }

    pub fn draw(&self) {
        let grid_color = Color::new(0.5, 0.5, 0.5, 0.25);
        let grid_width = self.grid_size;
        let grid_height = self.grid_size;

        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            grid_width,
            grid_height,
            1.0,
            grid_color,
        );
    }
}
