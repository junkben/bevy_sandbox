use std::collections::HashMap;

use bevy::prelude::*;

use super::TurnState;
use crate::{
    piece::{color::PieceColor, Piece},
    position::Position,
    resources::BoardState
};

pub struct UpdateBoardStatePlugin;

impl Plugin for UpdateBoardStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(TurnState::UpdateBoardState),
            update_board_state
        );
    }
}

fn update_board_state(
    mut turn_state: ResMut<NextState<TurnState>>,
    mut board_state: ResMut<BoardState>,
    position_query: Query<(&Position, &Piece)>
) {
    // Clear piece placement map
    board_state.piece_placement_map = Position::ALL
        .into_iter()
        .map(|p| (p, None))
        .collect::<HashMap<Position, Option<Piece>>>();

    // Update map with pieces and their positions
    for (position, piece) in &position_query {
        board_state
            .piece_placement_map
            .insert(*position, Some(*piece));
    }

    // If Black just moved, then we've completed one turn rotation
    if board_state.active_color == PieceColor::BLACK {
        board_state.completed_turns += 1;
    }

    // Every turn advances the halfmove clock
    board_state.halfmove_clock += 1;

    // Switch active color
    use PieceColor::*;
    board_state.active_color = match board_state.active_color {
        White => PieceColor::BLACK,
        Black => PieceColor::WHITE
    };

    debug!("moving to {:?}", TurnState::End);
    turn_state.set(TurnState::End);
}
