use macroquad::prelude::*;

pub struct Boundary {
    pos: Vec2,
    width: f32,
    height: f32,
}

impl Boundary {
    pub fn new(padding: f32) -> Self {
        let width = screen_width() - padding * 2.0;
        let height = screen_height() - padding * 2.0;
        let pos = Vec2::new(padding, padding);

        Self { pos, width, height }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(self.pos.x, self.pos.y, self.width, self.height, 1., WHITE);
    }

    pub fn check_collision(&self) {
        todo!();
    }
}
