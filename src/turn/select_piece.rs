use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::TurnState;
use crate::{
    piece::{AvailableMoves, Piece, SelectPiece},
    position::Position,
    resources::{ActiveColor, PendingMove}
};

pub struct SelectPiecePlugin;

impl Plugin for SelectPiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(TurnState::SelectPiece),
            enable_piece_selection
        )
        .add_systems(OnExit(TurnState::SelectPiece), disable_piece_selection)
        .add_systems(Update, select_piece.run_if(on_event::<SelectPiece>()));
    }
}

fn select_piece(
    mut events: EventReader<SelectPiece>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut pending_move: ResMut<PendingMove>,
    piece_query: Query<(Entity, &Position, &Piece)>
) {
    let Some(event) = events.into_iter().last() else {
        error!("not exactly one SelectPiece event");
        return;
    };

    let Ok((entity, _position, piece)) = piece_query.get(event.entity) else {
        error!("no matching entity in piece query");
        return;
    };

    pending_move.entity = Some(entity);
    pending_move.piece = Some(*piece);

    debug!("moving to {:?}", TurnState::SelectDestinationSquare);
    turn_state.set(TurnState::SelectDestinationSquare);
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
