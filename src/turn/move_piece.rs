use bevy::prelude::*;

use super::TurnState;
use crate::{move_tracker::MoveTracker, piece::Piece, position::Position};

#[derive(Resource, Default, Debug)]
pub struct PendingMove {
    pub piece: Option<Piece>,
    pub start: Option<Position>,
    pub end:   Option<Position>
}

impl PendingMove {
    fn ready(&self) -> Option<(&Position, &Position)> {
        self.start.as_ref().zip(self.end.as_ref())
    }
}

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PendingMove::default())
            .add_systems(
                Update,
                move_piece.run_if(in_state(TurnState::MovePiece))
            )
            .add_systems(OnExit(TurnState::MovePiece), clear_pending_move);
    }
}

fn move_piece(
    time: Res<Time>,
    mut turn_state: ResMut<NextState<TurnState>>,
    pending_move: Res<PendingMove>,
    mut piece_query: Query<
        (&mut Transform, &mut Position, &mut MoveTracker),
        With<Piece>
    >
) {
    if let Some((start, end)) = pending_move.ready() {
        for (mut transform, mut position, mut move_tracker) in
            piece_query.iter_mut()
        {
            if !position.eq(&start) {
                continue;
            }

            let direction = end.translation() - transform.translation;
            if direction.length() > 0.1 {
                transform.translation +=
                    direction.normalize() * time.delta_seconds() * 3.0;
            } else {
                transform.translation = end.translation();

                // TODO: find better way to reassign positions
                position.set_rank(*end.rank());
                position.set_file(*end.file());
                move_tracker.inc();

                debug!("moving to {:?}", TurnState::Start);
                turn_state.set(TurnState::UpdateBoardState);
                break;
            }
        }
    } else {
        warn!("pending move not ready: {:?}, resetting...", pending_move);
        debug!("moving to {:?}", TurnState::Start);
        turn_state.set(TurnState::Start);
    }
}

fn clear_pending_move(mut pending_move: ResMut<PendingMove>) {
    pending_move.start = None;
    pending_move.end = None;
}
