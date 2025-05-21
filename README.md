# Fluid Simulation

A 2D fluid simulation implemented in Rust using the Macroquad game engine. This simulation uses Smoothed Particle Hydrodynamics (SPH) to create realistic fluid behavior.

![screen shot](./image.png)

## Features

- Multiple fluid spawn modes: Grid, and Flow
- Multiple fluid types: Liquid and Gas
- Interactive fluid manipulation (push and pull)
- Real-time physics simulation
- Configurable parameters for fluid behavior
- Fullscreen rendering with FPS counter

## Requirements

- Rust 1.75+ (2024 edition)
- Cargo (Rust's package manager)

## Dependencies

- [macroquad](https://github.com/not-fl3/macroquad) (0.4.14) - A cross-platform game engine
- [rayon](https://github.com/rayon-rs/rayon) (1.10.0) - Data parallelism library
- [clap](https://github.com/clap-rs/clap) (4.5.4) - Command Line Argument Parser
- [serde](https://github.com/serde-rs/serde) (1.0) - Serialization/deserialization framework
- [toml](https://github.com/toml-rs/toml) (0.8.13) - TOML parsing library

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/Abots-jzm/fluid-simulation
   cd fluid-simulation
   ```

2. Build and run the project:

   ```bash
   cargo run --release
   ```

   The `--release` flag is recommended for better performance.

## Controls

- **Left Mouse Button**: Pull fluid particles
- **Right Mouse Button**: Push fluid particles
- **Space**: Pause/Resume simulation
- **Esc**: Exit application

## Configuration

The simulation behavior can be customized by modifying the `config.toml` file or by using command-line arguments.

Key parameters in `config.toml` (found in the `Config` struct in `src/config.rs`):

- `particle_radius`: Size of each fluid particle
- `particle_count`: Total number of particles in the simulation
- `fluid_type`: Type of fluid (Liquid, Gas) - can be overridden by `--fluid-type` or `-f` CLI argument.
- `fluid_spawn_mode`: Initial distribution of particles (Grid, Flow) - can be overridden by `--spawn-mode` or `-s` CLI argument.
- `gravity`: Force applied to particles
- `smoothing_radius`: Radius used for particle interactions
- `pressure_multiplier`: Strength of pressure forces
- `viscosity_strength`: Fluid thickness/stickiness
- `liquid`: Specific parameters for liquid fluid type.
- `gas`: Specific parameters for gas fluid type.

Command-line arguments can override settings from `config.toml`:

- `--fluid-type <TYPE>` or `-f <TYPE>`: Set the fluid type (e.g., `liquid`, `gas`).
- `--spawn-mode <MODE>` or `-s <MODE>`: Set the spawn mode (e.g., `grid`, `flow`).
- `--config-file <PATH>`: Specify a custom path for the configuration file (defaults to `config.toml`).

Example:

```bash
cargo run --release -- --fluid-type liquid --spawn-mode flow
```

## How It Works

This simulation uses Smoothed Particle Hydrodynamics (SPH), a computational method for simulating fluid flows. The basic principle involves:

1. Dividing the fluid into discrete particles
2. Using spatial partitioning (grid) for efficient neighbor finding
3. Calculating density, pressure, and viscosity forces between particles
4. Integrating forces to update particle positions and velocities

## Performance

The simulation uses Rayon for parallel computation of particle interactions, significantly improving performance for large numbers of particles.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
