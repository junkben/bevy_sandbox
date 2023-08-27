use std::collections::HashMap;

use bevy::prelude::*;

use crate::{chess_piece::*, square::*};

pub const INITIAL_BOARD_STATE: [(SquareId, Option<ChessPiece>); 64] = [
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

pub struct EnPassantTarget {
    pub square: Option<SquareId>
}

impl std::fmt::Display for EnPassantTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.square {
            Some(id) => write!(f, "{id}"),
            None => write!(f, "-")
        }
    }
}

#[derive(Resource)]
pub struct BoardState {
    pub piece_placement_map:   HashMap<SquareId, Option<ChessPiece>>,
    pub active_color:          PColor,
    pub castling_availability: CastlingAvailability,
    pub en_passant_target:     EnPassantTarget,
    pub halfmove_clock:        u32,
    pub fullmove_num:          u32
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            piece_placement_map:   HashMap::from(INITIAL_BOARD_STATE),
            active_color:          PColor::White,
            castling_availability: CastlingAvailability::default(),
            en_passant_target:     EnPassantTarget { square: None },
            halfmove_clock:        0,
            fullmove_num:          1
        }
    }
}
