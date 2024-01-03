use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_mod_picking::backends::raycast::RaycastPickable;
use bevy_panorbit_camera::PanOrbitCamera;

pub struct ChessCameraPlugin;
impl Plugin for ChessCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<SetCameraTargetAlpha>()
			.add_systems(Startup, spawn_camera)
			.add_systems(
				Update,
				check_event_set_camera_target_alpha
					.run_if(on_event::<SetCameraTargetAlpha>())
			);
	}
}

#[derive(Bundle)]
pub struct ChessCameraBundle {
	camera_3d: Camera3dBundle,
	pan_orbit: PanOrbitCamera,
	raycast:   RaycastPickable
}

impl Default for ChessCameraBundle {
	fn default() -> Self {
		Self {
			camera_3d: Camera3dBundle::default(),
			pan_orbit: PanOrbitCamera {
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
			raycast:   RaycastPickable::default()
		}
	}
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn(ChessCameraBundle::default());
}

#[derive(Event)]
pub struct SetCameraTargetAlpha {
	pub entity:       Entity,
	pub target_alpha: f32
}

fn check_event_set_camera_target_alpha(
	mut er_set_target_alpha: EventReader<SetCameraTargetAlpha>,
	mut query_camera: Query<&mut PanOrbitCamera>
) {
	for event in er_set_target_alpha.read() {
		let Ok(mut camera) = query_camera.get_mut(event.entity) else {
			return;
		};

		camera.target_alpha = event.target_alpha;
	}
}
