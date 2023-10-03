use bevy::prelude::*;

use crate::{
    piece::{Piece, PieceType},
    position::Position
};

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
        is_en_passant: bool,
        captured:      Entity
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

#[derive(Debug, Copy, Clone)]
pub struct MoveInfo {
    pub entity:           Entity,
    pub piece:            Piece,
    pub initial_position: Position,
    pub final_position:   Position,
    pub move_type:        MoveType
}

impl std::fmt::Display for MoveInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.move_type {
            MoveType::Move => match self.piece.piece_type() {
                PieceType::Pawn => format!("{}", self.final_position),
                _ => format!("{}{}", self.piece, self.final_position)
            },
            MoveType::Capture { .. } => {
                format!("{}x{}", self.piece, self.final_position)
            },
            _ => todo!()
        })
    }
}
