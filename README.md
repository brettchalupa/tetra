# Tetra

[![Build Status](https://github.com/17cupsofcoffee/tetra/workflows/CI%20Build/badge.svg?branch=master)](https://github.com/17cupsofcoffee/tetra/actions?query=branch%3Amaster)
[![Crates.io](https://img.shields.io/crates/v/tetra.svg)](https://crates.io/crates/tetra)
[![Documentation](https://docs.rs/tetra/badge.svg)](https://docs.rs/tetra)
[![License](https://img.shields.io/crates/l/tetra.svg)](LICENSE)

Tetra is a simple 2D game framework written in Rust. It uses SDL2 for event handling and OpenGL 3.2+ for rendering.

* [Website](https://tetra.seventeencups.net)
* [API Docs](https://docs.rs/tetra)
* [FAQ](https://tetra.seventeencups.net/FAQ)

## Features

* XNA/MonoGame-inspired API
* Efficient 2D rendering, with draw call batching by default
* Easy input handling, via polling or events, with support for gamepads
* Deterministic game loop by default, à la [Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/)
* Common building blocks built-in, such as:
    * Font rendering
    * Cameras
    * Screen scaling

## Installation

To add Tetra to your project, add the following line to your `Cargo.toml` file:

```toml
tetra = "0.3"
```

You will also need to install the SDL2 native libraries - full details are provided in the [documentation](https://tetra.seventeencups.net/installation).

## Examples

To get a simple window displayed on screen, the following code can be used:

```rust ,noplaypen
use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", 1280, 720)
        .build()?
        .run(|_| Ok(GameState))
}
```

You can see this example in action by running `cargo run --example hello_world`.

The full list of examples is available [here](https://github.com/17cupsofcoffee/tetra/tree/master/examples).

## Support/Feedback

Tetra is fairly early in development, so you might run into bugs/flaky docs/general weirdness. Please feel free to open an issue/PR if you find something! You can also contact me via [Twitter](https://twitter.com/17cupsofcoffee), or find me lurking in the #games-and-graphics channel on the [Rust Community Discord](https://bit.ly/rust-community).
