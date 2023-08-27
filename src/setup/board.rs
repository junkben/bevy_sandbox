use bevy::prelude::*;
use itertools::iproduct;
use strum::IntoEnumIterator;

use crate::{
    resources::theme::Theme,
    square::{File, Rank}
};

pub fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        ..default()
    }));
    let mat_white = materials.add(theme.data().square_white.into());
    let mat_black = materials.add(theme.data().square_black.into());

    // Spawn 64 squares
    for (r, f) in iproduct!(Rank::iter(), File::iter()) {
        let position = Vec3::from(r) + Vec3::from(f);

        // Change material according to position to get alternating
        // pattern
        let material: Handle<StandardMaterial> =
            if (position.x as i32 + position.z as i32) % 2 == 0 {
                mat_white.clone()
            } else {
                mat_black.clone()
            };

        // Spawn the chess square
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material,
            transform: Transform::from_translation(position),
            ..default()
        });
    }
}
