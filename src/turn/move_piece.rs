use bevy::prelude::*;

use super::TurnState;
use crate::{
    move_tracker::MoveTracker,
    physics::TranslationalMotionDone,
    piece::{MovePieceToBoardPosition, Piece},
    position::Position,
    resources::{MoveHistory, PendingMove}
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
    mut move_history: ResMut<MoveHistory>,
    mut piece_query: Query<(&mut Position, &mut MoveTracker), With<Piece>>
) {
    if let Some(mi) = pending_move.confirm() {
        let Ok((mut position, mut move_tracker)) =
            piece_query.get_mut(mi.entity)
        else {
            error!("no entity matches piece query");
            return;
        };

        // Update position of piece to new position
        position.set_rank(*mi.final_position.rank());
        position.set_file(*mi.final_position.file());

        // Increment the move tracker
        move_tracker.inc();

        // Send the event to physically move the piece to the new board position
        event_writer.send(MovePieceToBoardPosition {
            entity:      mi.entity,
            destination: mi.final_position
        });

        // Add the move to the MoveHistory resource
        move_history.append_move(mi);
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
