use bevy::prelude::*;

use super::{available_moves::AvailableMoves, Piece};
use crate::{move_tracker::MoveTracker, position::Position, resources::Theme};

pub fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>
) {
    // Spawn pieces in proper squares
    for (position, piece) in INITIAL_PIECE_POSITIONS.iter() {
        let pbr_bundle =
            piece.pbr_bundle(&asset_server, &mut materials, position, &theme);

        let piece_bundle = super::PieceBundle {
            pbr_bundle,
            board_position: position.clone(),
            piece: piece.clone(),
            move_tracker: MoveTracker::default(),
            available_moves: AvailableMoves::default()
        };
        commands.spawn(piece_bundle);
    }
}

const INITIAL_PIECE_POSITIONS: [(Position, Piece); 32] = [
    (Position::A8, Piece::BLACK_ROOK),
    (Position::B8, Piece::BLACK_KNIGHT),
    (Position::C8, Piece::BLACK_BISHOP),
    (Position::D8, Piece::BLACK_QUEEN),
    (Position::E8, Piece::BLACK_KING),
    (Position::F8, Piece::BLACK_BISHOP),
    (Position::G8, Piece::BLACK_KNIGHT),
    (Position::H8, Piece::BLACK_ROOK),
    (Position::A7, Piece::BLACK_PAWN),
    (Position::B7, Piece::BLACK_PAWN),
    (Position::C7, Piece::BLACK_PAWN),
    (Position::D7, Piece::BLACK_PAWN),
    (Position::E7, Piece::BLACK_PAWN),
    (Position::F7, Piece::BLACK_PAWN),
    (Position::G7, Piece::BLACK_PAWN),
    (Position::H7, Piece::BLACK_PAWN),
    (Position::A2, Piece::WHITE_PAWN),
    (Position::B2, Piece::WHITE_PAWN),
    (Position::C2, Piece::WHITE_PAWN),
    (Position::D2, Piece::WHITE_PAWN),
    (Position::E2, Piece::WHITE_PAWN),
    (Position::F2, Piece::WHITE_PAWN),
    (Position::G2, Piece::WHITE_PAWN),
    (Position::H2, Piece::WHITE_PAWN),
    (Position::A1, Piece::WHITE_ROOK),
    (Position::B1, Piece::WHITE_KNIGHT),
    (Position::C1, Piece::WHITE_BISHOP),
    (Position::D1, Piece::WHITE_QUEEN),
    (Position::E1, Piece::WHITE_KING),
    (Position::F1, Piece::WHITE_BISHOP),
    (Position::G1, Piece::WHITE_KNIGHT),
    (Position::H1, Piece::WHITE_ROOK)
];
