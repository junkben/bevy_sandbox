use bevy::prelude::*;

use crate::game::resources::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SquareColor {
	Light,
	Dark
}

impl SquareColor {
	pub fn color(&self, theme: &Res<Theme>) -> Color {
		use SquareColor::*;
		match self {
			Light => theme.data().square_light,
			Dark => theme.data().square_dark
		}
	}
}
