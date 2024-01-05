mod end;
mod move_piece;
mod select_move;
mod start;
mod update_board_state;

use bevy::prelude::*;

pub struct TurnManagerPlugin;
impl Plugin for TurnManagerPlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<TurnState>().add_plugins((
			start::TurnStartPlugin,
			select_move::SelectMovePlugin,
			move_piece::PieceMovementPlugin,
			update_board_state::UpdateBoardStatePlugin,
			end::EndTurnPlugin
		));
	}
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
	#[default]
	// No game in progress
	None,
	/// New turn starting
	Start,
	/// Waiting for player to select a piece and destination
	SelectMove,
	/// Waiting for piece to complete movement animation
	MovePiece,
	/// Process board state resulting from player's move
	UpdateBoardState,
	/// Turn is finishing
	End
}
