use bevy::prelude::*;

use super::TurnState;
use crate::{piece::Piece, position::BoardPosition};

#[derive(Resource, Default)]
pub struct PendingMove {
    pub start: Option<BoardPosition>,
    pub end:   Option<BoardPosition>
}

impl PendingMove {
    fn ready(&self) -> Option<(&BoardPosition, &BoardPosition)> {
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
    mut piece_query: Query<(&mut Transform, &BoardPosition), With<Piece>>
) {
    if let Some((start, end)) = pending_move.ready() {
        for (mut transform, mut position) in piece_query.iter_mut() {
            if !position.eq(&start) {
                continue;
            }

            let direction = end.vec3() - transform.translation;
            if direction.length() > 0.1 {
                transform.translation +=
                    direction.normalize() * time.delta_seconds() * 5.0;
                debug!(?transform)
            } else {
                transform.translation = end.vec3();
                // position.set(*end.rank(), *end.file());
                turn_state.set(TurnState::UpdateBoardState);
                debug!(?turn_state)
            }
        }
    }
}

fn clear_pending_move(mut pending_move: ResMut<PendingMove>) {
    pending_move.start = None;
    pending_move.end = None;
}
