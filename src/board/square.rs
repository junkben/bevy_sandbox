use bevy::prelude::*;

use super::{color::SquareColor, position::BoardPosition};
use crate::resources::theme::Theme;

pub const WHITE_SQUARE: Square = Square {
    color: SquareColor::White
};
pub const BLACK_SQUARE: Square = Square {
    color: SquareColor::Black
};

const SQUARE_SIZE: f32 = 1.0;
const SCALE: Vec3 = Vec3::ONE;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub color: SquareColor
}

impl Square {
    fn mesh_handle(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        size: f32
    ) -> Handle<Mesh> {
        meshes.add(Mesh::from(shape::Plane { size, ..default() }))
    }

    fn material_handle(
        &self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        theme: &Res<Theme>
    ) -> Handle<StandardMaterial> {
        let base_color = self.color.color(theme);
        materials.add(StandardMaterial {
            base_color,
            ..default()
        })
    }

    fn mesh_offset(&self) -> Vec3 { Vec3::ZERO }

    fn position(&self, board_position: BoardPosition) -> Vec3 {
        self.mesh_offset() + board_position.vec3()
    }

    pub fn pbr_bundle(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        board_position: &BoardPosition,
        theme: &Res<Theme>
    ) -> PbrBundle {
        let size = SQUARE_SIZE;
        let mesh = self.mesh_handle(meshes, size);
        let material = self.material_handle(materials, theme);
        let translation = self.position(board_position.clone());
        let transform =
            Transform::from_translation(translation).with_scale(SCALE);

        PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        }
    }
}
