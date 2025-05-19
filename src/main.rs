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
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();

    //BENCHMARK CODE

    // // Adjustable simulation time in seconds
    // let simulation_duration = 10.0; //benchmark code

    // let mut fps_update_timer = 0.0;
    // let mut current_fps = 0;
    // let mut elapsed_time = 0.0; // benchmark code

    // // FPS statistics - benchmark code
    // let mut min_fps = i32::MAX;
    // let mut max_fps = 0;
    // let mut fps_sum = 0;
    // let mut fps_count = 0;

    // while elapsed_time < simulation_duration {
    //     clear_background(BLACK);
    //     simulation.handle_input();
    //     simulation.update(get_frame_time());
    //     simulation.render();

    //     let frame_time = get_frame_time(); // benchmark code
    //     elapsed_time += frame_time; // benchmark code

    //     // Update FPS counter once per second
    //     fps_update_timer += frame_time;
    //     if fps_update_timer >= 1.0 {
    //         current_fps = get_fps();
    //         fps_update_timer = 0.0;

    //         // Update FPS statistics - benchmark code
    //         min_fps = min_fps.min(current_fps);
    //         max_fps = max_fps.max(current_fps);
    //         fps_sum += current_fps;
    //         fps_count += 1;
    //     }

    //     // Display the cached FPS value and remaining time
    //     let screen_width = screen_width();
    //     draw_text(
    //         &format!(
    //             "FPS: {} | Time: {:.1}/{:.1}s", //some are benchmark code
    //             current_fps, elapsed_time, simulation_duration
    //         ),
    //         screen_width - 250.0,
    //         20.0,
    //         20.0,
    //         WHITE,
    //     );

    //     next_frame().await
    // }

    // // Calculate average FPS - benchmark code
    // let avg_fps = if fps_count > 0 {
    //     fps_sum as f32 / fps_count as f32
    // } else {
    //     0.0
    // };

    // // Print FPS statistics to console
    // println!("------- FPS Statistics -------");
    // println!("Highest FPS: {}", max_fps);
    // println!("Lowest FPS: {}", min_fps);
    // println!("Average FPS: {:.2}", avg_fps);
    // println!("-----------------------------");

    //REAL CODE
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
