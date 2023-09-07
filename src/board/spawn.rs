use bevy::prelude::*;

use super::{square::*, SquareBundle};
use crate::{position::Position, resources::Theme};

fn spawn_square(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    theme: &Res<Theme>,
    board_position: &Position
) {
    let square: Square = determine_square(&board_position);
    let pbr_bundle =
        square.pbr_bundle(meshes, materials, board_position, &theme);

    // Assemble the square bundle
    let square_bundle =
        SquareBundle::new(square, pbr_bundle, board_position.clone());

    // Spawn the square bundle
    commands.spawn(square_bundle);
}

pub fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>
) {
    // Spawn all board squares
    for board_position in Position::iter() {
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
fn determine_square(board_position: &Position) -> Square {
    let (x, z) = board_position.xz();
    debug!(?x, ?z);
    if (x + z) % 2 == 0 {
        WHITE_SQUARE
    } else {
        BLACK_SQUARE
    }
}
