use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::square::CENTER_OF_BOARD;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 6.))
                .looking_at(CENTER_OF_BOARD, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default()
    ));
}
