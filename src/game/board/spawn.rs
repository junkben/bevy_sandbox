use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{square::*, SquareSelectionBundle, UserSelectedSquare};
use crate::game::{position::Position, resources::Theme};

#[derive(Bundle)]
struct SquareBundle {
	square:           Square,
	pbr_bundle:       PbrBundle,
	position:         Position,
	selection_bundle: SquareSelectionBundle,
	on_pointer_click: On<Pointer<Click>>
}

impl SquareBundle {
	pub fn new(
		square: Square,
		pbr_bundle: PbrBundle,
		position: Position
	) -> SquareBundle {
		SquareBundle {
			square,
			pbr_bundle,
			position,
			selection_bundle: SquareSelectionBundle::default(),
			on_pointer_click: On::<Pointer<Click>>::send_event::<
				UserSelectedSquare
			>()
		}
	}
}

fn spawn_square(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	theme: &Res<Theme>,
	board_position: &Position
) {
	let square: Square = determine_square(&board_position);
	let pbr_bundle =
		square.pbr_bundle(meshes, materials, board_position, &theme);

	// Assemble the square bundle
	let square_bundle =
		SquareBundle::new(square, pbr_bundle, board_position.clone());

	// Spawn the square bundle
	commands.spawn(square_bundle);
}

pub fn spawn_board(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	theme: Res<Theme>
) {
	// Spawn all board squares
	for board_position in Position::iter() {
		spawn_square(
			&mut commands,
			&mut meshes,
			&mut materials,
			&theme,
			board_position
		)
	}
}

/// Change square color according to position to get alternating pattern
fn determine_square(board_position: &Position) -> Square {
	let (x, z) = board_position.xz();
	if (x + z) % 2 == 0 {
		WHITE_SQUARE
	} else {
		BLACK_SQUARE
	}
}
