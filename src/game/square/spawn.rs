use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{Square, SquareColor, SquareSelectionBundle, UserSelectedSquare};
use crate::game::{position::Position, resources::Theme};

#[derive(Bundle)]
pub struct SquareBundle {
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

pub fn spawn_square(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	theme: &Res<Theme>,
	square: &Square,
	board_position: &Position
) {
	let pbr_bundle =
		square.pbr_bundle(meshes, materials, board_position, &theme);

	// Assemble the square bundle
	let square_bundle =
		SquareBundle::new(*square, pbr_bundle, board_position.clone());

	// Spawn the square bundle
	commands.spawn(square_bundle);
}
