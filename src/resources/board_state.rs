use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    board::position::*,
    piece::{color::PieceColor, *}
};

pub const INITIAL_BOARD_STATE: [(BoardPosition, Option<Piece>); 64] = [
    (A8, Some(BLACK_ROOK)),
    (B8, Some(BLACK_KNIGHT)),
    (C8, Some(BLACK_BISHOP)),
    (D8, Some(BLACK_QUEEN)),
    (E8, Some(BLACK_KING)),
    (F8, Some(BLACK_BISHOP)),
    (G8, Some(BLACK_KNIGHT)),
    (H8, Some(BLACK_ROOK)),
    (A7, Some(BLACK_PAWN)),
    (B7, Some(BLACK_PAWN)),
    (C7, Some(BLACK_PAWN)),
    (D7, Some(BLACK_PAWN)),
    (E7, Some(BLACK_PAWN)),
    (F7, Some(BLACK_PAWN)),
    (G7, Some(BLACK_PAWN)),
    (H7, Some(BLACK_PAWN)),
    (A6, None),
    (B6, None),
    (C6, None),
    (D6, None),
    (E6, None),
    (F6, None),
    (G6, None),
    (H6, None),
    (A5, None),
    (B5, None),
    (C5, None),
    (D5, None),
    (E5, None),
    (F5, None),
    (G5, None),
    (H5, None),
    (A4, None),
    (B4, None),
    (C4, None),
    (D4, None),
    (E4, None),
    (F4, None),
    (G4, None),
    (H4, None),
    (A3, None),
    (B3, None),
    (C3, None),
    (D3, None),
    (E3, None),
    (F3, None),
    (G3, None),
    (H3, None),
    (A2, Some(WHITE_PAWN)),
    (B2, Some(WHITE_PAWN)),
    (C2, Some(WHITE_PAWN)),
    (D2, Some(WHITE_PAWN)),
    (E2, Some(WHITE_PAWN)),
    (F2, Some(WHITE_PAWN)),
    (G2, Some(WHITE_PAWN)),
    (H2, Some(WHITE_PAWN)),
    (A1, Some(WHITE_ROOK)),
    (B1, Some(WHITE_KNIGHT)),
    (C1, Some(WHITE_BISHOP)),
    (D1, Some(WHITE_QUEEN)),
    (E1, Some(WHITE_KING)),
    (F1, Some(WHITE_BISHOP)),
    (G1, Some(WHITE_KNIGHT)),
    (H1, Some(WHITE_ROOK))
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
    pub piece_placement_map:   HashMap<BoardPosition, Option<Piece>>,
    /// Tracks whose turn it is. White always goes first.
    pub active_color:          PieceColor,
    /// Tracks what players can castle and to what side
    pub castling_availability: CastlingAvailability,
    /// Tracks whether or not there's a target for an en passant capture
    pub en_passant_target:     Option<BoardPosition>,
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

impl BoardState {
    pub fn next_turn(&mut self) {
        // If Black just moved, then we've completed one turn rotation
        if self.active_color == PieceColor::Black {
            self.completed_turns += 1;
        }

        // Every turn advances the halfmove clock
        self.halfmove_clock += 1;

        // Switch active color
        use PieceColor::*;
        self.active_color = match self.active_color {
            White => Black,
            Black => White
        };

        debug!(?self)
    }
}
