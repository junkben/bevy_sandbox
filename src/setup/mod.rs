use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use itertools::iproduct;

pub mod fen;
mod pieces;

pub use pieces::create_pieces;
use strum::IntoEnumIterator;

use crate::square::{File, Rank, CENTER_OF_BOARD};

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        ..default()
    }));
    let mat_white = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let mat_black = materials.add(Color::rgb(0., 0.1, 0.1).into());

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

pub fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 6.))
                .looking_at(CENTER_OF_BOARD, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default()
    ));
}

pub fn create_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, -4.0),
        ..default()
    });
}
