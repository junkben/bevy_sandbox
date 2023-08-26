#[macro_use] extern crate strum;

mod setup;
mod square;

use bevy::{log::LogPlugin, prelude::*};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use setup::{fen::BoardState, *};

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa::Sample4)
        // Grab initial boardstate
        .insert_resource(BoardState::default())
        // Set WindowDescriptor Resource to change title and size
        .add_plugins(DefaultPlugins.set(window_plugin()).set(log_plugin()))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(
            Startup,
            (
                create_board,
                create_camera,
                create_light,
                setup::create_pieces
            )
        )
        .run();
}

fn log_plugin() -> LogPlugin {
    LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_sandbox=debug".into(),
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
