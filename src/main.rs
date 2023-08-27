#[macro_use] extern crate strum;

mod board;
mod camera;
mod light;
mod piece;
pub mod resources;

use bevy::{log::LogPlugin, prelude::*};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use board::BoardPlugin;
use camera::ChessCameraPlugin;
use light::ChessLightPlugin;
use piece::PiecesPlugin;
use resources::{board_state::BoardState, theme::Theme};

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa::Sample4)
        // Grab initial boardstate
        .insert_resource(BoardState::default())
        .insert_resource(Theme::Classy)
        // Set WindowDescriptor Resource to change title and size
        .add_plugins(DefaultPlugins.set(window_plugin()).set(log_plugin()))
        .add_plugins((
            PanOrbitCameraPlugin,
            DefaultPickingPlugins,
            ChessCameraPlugin,
            ChessLightPlugin,
            PiecesPlugin,
            BoardPlugin
        ))
        .run();
}

fn log_plugin() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_sandbox=debug,\
                 bevy_mod_picking=warn,naga=warn"
            .into(),
        level:  bevy::log::Level::DEBUG
    }
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "TEST".to_string(),
            resolution: (640.0, 480.0).into(),
            ..default()
        }),
        ..default()
    }
}
