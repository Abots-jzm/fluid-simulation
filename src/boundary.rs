use macroquad::prelude::*;

use crate::particle::Particle;

pub struct Boundary {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    damping: f32,
}

impl Boundary {
    pub fn new(damping: f32, grid_size: f32) -> Self {
        // Make width and height multiples of grid_size
        let width = ((screen_width() / grid_size).floor() * grid_size) - grid_size * 2.;
        let height = ((screen_height() / grid_size).floor() * grid_size) - grid_size * 2.;
        let pos = Vec2::new(
            (screen_width() - width) / 2.0,
            (screen_height() - height) / 2.0,
        );

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

    pub fn check_collision(&self, particles: &mut [Particle]) {
        for particle in particles.iter_mut() {
            if particle.is_ghost {
                continue;
            }

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
