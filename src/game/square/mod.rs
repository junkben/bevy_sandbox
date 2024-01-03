mod color;
mod selection;
mod spawn;

use bevy::prelude::*;
pub use color::SquareColor;
pub use selection::{
	SquareSelectPlugin, SquareSelectionBundle, UserSelectedSquare
};
pub use spawn::{spawn_square, SquareBundle};

use crate::game::{position::Position, resources::Theme};

pub struct SquarePlugin;
impl Plugin for SquarePlugin {
	fn build(&self, app: &mut App) { app.add_plugins(SquareSelectPlugin); }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Square {
	pub square_color: SquareColor
}

impl Square {
	const ROUGHNESS: f32 = 1.0;
	const SCALE: Vec3 = Vec3::ONE;
	const SIZE: f32 = 1.0;

	fn mesh_handle(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
		meshes.add(Mesh::from(shape::Cube {
			size: Square::SIZE,
			..default()
		}))
	}

	fn material_handle(
		&self,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		theme: &Res<Theme>
	) -> Handle<StandardMaterial> {
		let base_color = self.square_color.color(theme);
		materials.add(StandardMaterial {
			base_color,
			perceptual_roughness: Square::ROUGHNESS,
			..default()
		})
	}

	fn mesh_translation_offset(&self) -> Vec3 {
		Vec3::new(0.0, -Square::SIZE / 2.0, 0.0)
	}

	fn translation(&self, board_position: Position) -> Vec3 {
		self.mesh_translation_offset() + board_position.translation()
	}

	pub fn pbr_bundle(
		&self,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		board_position: &Position,
		theme: &Res<Theme>
	) -> PbrBundle {
		let mesh = self.mesh_handle(meshes);
		let material = self.material_handle(materials, theme);
		let translation = self.translation(board_position.clone());
		let transform =
			Transform::from_translation(translation).with_scale(Square::SCALE);

		PbrBundle {
			mesh,
			material,
			transform,
			..default()
		}
	}
}
