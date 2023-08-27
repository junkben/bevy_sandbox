use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub struct ChessCameraPlugin;
impl Plugin for ChessCameraPlugin {
    fn build(&self, app: &mut App) { app.add_systems(Startup, spawn_camera); }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        PanOrbitCamera {
            focus: Vec3::new(4.0, 0.0, -4.0),
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Middle,
            modifier_pan: Some(KeyCode::ShiftLeft),
            ..default()
        },
        RaycastPickCamera::default()
    ));
}
