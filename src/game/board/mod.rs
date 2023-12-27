mod spawn;

use bevy::prelude::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn::spawn_board);
	}
}
