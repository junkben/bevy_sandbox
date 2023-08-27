use bevy::prelude::*;

use super::Piece;
use crate::board::position::BoardPosition;

pub fn move_pieces(
    time: Res<Time>,
    mut query: Query<(&Piece, &BoardPosition, &mut Transform)>
) {
    for (_p, bp, mut t) in query.iter_mut() {
        // Get the direction to move in
        let move_vec = bp.vec3() - t.translation;

        // Only move if the piece isn't already there (distance is big)
        if move_vec.length() > 0.1 {
            t.translation += move_vec.normalize() * time.delta_seconds();
        }
    }
}
