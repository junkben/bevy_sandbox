// This lint usually gives bad advice in the context of Bevy -- hiding complex
// queries behind type aliases tends to obfuscate code while offering no
// improvement in code cleanliness.
#![allow(clippy::type_complexity)]

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

use bevy::prelude::*;
pub use move_info::{MoveInfo, MoveType};
pub use settings::GameSettings;

use self::turn::TurnState;
use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			// Set antialiasing to use 4 samples
			.insert_resource(Msaa::Sample4)
			// Add in Game Settings
			.insert_resource(GameSettings::default())
			// Add resources first
			.add_plugins(resources::ResourcesPlugin)
			// Set WindowDescriptor Resource to change title and size
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
			.add_systems(
				OnEnter(GameState::Game),
				entering_game_state.run_if(in_state(TurnState::None))
			);
	}
}

fn entering_game_state(mut turn_state: ResMut<NextState<TurnState>>) {
	turn_state.set(TurnState::GameStart);
}