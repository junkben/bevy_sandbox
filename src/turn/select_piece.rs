use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{PickingInteraction, RaycastPickTarget},
    selection::PickSelection
};

use super::{move_piece::PendingMove, TurnState};
use crate::{piece::Piece, position::Position, resources::AvailableMoves};

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
    available_moves: Res<AvailableMoves>,
    nonpickable_query: Query<
        (Entity, &Piece, &Position),
        Without<PickSelection>
    >
) {
    // Give Selection components to pieces whose color matches the active one
    for (entity, piece, position) in nonpickable_query.iter() {
        if let Some(_moves) = available_moves.moves_from(*piece, *position) {
            commands.entity(entity).insert((
                PickSelection::default(),
                RaycastPickTarget::default()
            ));
        }
    }
}

fn disable_piece_selection(
    mut commands: Commands,
    pickable_query: Query<Entity, (With<PickSelection>, With<Piece>)>
) {
    // Remove Selection components from piece entities
    for entity in pickable_query.iter() {
        commands
            .entity(entity)
            .remove::<PickSelection>()
            .remove::<RaycastPickTarget>();
    }
}
