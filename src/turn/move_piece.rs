use bevy::prelude::*;

use super::TurnState;
use crate::{
    move_tracker::MoveTracker,
    physics::TranslationalMotionDone,
    piece::{MovePieceToBoardPosition, Piece},
    position::Position,
    resources::PendingMove
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PendingMove::default())
            .add_systems(OnEnter(TurnState::MovePiece), confirm_pending_move)
            .add_systems(
                Update,
                wait_for_piece_motion_to_complete
                    .run_if(in_state(TurnState::MovePiece))
            );
    }
}

fn confirm_pending_move(
    mut event_writer: EventWriter<MovePieceToBoardPosition>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut pending_move: ResMut<PendingMove>,
    mut piece_query: Query<(&mut Position, &mut MoveTracker), With<Piece>>
) {
    if let Some((entity, _piece, destination)) = pending_move.confirm() {
        let Ok((mut position, mut move_tracker)) = piece_query.get_mut(entity)
        else {
            error!("no entity matches piece query");
            return;
        };

        //
        position.set_rank(*destination.rank());
        position.set_file(*destination.file());
        move_tracker.inc();
        event_writer.send(MovePieceToBoardPosition {
            entity,
            destination
        });
    } else {
        warn!("pending move not ready: {:?}, resetting...", pending_move);
        debug!("moving back to {:?}", TurnState::Start);
        turn_state.set(TurnState::Start);
    }
}

fn wait_for_piece_motion_to_complete(
    mut event_reader: EventReader<TranslationalMotionDone>,
    mut turn_state: ResMut<NextState<TurnState>>,
    piece_query: Query<Entity, With<Piece>>
) {
    for event in event_reader.into_iter() {
        let Ok(_entity) = piece_query.get(event.entity) else {
            return;
        };

        // Event entity handshake succeeded, move to next state
        debug!("moving to {:?}", TurnState::UpdateBoardState);
        turn_state.set(TurnState::UpdateBoardState);
    }
}
