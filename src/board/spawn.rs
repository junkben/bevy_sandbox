use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::prelude::*;

use super::{square::*, SquareBundle};
use crate::{
    board::position::{BoardPosition, BOARD_POSITIONS},
    resources::Theme
};

fn spawn_square(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    theme: &Res<Theme>,
    board_position: BoardPosition
) {
    let square: Square = determine_square(&board_position);
    let pbr_bundle =
        square.pbr_bundle(meshes, materials, &board_position, &theme);

    // Assemble the square bundle
    let square_bundle = SquareBundle::new(square, pbr_bundle, board_position);
    let highlight = highlight();

    // Spawn the square bundle
    commands.spawn((square_bundle, highlight));
}

pub fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>
) {
    // Spawn all board squares
    for board_position in BOARD_POSITIONS {
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            &theme,
            board_position
        )
    }
}

/// Change square color according to position to get alternating pattern
fn determine_square(board_position: &BoardPosition) -> Square {
    let position_vec = board_position.vec3();
    if (position_vec.x as i32 + position_vec.z as i32) % 2 == 0 {
        WHITE_SQUARE
    } else {
        BLACK_SQUARE
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
