use macroquad::prelude::*;

mod boundary;
mod config;
mod fluid;
mod grid;
mod particle;
mod physics;
mod simulation;

use crate::simulation::Simulation;
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
    let mut simulation = Simulation::new();

    let mut fps_update_timer = 0.0;
    let mut current_fps = 0;

    loop {
        clear_background(BLACK);
        simulation.handle_input();
        simulation.update(get_frame_time());
        simulation.render();

        // Update FPS counter once per second
        fps_update_timer += get_frame_time();
        if fps_update_timer >= 1.0 {
            current_fps = get_fps();
            fps_update_timer = 0.0;
        }

        // Display the cached FPS value
        let screen_width = screen_width();
        draw_text(
            &format!("FPS: {}", current_fps),
            screen_width - 100.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
