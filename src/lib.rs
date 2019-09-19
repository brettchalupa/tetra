//! Tetra is a simple 2D game framework written in Rust. It uses SDL2 for event handling and OpenGL 3.2+ for rendering.
//!
//! * [Website/Tutorial](https://tetra.seventeencups.net)
//! * [API Docs](https://docs.rs/tetra)
//! * [FAQ](https://tetra.seventeencups.net/FAQ)
//!
//! ## Features
//!
//! * XNA/MonoGame-inspired API
//! * Efficient 2D rendering, with draw call batching by default
//! * Simple input handling
//! * Animations/spritesheets
//! * TTF font rendering
//! * Multiple screen scaling algorithms, including pixel-perfect variants (for those chunky retro pixels)
//! * Deterministic game loop, à la [Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/)
//!
//! ## Installation
//!
//! To add Tetra to your project, add the following line to your `Cargo.toml` file:
//!
//! ```toml
//! tetra = "0.2"
//! ```
//!
//! Tetra currently requires Rust 1.32 or higher.
//!
//! You will also need to install the SDL2 native libraries, as described [here](https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements). The 'bundled' and 'static linking' features described can be activated using the `sdl2_bundled` and `sdl2_static_link` Cargo features in Tetra.
//!
//! ## Examples
//!
//! To get a simple window displayed on screen, the following code can be used:
//!
//! ```no_run
//! use tetra::graphics::{self, Color};
//! use tetra::{Context, ContextBuilder, State};
//!
//! struct GameState;
//!
//! impl State for GameState {
//!     fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
//!         // Cornflower blue, as is tradition
//!         graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     ContextBuilder::new("Hello, world!", 1280, 720).run(|_| Ok(GameState));
//! }
//! ```
//!
//! You can see this example in action by running `cargo run --example hello_world`.
//!
//! The full list of examples is available [here](https://github.com/17cupsofcoffee/tetra/tree/master/examples).
//!
//! ## Support/Feedback
//!
//! Tetra is fairly early in development, so you might run into bugs/flaky docs/general weirdness. Please feel free to open an issue/PR if you find something! You can also contact me via [Twitter](https://twitter.com/17cupsofcoffee), or find me lurking in the #games-and-graphics channel on the [Rust Community Discord](https://bit.ly/rust-community).

#![warn(missing_docs)]

pub mod audio;
pub mod error;
mod fs;
pub mod glm;
pub mod graphics;
pub mod input;
mod platform;
pub mod time;
pub mod window;

pub use crate::error::{Result, TetraError};
use crate::graphics::opengl::GLDevice;
use crate::graphics::GraphicsContext;
use crate::input::InputContext;
use crate::platform::Platform;
use crate::time::TimeContext;

/// A trait representing a type that contains game state and provides logic for updating it
/// and drawing it to the screen. This is where you'll write your game logic!
///
/// The methods on `State` allow you to return a `Result`, either explicitly or via the `?`
/// operator. If an error is returned, the game will close and the error will be returned from
/// the `run` function that was used to start it.
#[allow(unused_variables)]
pub trait State {
    /// Called when it is time for the game to update, at the interval specified by the context's
    /// tick rate.
    ///
    /// The game will update at a fixed time step (defaulting to 60fps), and draw as fast as it is
    /// allowed to (depending on CPU/vsync settings). This allows for deterministic updates even at
    /// varying framerates, but may require you to do some interpolation in the `draw` method in
    /// order to make things look smooth.
    ///
    /// See [Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/) for more info.
    fn update(&mut self, ctx: &mut Context) -> Result {
        Ok(())
    }

    /// Called when it is time for the game to be drawn.
    ///
    /// As drawing will not necessarily be in step with updating, the `dt` argument is provided -
    /// this will be a number between 0 and 1, specifying how far through the current tick you are.
    ///
    /// For example, if the player is meant to move 16 pixels per frame, and the current `dt` is 0.5,
    /// you should draw them 8 pixels along.
    fn draw(&mut self, ctx: &mut Context, dt: f64) -> Result {
        Ok(())
    }

    fn error(error: TetraError) {
        println!("Error: {}", error);
    }
}

/// A struct containing all of the 'global' state within the framework.
pub struct Context {
    platform: Platform,
    gl: GLDevice,

    graphics: GraphicsContext,
    input: InputContext,
    time: TimeContext,

    running: bool,
    quit_on_escape: bool,
}

impl Context {
    pub(crate) fn new(builder: &ContextBuilder) -> Result<Context> {
        let (platform, gl_context, width, height) = Platform::new(builder)?;
        let mut gl = GLDevice::new(gl_context)?;

        let graphics = GraphicsContext::new(&mut gl, width, height)?;
        let input = InputContext::new();
        let time = TimeContext::new(builder.tick_rate);

        Ok(Context {
            platform,
            gl,

            graphics,
            input,
            time,

            running: false,
            quit_on_escape: builder.quit_on_escape,
        })
    }
}

/// Creates a new `Context` based on the provided options.
#[derive(Debug, Clone)]
pub struct ContextBuilder {
    title: String,
    window_width: i32,
    window_height: i32,
    vsync: bool,
    tick_rate: f64,
    fullscreen: bool,
    maximized: bool,
    minimized: bool,
    resizable: bool,
    borderless: bool,
    show_mouse: bool,
    quit_on_escape: bool,
}

impl ContextBuilder {
    /// Creates a new ContextBuilder.
    pub fn new<S>(title: S, window_width: i32, window_height: i32) -> ContextBuilder
    where
        S: Into<String>,
    {
        ContextBuilder {
            title: title.into(),
            window_width,
            window_height,

            ..ContextBuilder::default()
        }
    }

    /// Sets the title of the window.
    ///
    /// Defaults to `"Tetra"`.
    pub fn title<S>(&mut self, title: S) -> &mut ContextBuilder
    where
        S: Into<String>,
    {
        self.title = title.into();
        self
    }

    /// Enables or disables vsync.
    ///
    /// Defaults to `true`.
    pub fn vsync(&mut self, vsync: bool) -> &mut ContextBuilder {
        self.vsync = vsync;
        self
    }

    /// Sets the game's update tick rate, in ticks per second.
    ///
    /// Defaults to `60.0`.
    pub fn tick_rate(&mut self, tick_rate: f64) -> &mut ContextBuilder {
        self.tick_rate = 1.0 / tick_rate;
        self
    }

    /// Sets whether or not the window should start in fullscreen.
    ///
    /// Defaults to `false`.
    pub fn fullscreen(&mut self, fullscreen: bool) -> &mut ContextBuilder {
        self.fullscreen = fullscreen;
        self
    }

    /// Sets whether or not the window should start maximized.
    ///
    /// Defaults to `false`.
    pub fn maximized(&mut self, maximized: bool) -> &mut ContextBuilder {
        self.maximized = maximized;
        self
    }

    /// Sets whether or not the window should start minimized.
    ///
    /// Defaults to `false`.
    pub fn minimized(&mut self, minimized: bool) -> &mut ContextBuilder {
        self.minimized = minimized;
        self
    }

    /// Sets whether or not the window should be resizable.
    ///
    /// Defaults to `false`.
    pub fn resizable(&mut self, resizable: bool) -> &mut ContextBuilder {
        self.resizable = resizable;
        self
    }

    /// Sets whether or not the window should be borderless.
    ///
    /// Defaults to `false`.
    pub fn borderless(&mut self, borderless: bool) -> &mut ContextBuilder {
        self.borderless = borderless;
        self
    }

    /// Sets whether or not the mouse cursor should be visible.
    ///
    /// Defaults to `false`.
    pub fn show_mouse(&mut self, show_mouse: bool) -> &mut ContextBuilder {
        self.show_mouse = show_mouse;
        self
    }

    /// Sets whether or not the game should close when the Escape key is pressed.
    ///
    /// Defaults to `false`.
    pub fn quit_on_escape(&mut self, quit_on_escape: bool) -> &mut ContextBuilder {
        self.quit_on_escape = quit_on_escape;
        self
    }

    pub fn run<S, F>(&self, init: F)
    where
        S: State + 'static,
        F: FnOnce(&mut Context) -> Result<S>,
    {
        let mut ctx = match Context::new(self) {
            Ok(ctx) => ctx,
            Err(e) => return S::error(e),
        };

        let state = match init(&mut ctx) {
            Ok(state) => state,
            Err(e) => return S::error(e),
        };

        time::reset(&mut ctx);

        ctx.running = true;
        platform::run_loop(ctx, state, run_frame);
    }
}

impl Default for ContextBuilder {
    fn default() -> ContextBuilder {
        ContextBuilder {
            title: "Tetra".into(),
            window_width: 1280,
            window_height: 720,
            vsync: true,
            tick_rate: 1.0 / 60.0,
            fullscreen: false,
            maximized: false,
            minimized: false,
            resizable: false,
            borderless: false,
            show_mouse: false,
            quit_on_escape: false,
        }
    }
}

fn run_frame<S>(ctx: &mut Context, state: &mut S)
where
    S: State,
{
    time::tick(ctx);

    if let Err(e) = platform::handle_events(ctx) {
        ctx.running = false;
        return S::error(e);
    }

    while time::is_tick_ready(ctx) {
        if let Err(e) = state.update(ctx) {
            ctx.running = false;
            return S::error(e);
        }

        input::cleanup_after_state_update(ctx);

        time::consume_tick(ctx);
    }

    if let Err(e) = state.draw(ctx, time::get_alpha(ctx)) {
        ctx.running = false;
        return S::error(e);
    }

    graphics::present(ctx);

    std::thread::yield_now();
}

// TODO: Switch to a proc macro?
// TODO: This doesn't work outside of Tetra, dependencies aren't accessible

#[macro_export]
macro_rules! wasm_main {
    ($name:ident) => {
        #[cfg(target_arch = "wasm32")]
        #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen::prelude::wasm_bindgen(start))]
        pub fn wasm_main() {
            ::console_error_panic_hook::set_once();
            $name();
        }
    };
}
