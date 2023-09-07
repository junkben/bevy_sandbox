use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{move_piece::PendingMove, TurnState};
use crate::{
    board::{Square, SquareSelectionBundle},
    piece::{AvailableMoves, Piece},
    position::Position
};

#[derive(Resource, Debug, Default)]
pub struct SelectedBoardPosition(pub Option<Position>);

pub struct SelectSquarePlugin;

impl Plugin for SelectSquarePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedBoardPosition::default())
            .add_systems(
                OnEnter(TurnState::SelectDestinationSquare),
                enable_square_selection
            )
            .add_systems(
                OnExit(TurnState::SelectDestinationSquare),
                disable_square_selection
            )
            .add_systems(
                Update,
                select_square
                    .run_if(in_state(TurnState::SelectDestinationSquare))
            );
    }
}

pub fn select_square(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut pending_move: ResMut<PendingMove>,
    mut square_query: Query<
        (Option<&PickingInteraction>, &Position),
        With<Square>
    >
) {
    // Run only if the left mouse was just pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Run through all squares
    for (interaction, board_position) in &mut square_query {
        // Go next if the picking interaction is not pressed
        if interaction != Some(&PickingInteraction::Pressed) {
            continue;
        }

        pending_move.end = Some(*board_position);
        break;
    }

    debug!("moving to {:?}", TurnState::MovePiece);
    turn_state.set(TurnState::MovePiece);
}

fn enable_square_selection(
    mut commands: Commands,
    pending_move: Res<PendingMove>,
    piece_query: Query<(&AvailableMoves, &Position), With<Piece>>,
    nonpickable_query: Query<
        (Entity, &Position),
        (With<Square>, Without<PickSelection>)
    >
) {
    let position = pending_move.start.unwrap();
    let moves = *piece_query
        .iter()
        .filter(|&(_, _position)| _position == &position)
        .map(|(moves, _)| moves)
        .collect::<Vec<_>>()
        .get(0)
        .unwrap();

    // Give Selection components to square entities
    for (entity, position) in nonpickable_query.iter() {
        // Add selection if the square's position is an available move
        if moves.0.contains(position) {
            SquareSelectionBundle::add_selection(&mut commands, entity);
        }
    }
}

fn disable_square_selection(
    mut commands: Commands,
    pickable_query: Query<Entity, (With<Square>, With<PickSelection>)>
) {
    // Remove Selection components from square entities
    for entity in pickable_query.iter() {
        SquareSelectionBundle::remove_selection(&mut commands, entity);
    }
}
