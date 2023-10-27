use bevy::prelude::*;

use crate::game::piece::PieceColor;

/// Tracks whose turn it is. White always goes first.
#[derive(Resource)]
pub struct ActiveColor(pub PieceColor);

impl ActiveColor {
	pub fn next(&mut self) {
		use PieceColor::*;
		self.0 = match self.0 {
			White => Black,
			Black => White
		}
	}
}

impl Default for ActiveColor {
	fn default() -> Self { Self(PieceColor::White) }
}
