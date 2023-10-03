use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::TurnState;
use crate::{
    board::{SelectSquare, Square},
    piece::{AvailableMoves, Piece, SelectPiece},
    position::Position,
    resources::{ActiveColor, PendingMove}
};

pub struct SelectMovePlugin;

#[derive(Event)]
pub struct PieceSelected;

impl Plugin for SelectMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PieceSelected>()
            .add_systems(OnEnter(TurnState::SelectMove), enable_piece_selection)
            .add_systems(
                Update,
                (
                    select_piece.run_if(on_event::<SelectPiece>()),
                    select_square.run_if(on_event::<SelectSquare>()),
                    update_square_selection.run_if(on_event::<PieceSelected>())
                )
            )
            .add_systems(
                OnExit(TurnState::SelectMove),
                (disable_square_selection, disable_piece_selection)
            );
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

    pending_move.final_position = Some(*position);

    debug!("moving to {:?}", TurnState::MovePiece);
    turn_state.set(TurnState::MovePiece);
}

fn update_square_selection(
    mut event_reader: EventReader<PieceSelected>,
    pending_move: Res<PendingMove>,
    moves_query: Query<&AvailableMoves, With<Piece>>,
    mut square_query: Query<(Entity, &Position, &mut Pickable), With<Square>>
) {
    let Some(_) = event_reader.into_iter().last() else {
        error!("not exactly one PieceSelected event");
        return;
    };

    let Some(pending_move_entity) = pending_move.entity else {
        error!("no pending_move entity, cannot enable square selection");
        return;
    };

    let Ok(moves) = moves_query.get(pending_move_entity) else {
        panic!("cannot find available moves for piece");
    };

    if moves.0.is_empty() {
        panic!("available move vector empty")
    }

    // Give Selection components to square entities
    for (entity, position, mut pickable) in square_query.iter_mut() {
        // Add selection if the square's position is an available move
        if moves.contains_move_to(position) {
            debug!("enabling pickable for square entity {:?}", entity);
            pickable.should_emit_events = true;
        } else {
            pickable.should_emit_events = false;
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

fn disable_piece_selection(
    mut commands: Commands,
    pickable_query: Query<Entity, With<Piece>>
) {
    // Remove Selection components from piece entities
    for entity in pickable_query.iter() {
        debug!("disabling pickable for piece entity {:?}", entity);
        commands.entity(entity).insert(Pickable::IGNORE);
    }
}

fn select_piece(
    mut event_reader: EventReader<SelectPiece>,
    mut event_writer: EventWriter<PieceSelected>,
    mut pending_move: ResMut<PendingMove>,
    piece_query: Query<(Entity, &Position, &Piece)>
) {
    let Some(event) = event_reader.into_iter().last() else {
        error!("not exactly one SelectPiece event");
        return;
    };

    let Ok((entity, position, piece)) = piece_query.get(event.entity) else {
        error!("no matching entity in piece query");
        return;
    };

    pending_move.initial_position = Some(*position);
    pending_move.entity = Some(entity);
    pending_move.piece = Some(*piece);
    event_writer.send(PieceSelected)
}

fn enable_piece_selection(
    mut commands: Commands,
    active_color: Res<ActiveColor>,
    nonpickable_query: Query<(Entity, &AvailableMoves, &Piece)>
) {
    // Give Selection components to pieces whose color matches the active one
    for (entity, available_moves, piece) in nonpickable_query.iter() {
        // Add selection if the piece has available moves and its color matches
        // the active one
        if !available_moves.0.is_empty()
            && piece.piece_color() == &active_color.0
        {
            debug!("enabling pickable for piece entity {:?}", entity);
            commands.entity(entity).insert(Pickable::default());
        }
    }
}
