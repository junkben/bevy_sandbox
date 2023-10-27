use bevy::prelude::*;

pub fn window_plugin() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			title: "TEST".to_string(),
			resolution: (1080.0, 720.0).into(),
			..default()
		}),
		..default()
	}
}
