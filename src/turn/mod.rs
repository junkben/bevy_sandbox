mod end;
mod move_piece;
mod select_piece;
mod select_square;
mod start;
mod update_board_state;

use bevy::prelude::*;

pub struct TurnManagerPlugin;
impl Plugin for TurnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<TurnState>().add_plugins((
            start::TurnStartPlugin,
            select_piece::SelectPiecePlugin,
            select_square::SelectSquarePlugin,
            move_piece::PieceMovementPlugin,
            update_board_state::UpdateBoardStatePlugin,
            end::EndTurnPlugin
        ));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    /// New turn starting
    Start,
    /// Process possible destination squares for each piece
    // ProcessAvailableMoves,
    /// Waiting for player to select a piece
    SelectPiece,
    /// Piece has been selected, waiting for player to select a destination
    /// square
    SelectDestinationSquare,
    /// Waiting for piece to complete movement animation
    MovePiece,
    /// Process board state resulting from player's move
    UpdateBoardState,
    /// Turn is finishing
    End
}
