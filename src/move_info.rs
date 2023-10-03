use bevy::prelude::*;

use crate::{piece::Piece, position::Position};

#[derive(Debug, Default, Copy, Clone)]
pub enum MoveType {
    /// A piece is moved to an unoccupied square
    #[default]
    Move,

    /// A piece is moved to a space occupied by an opponent's piece, which is
    /// captured and removed from play. With the sole exception of en passant,
    /// all pieces capture by moving to the square that the opponent's piece
    /// occupies.
    Capture {
        is_en_passant: bool
    },

    PawnPromotion {
        promoted_to: Piece
    },
    Castle {
        is_kingside: bool
    },
    Check,
    DrawOffer
}

#[derive(Debug)]
pub struct MoveInfo {
    pub entity:           Entity,
    pub piece:            Piece,
    pub initial_position: Position,
    pub final_position:   Position,
    pub move_type:        MoveType
}
