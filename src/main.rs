use macroquad::prelude::*;

mod boundary;
mod config;
mod fluid;
mod grid;
mod particle;
mod physics;
mod simulation;
mod spawner;

use crate::config::{Config, FluidSpawnMode, FluidType};
use crate::simulation::Simulation;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, short = 'f', value_enum)]
    fluid_type: Option<FluidType>,

    #[clap(long, short = 's', value_enum)]
    spawn_mode: Option<FluidSpawnMode>,

    #[clap(long, default_value = "config.toml")]
    config_file: String,
}

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
    let cli = Cli::parse();

    let config_str = fs::read_to_string(&cli.config_file)
        .unwrap_or_else(|e| panic!("Failed to read config file '{}': {}", cli.config_file, e));

    let mut config: Config = toml::from_str(&config_str)
        .unwrap_or_else(|e| panic!("Failed to parse config file '{}': {}", cli.config_file, e));

    if let Some(ft) = cli.fluid_type {
        config.fluid_type = ft;
    }
    if let Some(sm) = cli.spawn_mode {
        config.fluid_spawn_mode = sm;
    }

    if config.fluid_type == FluidType::Gas && config.fluid_spawn_mode == FluidSpawnMode::Flow {
        panic!("Flow spawn mode is not supported for Gas fluid type.");
    }

    config.adapt_to_fluid_type();

    let mut simulation = Simulation::new(config);

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

        total_time += frame_time;
        frame_count += 1;

        fps_update_timer += frame_time;
        if fps_update_timer >= 1.0 {
            avg_fps = if total_time > 0.0 {
                (frame_count as f32 / total_time).round() as i32
            } else {
                0
            };
            fps_update_timer = 0.0;
            frame_count = 0;
            total_time = 0.0;
        }

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
