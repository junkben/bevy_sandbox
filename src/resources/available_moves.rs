use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    piece::{color::PieceColor, movement::*, piece_type::PieceType, *},
    position::*
};

/// A map corresponding a piece and its current position with all of its
/// possible new positions
#[derive(Resource, Default)]
pub struct AvailableMoves(pub HashMap<(Piece, Position), Vec<Position>>);

impl std::fmt::Display for AvailableMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.list())
    }
}

impl AvailableMoves {
    pub fn list(&self) -> String {
        let mut strings: Vec<String> = vec![];
        for ((piece, _start), moves) in self.0.clone() {
            for m in moves {
                strings.push(format!("{piece}{m}"));
            }
        }
        strings.join(", ")
    }

    pub fn moves_from(
        &self,
        piece: Piece,
        position: Position
    ) -> Option<&Vec<Position>> {
        self.0.get(&(piece, position))
    }

    pub fn recalculate(
        &mut self,
        piece_map: &HashMap<Position, Option<Piece>>,
        active_color: &PieceColor
    ) {
        self.0 = HashMap::new();
        debug!("1");

        for (start, piece_opt) in piece_map.clone() {
            if piece_opt.is_none() {
                continue;
            }

            let piece = piece_opt.unwrap();

            if piece.piece_color() != active_color {
                continue;
            }

            use PieceColor::*;
            use PieceType::*;
            let movement_pattern =
                match (piece.piece_color(), piece.piece_type()) {
                    (_, King) => PieceMovementBehavior::KING,
                    (_, Queen) => PieceMovementBehavior::QUEEN,
                    (_, Rook) => PieceMovementBehavior::ROOK,
                    (_, Bishop) => PieceMovementBehavior::BISHOP,
                    (_, Knight) => PieceMovementBehavior::KNIGHT,
                    (White, Pawn) => PieceMovementBehavior::PAWN_WHITE,
                    (Black, Pawn) => PieceMovementBehavior::PAWN_BLACK
                };
            let (start_x, start_z) = start.xz();
            let start_vec = Vec3::new(start_x as f32, 0.0, start_z as f32);

            let mut moves: Vec<Position> = Vec::new();

            for direction in movement_pattern.directions() {
                let mut l: u8 = 1u8;

                while l <= movement_pattern.length() {
                    let vector = start_vec + (direction.clone() * l as f32);

                    // If the proposed Position can't exist, break
                    let new_move = match Position::try_from_vec3(vector) {
                        Some(bp) => bp,
                        None => break
                    };

                    // Check if there is a piece at the end position
                    if let Some(other_piece) = piece_map.get(&new_move).unwrap()
                    {
                        // We're breaking as we can't possibly go past this
                        // piece, but if the piece is of
                        // the opposite color, then we can still
                        // move there. If it's our piece, then we can't
                        // move there or past it.
                        if other_piece.piece_color() != piece.piece_color() {
                            moves.push(new_move);
                        }
                        break;
                    }

                    // Move is to a legitimate position and there's no piece in
                    // the way
                    moves.push(new_move);
                    l += 1;
                }
            }

            if moves.is_empty() {
                continue;
            }

            self.0.insert((piece, start), moves);
        }
    }
}
