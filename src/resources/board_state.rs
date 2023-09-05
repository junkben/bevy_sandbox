use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    piece::{color::PieceColor, Piece},
    position::Position
};

pub const INITIAL_BOARD_STATE: [(Position, Option<Piece>); 64] = [
    (Position::A8, Some(Piece::BLACK_ROOK)),
    (Position::B8, Some(Piece::BLACK_KNIGHT)),
    (Position::C8, Some(Piece::BLACK_BISHOP)),
    (Position::D8, Some(Piece::BLACK_QUEEN)),
    (Position::E8, Some(Piece::BLACK_KING)),
    (Position::F8, Some(Piece::BLACK_BISHOP)),
    (Position::G8, Some(Piece::BLACK_KNIGHT)),
    (Position::H8, Some(Piece::BLACK_ROOK)),
    (Position::A7, Some(Piece::BLACK_PAWN)),
    (Position::B7, Some(Piece::BLACK_PAWN)),
    (Position::C7, Some(Piece::BLACK_PAWN)),
    (Position::D7, Some(Piece::BLACK_PAWN)),
    (Position::E7, Some(Piece::BLACK_PAWN)),
    (Position::F7, Some(Piece::BLACK_PAWN)),
    (Position::G7, Some(Piece::BLACK_PAWN)),
    (Position::H7, Some(Piece::BLACK_PAWN)),
    (Position::A6, None),
    (Position::B6, None),
    (Position::C6, None),
    (Position::D6, None),
    (Position::E6, None),
    (Position::F6, None),
    (Position::G6, None),
    (Position::H6, None),
    (Position::A5, None),
    (Position::B5, None),
    (Position::C5, None),
    (Position::D5, None),
    (Position::E5, None),
    (Position::F5, None),
    (Position::G5, None),
    (Position::H5, None),
    (Position::A4, None),
    (Position::B4, None),
    (Position::C4, None),
    (Position::D4, None),
    (Position::E4, None),
    (Position::F4, None),
    (Position::G4, None),
    (Position::H4, None),
    (Position::A3, None),
    (Position::B3, None),
    (Position::C3, None),
    (Position::D3, None),
    (Position::E3, None),
    (Position::F3, None),
    (Position::G3, None),
    (Position::H3, None),
    (Position::A2, Some(Piece::WHITE_PAWN)),
    (Position::B2, Some(Piece::WHITE_PAWN)),
    (Position::C2, Some(Piece::WHITE_PAWN)),
    (Position::D2, Some(Piece::WHITE_PAWN)),
    (Position::E2, Some(Piece::WHITE_PAWN)),
    (Position::F2, Some(Piece::WHITE_PAWN)),
    (Position::G2, Some(Piece::WHITE_PAWN)),
    (Position::H2, Some(Piece::WHITE_PAWN)),
    (Position::A1, Some(Piece::WHITE_ROOK)),
    (Position::B1, Some(Piece::WHITE_KNIGHT)),
    (Position::C1, Some(Piece::WHITE_BISHOP)),
    (Position::D1, Some(Piece::WHITE_QUEEN)),
    (Position::E1, Some(Piece::WHITE_KING)),
    (Position::F1, Some(Piece::WHITE_BISHOP)),
    (Position::G1, Some(Piece::WHITE_KNIGHT)),
    (Position::H1, Some(Piece::WHITE_ROOK))
];

#[derive(Debug)]
pub struct CastlingAvailability {
    pub white_kingside:  bool,
    pub white_queenside: bool,
    pub black_kingside:  bool,
    pub black_queenside: bool
}

impl std::fmt::Display for CastlingAvailability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wk = if self.white_kingside { "K" } else { "" };
        let wq = if self.white_queenside { "Q" } else { "" };
        let bk = if self.black_kingside { "k" } else { "" };
        let bq = if self.black_queenside { "q" } else { "" };
        write!(f, "{wk}{wq}{bk}{bq}")
    }
}

impl Default for CastlingAvailability {
    fn default() -> Self {
        Self {
            white_kingside:  true,
            white_queenside: true,
            black_kingside:  true,
            black_queenside: true
        }
    }
}

#[derive(Resource, Debug)]
pub struct BoardState {
    pub piece_placement_map:   HashMap<Position, Option<Piece>>,
    /// Tracks whose turn it is. White always goes first.
    pub active_color:          PieceColor,
    /// Tracks what players can castle and to what side
    pub castling_availability: CastlingAvailability,
    /// Tracks whether or not there's a target for an en passant capture
    pub en_passant_target:     Option<Position>,
    /// The number of moves both players have made since the last pawn advance
    /// or piece capture. Used to enforce the 50-move draw rule, where the
    /// game ends in a draw after 100 half-moves
    pub halfmove_clock:        u32,
    /// Number is incremented by one every time Black moves, i.e. a "fullmove"
    pub completed_turns:       u32
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            piece_placement_map:   HashMap::from(INITIAL_BOARD_STATE),
            active_color:          PieceColor::White,
            castling_availability: CastlingAvailability::default(),
            en_passant_target:     None,
            halfmove_clock:        0,
            completed_turns:       1
        }
    }
}

impl std::fmt::Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl BoardState {
    pub fn get_piece_at(&self, board_position: Position) -> Option<&Piece> {
        match self.piece_placement_map.get(&board_position) {
            Some(piece_opt) => piece_opt.as_ref(),
            None => panic!("no entry for position {board_position}")
        }
    }
}
