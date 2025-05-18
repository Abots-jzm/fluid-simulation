use macroquad::prelude::*;

pub struct Game {
    is_running: bool,
}

impl Game {
    pub fn new() -> Self {
        Self { is_running: true }
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.is_running = false;
        }
    }

    pub fn update(&mut self, _delta_time: f32) {
        // Update game state here
    }

    pub fn render(&self) {
        // Render game state here
    }
}
