mod board;
mod camera;
mod end_game;
mod light;
mod move_info;
mod move_tracker;
mod physics;
mod piece;
mod position;
pub mod resources;
mod settings;
mod turn;

use bevy::{
	diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
	prelude::*
};
pub use move_info::{MoveInfo, MoveType};
pub use settings::GameSettings;

fn main() {
	App::new()
		// Set antialiasing to use 4 samples
		.insert_resource(Msaa::Sample4)
		// Add in Game Settings
		.insert_resource(GameSettings::default())
		// Add resources first
		.add_plugins(resources::ResourcesPlugin)
		// Set WindowDescriptor Resource to change title and size
		.add_plugins(DefaultPlugins.set(window_plugin()).set(log_plugin()))
		.add_plugins((
			bevy_panorbit_camera::PanOrbitCameraPlugin,
			bevy_mod_picking::DefaultPickingPlugins,
			turn::TurnManagerPlugin,
			camera::ChessCameraPlugin,
			light::ChessLightPlugin,
			piece::PiecesPlugin,
			board::BoardPlugin,
			physics::MotionPlugin
		))
		//.add_plugins((
		// 	LogDiagnosticsPlugin::default(),
		// 	FrameTimeDiagnosticsPlugin
		//))
		.run();
}

fn log_plugin() -> bevy::log::LogPlugin {
	bevy::log::LogPlugin {
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
