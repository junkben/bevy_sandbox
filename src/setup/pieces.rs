use bevy::prelude::*;

use crate::resources::{board_state::BoardState, theme::Theme};

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
            piece.spawn(
                &mut commands,
                &asset_server,
                &mut materials,
                board_position,
                &theme
            );
        }
    }
}
