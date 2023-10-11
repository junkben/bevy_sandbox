use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
	pub should_rotate_camera: bool,
	pub two_dimensional_mode: bool,
	pub drap_and_drop:        bool
}

impl Default for GameSettings {
	fn default() -> Self {
		Self {
			should_rotate_camera: false,
			two_dimensional_mode: false,
			drap_and_drop:        false
		}
	}
}
