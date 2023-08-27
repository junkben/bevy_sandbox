use std::f32::consts::TAU;

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
            // Set focal point
            focus: Vec3::new(4.0, 0.0, -4.0),
            // Set starting position, relative to focus
            alpha: Some(0.0),
            beta: Some(TAU / 8.0),
            radius: Some(12.0),
            // Set limits on rotation
            alpha_upper_limit: None,
            alpha_lower_limit: None,
            beta_upper_limit: Some(TAU / 8.0),
            beta_lower_limit: Some(TAU / 8.0),
            zoom_upper_limit: Some(20.0),
            zoom_lower_limit: Some(4.0),
            // Change the controls to match blender
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Middle,
            modifier_pan: Some(KeyCode::ShiftLeft),
            ..default()
        },
        RaycastPickCamera::default()
    ));
}
