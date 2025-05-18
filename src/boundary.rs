use macroquad::prelude::*;

use crate::particle::Particle;

pub struct Boundary {
    pos: Vec2,
    width: f32,
    height: f32,
    damping: f32,
}

impl Boundary {
    pub fn new(padding: f32, damping: f32) -> Self {
        let width = screen_width() - padding * 2.0;
        let height = screen_height() - padding * 2.0;
        let pos = Vec2::new(padding, padding);

        Self {
            pos,
            width,
            height,
            damping,
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(self.pos.x, self.pos.y, self.width, self.height, 1., WHITE);
    }

    pub fn check_collision(&self, particles: &mut Vec<&mut Particle>) {
        for particle in particles.iter_mut() {
            // Left boundary
            if particle.position.x <= self.pos.x + particle.radius {
                particle.position.x = self.pos.x + particle.radius;
                // Only negate velocity if moving toward the boundary
                if particle.velocity.x < 0.0 {
                    particle.velocity.x = -particle.velocity.x * self.damping;
                }
            }
            // Right boundary
            else if particle.position.x >= self.pos.x + self.width - particle.radius {
                particle.position.x = self.pos.x + self.width - particle.radius;
                // Only negate velocity if moving toward the boundary
                if particle.velocity.x > 0.0 {
                    particle.velocity.x = -particle.velocity.x * self.damping;
                }
            }

            // Top boundary
            if particle.position.y <= self.pos.y + particle.radius {
                particle.position.y = self.pos.y + particle.radius;
                // Only negate velocity if moving toward the boundary
                if particle.velocity.y < 0.0 {
                    particle.velocity.y = -particle.velocity.y * self.damping;
                }
            }
            // Bottom boundary
            else if particle.position.y >= self.pos.y + self.height - particle.radius {
                particle.position.y = self.pos.y + self.height - particle.radius;
                // Only negate velocity if moving toward the boundary
                if particle.velocity.y > 0.0 {
                    particle.velocity.y = -particle.velocity.y * self.damping;
                }
            }
        }
    }
}
