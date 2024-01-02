use bevy::prelude::*;

use super::TurnState;
use crate::game::{
	piece::{Piece, SpawnPiece, INITIAL_PIECE_POSITIONS},
	position::Position
};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(TurnState::GameStart), spawn_initial_pieces)
			.add_systems(
				Update,
				wait_for_piece_spawning.run_if(in_state(TurnState::GameStart))
			);
	}
}

fn spawn_initial_pieces(mut ew_spawn_piece: EventWriter<SpawnPiece>) {
	// Spawn pieces in proper squares
	for (position, piece) in INITIAL_PIECE_POSITIONS.clone().into_iter() {
		ew_spawn_piece.send(SpawnPiece { piece, position });
	}
}

fn wait_for_piece_spawning(
	mut turn_state: ResMut<NextState<TurnState>>,
	query_piece: Query<(&Position, &Piece)>
) {
	// Spawn pieces in proper squares
	for (a, b) in INITIAL_PIECE_POSITIONS.iter() {
		let mut spawned = false;

		for (position, piece) in query_piece.iter() {
			if position == a && piece == b {
				spawned = true;
				break;
			}
		}

		if !spawned {
			trace!("not done spawning pieces yet...");
			return;
		}
	}

	trace!("done spawning pieces, moving to {:?}", TurnState::Start);
	turn_state.set(TurnState::Start);
}
