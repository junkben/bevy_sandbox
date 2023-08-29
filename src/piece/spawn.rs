use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::resources::{BoardState, Theme};

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

            let piece_bundle = super::PieceBundle {
                pbr_bundle,
                board_position: board_position.clone(),
                piece: piece.clone(),
                pickable_bundle: PickableBundle::default(),
                raycast_pick_target: RaycastPickTarget::default()
            };
            commands.spawn((piece_bundle, highlight()));
        }
    }
}

fn highlight() -> Highlight<StandardMaterial> {
    Highlight {
        hovered:  Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.2, -0.2, 0.4, 0.0),
            ..matl.to_owned()
        })),
        pressed:  Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.3, -0.3, 0.5, 0.0),
            ..matl.to_owned()
        })),
        selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.3, 0.2, -0.3, 0.0),
            ..matl.to_owned()
        }))
    }
}
