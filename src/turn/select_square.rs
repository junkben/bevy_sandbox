use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::TurnState;
use crate::{
    board::{SelectSquare, Square},
    piece::{AvailableMoves, Piece},
    position::Position,
    resources::PendingMove
};

pub struct SelectSquarePlugin;

impl Plugin for SelectSquarePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(TurnState::SelectDestinationSquare),
            enable_square_selection
        )
        .add_systems(
            OnExit(TurnState::SelectDestinationSquare),
            disable_square_selection
        )
        .add_systems(Update, select_square.run_if(on_event::<SelectSquare>()));
    }
}

fn select_square(
    mut events: EventReader<SelectSquare>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut pending_move: ResMut<PendingMove>,
    square_query: Query<&Position, With<Square>>
) {
    let Some(event) = events.into_iter().last() else {
        error!("not exactly one SelectSquare event");
        return;
    };

    let Ok(position) = square_query.get(event.entity) else {
        error!("no matching entity in square query");
        return;
    };

    pending_move.destination = Some(*position);

    debug!("moving to {:?}", TurnState::MovePiece);
    turn_state.set(TurnState::MovePiece);
}

fn enable_square_selection(
    mut commands: Commands,
    pending_move: Res<PendingMove>,
    piece_query: Query<&AvailableMoves, With<Piece>>,
    nonpickable_query: Query<(Entity, &Position), With<Square>>
) {
    let Some(pending_move_entity) = pending_move.entity else {
        error!("no pending_move entity, cannot enable square selection");
        return;
    };

    let Ok(moves) = piece_query.get(pending_move_entity) else {
        panic!("cannot find available moves for piece");
    };

    if moves.0.is_empty() {
        panic!("available move vector empty")
    }

    // Give Selection components to square entities
    for (entity, position) in nonpickable_query.iter() {
        // Add selection if the square's position is an available move
        if moves.0.contains(position) {
            debug!("enabling pickable for square entity {:?}", entity);
            commands.entity(entity).insert(Pickable::default());
        }
    }
}

fn disable_square_selection(
    mut commands: Commands,
    mut pickable_query: Query<(Entity, &mut PickSelection), With<Square>>
) {
    // Remove Selection components from square entities
    for (entity, mut selection) in pickable_query.iter_mut() {
        selection.is_selected = false;

        debug!("disabling pickable for square entity {:?}", entity);
        commands.entity(entity).insert(Pickable::IGNORE);
    }
}
