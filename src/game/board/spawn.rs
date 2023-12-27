use bevy::prelude::*;

use crate::game::{
	position::Position,
	resources::Theme,
	square::{spawn_square, SquareColor}
};

pub fn spawn_board(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	theme: Res<Theme>
) {
	// Spawn all board squares
	for board_position in Position::iter() {
		let square_color = determine_square_color(board_position);
		spawn_square(
			&mut commands,
			&mut meshes,
			&mut materials,
			&theme,
			&square_color,
			&board_position
		)
	}
}

/// Change square color according to position to get alternating pattern
fn determine_square_color(board_position: &Position) -> SquareColor {
	let (x, z) = board_position.xz();
	match (x + z) % 2 == 0 {
		true => SquareColor::Light,
		false => SquareColor::Dark
	}
}
