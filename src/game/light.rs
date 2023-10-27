use bevy::prelude::*;

pub struct ChessLightPlugin;
impl Plugin for ChessLightPlugin {
	fn build(&self, app: &mut App) { app.add_systems(Startup, spawn_light); }
}

fn spawn_light(mut commands: Commands) {
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