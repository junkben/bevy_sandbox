use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    piece::Piece,
    resources::{board_state::BoardState, theme::Theme}
};

#[derive(Bundle)]
pub struct PieceBundle {
    pbr_bundle:     PbrBundle,
    board_position: BoardPosition,
    piece:          Piece
}

pub fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_state: Res<BoardState>,
    theme: Res<Theme>
) {
    // Spawn pieces in proper squares
    for (board_position, piece_opt) in &board_state.piece_placement_map {
        if let Some(piece) = piece_opt {
            let pbr_bundle = piece.pbr_bundle(
                &asset_server,
                &mut materials,
                board_position,
                &theme
            );

            let piece_bundle = PieceBundle {
                pbr_bundle,
                board_position: board_position.clone(),
                piece: piece.clone()
            };
            commands.spawn(piece_bundle);
        }
    }
}
