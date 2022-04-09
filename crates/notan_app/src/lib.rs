mod config;
pub mod empty;
mod keyboard;
mod mouse;
pub mod prelude;

mod app;
mod backend;
mod builder;
mod fps_plugin;
pub mod graphics;
mod handlers;
mod parsers;
mod timer;

pub mod assets;
mod events;
mod plugins;

pub mod input {
    pub use super::keyboard::*;
    pub use super::mouse::*;
}

pub use app::*;
pub use backend::*;
pub use events::*;
pub use fps_plugin::FpsPlugin;

pub use builder::*;
pub use plugins::*;

pub use graphics::*;

pub use config::WindowConfig;
