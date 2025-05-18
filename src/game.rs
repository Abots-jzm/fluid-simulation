use macroquad::prelude::*;

use crate::config::{Config, Mode};

pub struct Game {
    is_running: bool,
    config: Config,
}

impl Game {
    pub fn new() -> Self {
        let config = Config::new(Mode::Default);
        Self {
            is_running: true,
            config,
        }
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.is_running = false;
        }
    }

    pub fn update(&mut self, _delta_time: f32) {
        if !self.is_running {
            std::process::exit(0);
        }
    }

    pub fn render(&self) {
        self.draw_settings();
    }

    fn draw_settings(&self) {
        // Draw the settings on the right side of the screen
        let screen_width = screen_width();
        let screen_height = screen_height();
        let settings_width = 200.0;
        let settings_height = screen_height - 20.0;
        let settings_x = screen_width - settings_width - 10.0;
        let settings_y = 10.0;

        draw_rectangle(
            settings_x,
            settings_y,
            settings_width,
            settings_height,
            Color::from_rgba(0, 0, 0, 150),
        );
        draw_rectangle_lines(
            settings_x,
            settings_y,
            settings_width,
            settings_height,
            2.0,
            WHITE,
        );
        draw_text(
            "Settings",
            settings_x + 10.0,
            settings_y + 20.0,
            20.0,
            WHITE,
        );
        let display_items = self.config.get_display_items();
        let item_height = 30.0;
        for (i, (label, value)) in display_items.iter().enumerate() {
            let y = settings_y + 50.0 + i as f32 * item_height;
            draw_text(
                &format!("{}: {}", label, value),
                settings_x + 10.0,
                y,
                20.0,
                WHITE,
            );
        }
    }
}
