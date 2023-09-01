use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    piece::{color::PieceColor, piece_type::PieceType, *},
    position::*
};

// pub const INITIAL_BOARD_STATE: [(BoardPosition, Option<Piece>); 64] = [
//    (A8, Some(BLACK_ROOK)),
//    (B8, Some(BLACK_KNIGHT)),
//    (C8, Some(BLACK_BISHOP)),
//    (D8, Some(BLACK_QUEEN)),
//    (E8, Some(BLACK_KING)),
//    (F8, Some(BLACK_BISHOP)),
//    (G8, Some(BLACK_KNIGHT)),
//    (H8, Some(BLACK_ROOK)),
//    (A7, Some(BLACK_PAWN)),
//    (B7, Some(BLACK_PAWN)),
//    (C7, Some(BLACK_PAWN)),
//    (D7, Some(BLACK_PAWN)),
//    (E7, Some(BLACK_PAWN)),
//    (F7, Some(BLACK_PAWN)),
//    (G7, Some(BLACK_PAWN)),
//    (H7, Some(BLACK_PAWN)),
//    (A6, None),
//    (B6, None),
//    (C6, None),
//    (D6, None),
//    (E6, None),
//    (F6, None),
//    (G6, None),
//    (H6, None),
//    (A5, None),
//    (B5, None),
//    (C5, None),
//    (D5, None),
//    (E5, None),
//    (F5, None),
//    (G5, None),
//    (H5, None),
//    (A4, None),
//    (B4, None),
//    (C4, None),
//    (D4, None),
//    (E4, None),
//    (F4, None),
//    (G4, None),
//    (H4, None),
//    (A3, None),
//    (B3, None),
//    (C3, None),
//    (D3, None),
//    (E3, None),
//    (F3, None),
//    (G3, None),
//    (H3, None),
//    (A2, Some(WHITE_PAWN)),
//    (B2, Some(WHITE_PAWN)),
//    (C2, Some(WHITE_PAWN)),
//    (D2, Some(WHITE_PAWN)),
//    (E2, Some(WHITE_PAWN)),
//    (F2, Some(WHITE_PAWN)),
//    (G2, Some(WHITE_PAWN)),
//    (H2, Some(WHITE_PAWN)),
//    (A1, Some(WHITE_ROOK)),
//    (B1, Some(WHITE_KNIGHT)),
//    (C1, Some(WHITE_BISHOP)),
//    (D1, Some(WHITE_QUEEN)),
//    (E1, Some(WHITE_KING)),
//    (F1, Some(WHITE_BISHOP)),
//    (G1, Some(WHITE_KNIGHT)),
//    (H1, Some(WHITE_ROOK))
//];

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
            // TODO! initial board state
            piece_placement_map:   HashMap::default(),
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

    pub fn get_piece_at(
        &self,
        board_position: BoardPosition
    ) -> Option<&Piece> {
        match self.piece_placement_map.get(&board_position) {
            Some(piece_opt) => piece_opt.as_ref(),
            None => panic!("no entry for position {board_position}")
        }
    }
}

/// A map corresponding a piece and its current position with all of its
/// possible new positions
#[derive(Resource, Default)]
pub struct AvailableMoves(
    pub HashMap<(Piece, BoardPosition), Vec<BoardPosition>>
);

impl AvailableMoves {
    pub fn new(
        pieces_and_positions: impl Iterator<Item = (Piece, BoardPosition)>
    ) -> AvailableMoves {
        let mut hashmap: HashMap<(Piece, BoardPosition), Vec<BoardPosition>> =
            HashMap::new();

        for (piece, start) in pieces_and_positions {
            let mut positions: Vec<BoardPosition> = Vec::new();

            for end in BoardPosition::iter() {
                if start == end {
                    continue;
                }

                positions.push(end);
            }

            // match piece.piece_type {
            //    King => todo!(),
            //    Queen => todo!(),
            //    Rook => todo!(),
            //    Bishop => todo!(),
            //    Knight => todo!(),
            //    Pawn => todo!()
            //}

            hashmap.insert((piece, start), positions);
        }

        AvailableMoves(hashmap)
    }
}

fn moves_rook(start: &'static BoardPosition) -> Vec<BoardPosition> {
    let (f, r) = (start.file(), start.rank());
    let vertical = BoardPosition::iter().filter(|bp| bp.file() == f);
    let horizontal = BoardPosition::iter().filter(|bp| bp.rank() == r);

    let mut moves: Vec<BoardPosition> =
        vertical.chain(horizontal).collect::<Vec<_>>();
    moves.retain(|position| position != start);
    moves
}

const KNIGHT_VECTORS: [(isize, isize); 8] = [
    (1, 2),
    (2, 1),
    (1, -2),
    (2, -1),
    (-1, 2),
    (-2, 1),
    (-1, -2),
    (-2, -1)
];

const BISHOP_VECTORS: [(isize, isize); 28] = [
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (5, 5),
    (6, 6),
    (7, 7),
    (1, -1),
    (2, -2),
    (3, -3),
    (4, -4),
    (5, -5),
    (6, -6),
    (7, -7),
    (-1, 1),
    (-2, 2),
    (-3, 3),
    (-4, 4),
    (-5, 5),
    (-6, 6),
    (-7, 7),
    (-1, -1),
    (-2, -2),
    (-3, -3),
    (-4, -4),
    (-5, -5),
    (-6, -6),
    (-7, -7)
];

const ROOK_CASTLE_KINGSIDE: (isize, isize) = (-2, 0);
const ROOK_CASTLE_QUEENSIDE: (isize, isize) = (3, 0);

const KING_VECTORS: [(isize, isize); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1)
];

const KING_CASTLE_KINGSIDE: (isize, isize) = (2, 0);
const KING_CASTLE_QUEENSIDE: (isize, isize) = (-2, 0);

const PAWN_WHITE_VEC: (isize, isize) = (0, 1);
const PAWN_WHITE_FIRST_VEC: (isize, isize) = (0, 2);
const PAWN_WHITE_CAPTURE_VECTORS: [(isize, isize); 2] = [(1, 1), (-1, 1)];

const PAWN_BLACK_VEC: (isize, isize) = (0, -1);
const PAWN_BLACK_FIRST_VEC: (isize, isize) = (0, -2);
const PAWN_BLACK_CAPTURE_VECTORS: [(isize, isize); 2] = [(1, -1), (-1, -1)];
