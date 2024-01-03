use bevy::prelude::*;

use self::kelvin::KELVIN_TABLE;

mod kelvin;

pub struct ChessLightPlugin;
impl Plugin for ChessLightPlugin {
	fn build(&self, app: &mut App) { app.add_systems(Startup, spawn_light); }
}

fn spawn_light(mut commands: Commands) {
	let (r, g, b) = KELVIN_TABLE.get(&5000).unwrap().clone();
	let color = Color::rgb_u8(r, g, b);

	commands.spawn(PointLightBundle {
		point_light: PointLight {
			color,
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, -4.0),
		..default()
	});
}
