use bevy::prelude::*;

use crate::game::{position::Rank, resources::Theme};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TeamColor {
	White,
	Black
}

impl TeamColor {
	pub fn piece_color(&self, theme: &Res<Theme>) -> Color {
		use TeamColor::*;
		match self {
			White => theme.data().piece_white,
			Black => theme.data().piece_black
		}
	}

	pub fn promotion_rank(&self) -> &Rank {
		use TeamColor::*;
		match self {
			White => &Rank::EIGHT,
			Black => &Rank::ONE
		}
	}

	pub fn is_at_promotion_rank(&self, rank: &Rank) -> bool {
		self.promotion_rank() == rank
	}

	pub fn is_white(&self) -> bool { self == &TeamColor::White }

	pub fn is_black(&self) -> bool { self == &TeamColor::Black }
}
