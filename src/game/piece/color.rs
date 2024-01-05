use bevy::prelude::*;
use PieceColor::*;

use crate::game::resources::Theme;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceColor {
	White,
	Black
}

impl PieceColor {
	pub const BLACK: PieceColor = Black;
	pub const WHITE: PieceColor = White;

	pub fn color(&self, theme: &Res<Theme>) -> Color {
		match self {
			White => theme.data().piece_white,
			Black => theme.data().piece_black
		}
	}

	pub fn is_white(&self) -> bool { self == &PieceColor::White }
}
