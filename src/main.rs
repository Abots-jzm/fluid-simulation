use macroquad::prelude::*;

mod boundary;
mod config;
mod fluid;
mod particle;
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

    loop {
        clear_background(BLACK);
        simulation.handle_input();
        simulation.update(get_frame_time());
        simulation.render();
        next_frame().await
    }
}
