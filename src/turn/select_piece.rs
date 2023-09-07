use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{move_piece::PendingMove, TurnState};
use crate::{
    piece::{AvailableMoves, Piece, PieceSelectionBundle},
    position::Position,
    resources::ActiveColor
};

pub struct SelectPiecePlugin;

impl Plugin for SelectPiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(TurnState::SelectPiece),
            enable_piece_selection
        )
        .add_systems(OnExit(TurnState::SelectPiece), disable_piece_selection)
        .add_systems(
            Update,
            select_piece.run_if(in_state(TurnState::SelectPiece))
        );
    }
}

fn select_piece(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut pending_move: ResMut<PendingMove>,
    mut piece_query: Query<(Option<&PickingInteraction>, &Position, &Piece)>
) {
    // Ensure this runs exactly once when the left mouse button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Run through all pieces
    for (interaction, board_position, piece) in &mut piece_query {
        // Go next if the picking interaction is not pressed
        if interaction != Some(&PickingInteraction::Pressed) {
            continue;
        }

        pending_move.start = Some(*board_position);
        pending_move.piece = Some(*piece);
        break;
    }

    debug!("moving to {:?}", TurnState::SelectDestinationSquare);
    turn_state.set(TurnState::SelectDestinationSquare);
}

fn enable_piece_selection(
    mut commands: Commands,
    active_color: Res<ActiveColor>,
    nonpickable_query: Query<
        (Entity, &AvailableMoves, &Piece),
        (With<Piece>, With<Position>, Without<PickSelection>)
    >
) {
    // Give Selection components to pieces whose color matches the active one
    for (entity, available_moves, piece) in nonpickable_query.iter() {
        // Add selection if the piece has available moves and its color matches
        // the active one
        if !available_moves.0.is_empty()
            && piece.piece_color() == &active_color.0
        {
            PieceSelectionBundle::add_selection(&mut commands, entity);
        }
    }
}

fn disable_piece_selection(
    mut commands: Commands,
    pickable_query: Query<Entity, (With<PickSelection>, With<Piece>)>
) {
    // Remove Selection components from piece entities
    for entity in pickable_query.iter() {
        PieceSelectionBundle::remove_selection(&mut commands, entity)
    }
}
