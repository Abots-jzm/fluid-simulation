use macroquad::prelude::*;

mod boundary;
mod config;
mod fluid;
mod grid;
mod particle;
mod physics;
mod simulation;
mod spawner;

use crate::simulation::Simulation;
fn window_conf() -> Conf {
    Conf {
        window_title: "Fluid Simulation".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();

    let mut fps_update_timer = 0.0;
    let mut avg_fps = 0;
    let mut frame_count = 0;
    let mut total_time = 0.0;

    loop {
        let frame_time = get_frame_time();
        clear_background(BLACK);
        simulation.handle_input();
        simulation.update(frame_time);
        simulation.render();

        // Track total time and frames for average FPS calculation
        total_time += frame_time;
        frame_count += 1;

        // Update FPS counter once per second
        fps_update_timer += frame_time;
        if fps_update_timer >= 1.0 {
            // Calculate average FPS over the last second
            avg_fps = if total_time > 0.0 {
                (frame_count as f32 / total_time).round() as i32
            } else {
                0
            };
            fps_update_timer = 0.0;
            frame_count = 0;
            total_time = 0.0;
        }

        // Display the average FPS
        let screen_width = screen_width();
        draw_text(
            &format!("FPS: {}", avg_fps),
            screen_width - 100.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
