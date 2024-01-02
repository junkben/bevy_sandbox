use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::{
	position::Position,
	resources::Theme,
	square::{spawn_square, Square, SquareColor}
};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) { app.add_systems(Startup, spawn_board); }
}

#[derive(Component, Debug, Clone, PartialEq)]
struct Board {
	pub squares: HashMap<Position, Square>
}

impl Board {
	pub fn new() -> Board {
		// Create all squares
		let squares = Position::iter()
			.map(|board_position| {
				let square = determine_square_color_by_position(board_position);
				(*board_position, square)
			})
			.collect::<HashMap<_, _>>();

		Board { squares }
	}

	pub fn spawn(
		&self,
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<StandardMaterial>>,
		theme: Res<Theme>
	) {
		// Spawn all board squares
		for (board_position, square) in self.squares.iter() {
			spawn_square(
				&mut commands,
				&mut meshes,
				&mut materials,
				&theme,
				square,
				board_position
			)
		}
	}
}

/// Change square color according to position to get alternating pattern
fn determine_square_color_by_position(board_position: &Position) -> Square {
	let (x, z) = board_position.xz();
	let square_color = match (x + z) % 2 == 0 {
		true => SquareColor::Light,
		false => SquareColor::Dark
	};
	Square { square_color }
}

/// Spawns a new board
pub fn spawn_board(
	commands: Commands,
	meshes: ResMut<Assets<Mesh>>,
	materials: ResMut<Assets<StandardMaterial>>,
	theme: Res<Theme>
) {
	Board::new().spawn(commands, meshes, materials, theme)
}
