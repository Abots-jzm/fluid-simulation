use macroquad::prelude::*;

mod config;
mod game;

use crate::game::Game;
fn window_conf() -> Conf {
    Conf {
        window_title: "Fluid Simulation".to_owned(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BLACK);
        game.handle_input();
        game.update(get_frame_time());
        println!("FPS: {}", get_fps());
        game.render();
        next_frame().await
    }
}
