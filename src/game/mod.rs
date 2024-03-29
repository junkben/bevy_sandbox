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
mod player;
mod position;
pub mod resources;
mod settings;
mod square;
mod start;
mod team;
mod turn;

use bevy::prelude::*;
pub use move_info::{MoveInfo, MoveType};
pub use settings::GameSettings;
pub use start::GameStart;

use self::turn::TurnState;
use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			// Set antialiasing to use 4 samples
			.insert_resource(Msaa::Sample4)
			// Add resources first
			.add_plugins(resources::ResourcesPlugin)
			// System plugins
			.add_plugins((
				bevy_panorbit_camera::PanOrbitCameraPlugin,
				bevy_mod_picking::DefaultPickingPlugins
			))
			// Game-related plugins
			.add_plugins((
				board::BoardPlugin,
				camera::ChessCameraPlugin,
				light::ChessLightPlugin,
				physics::MotionPlugin,
				piece::PiecesPlugin,
				settings::GameSettingsPlugin,
				square::SquarePlugin,
				start::GameStartPlugin,
				turn::TurnManagerPlugin
			))
			.add_systems(
				OnEnter(GameState::Game),
				entering_game_state.run_if(in_state(TurnState::None))
			);
	}
}

fn entering_game_state(mut ew_game_start: EventWriter<GameStart>) {
	ew_game_start.send(GameStart);
}
