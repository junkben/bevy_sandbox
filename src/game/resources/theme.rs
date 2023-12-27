use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Default, Resource)]
pub enum Theme {
	BlackWhite,
	#[default]
	Classy
}

impl Theme {
	pub fn data(&self) -> ThemeData {
		use Theme::*;
		match self {
			BlackWhite => THEME_BLACK_WHITE,
			Classy => THEME_CLASSY
		}
	}
}

pub struct ThemeData {
	pub square_light: Color,
	pub square_dark:  Color,
	pub piece_white:  Color,
	pub piece_black:  Color
}

const THEME_BLACK_WHITE: ThemeData = ThemeData {
	square_light: Color::rgb(1.0, 0.9, 0.9),
	square_dark:  Color::rgb(0.0, 0.1, 0.1),
	piece_white:  Color::rgb(1.0, 0.9, 0.9),
	piece_black:  Color::rgb(0.0, 0.1, 0.1)
};

const THEME_CLASSY: ThemeData = ThemeData {
	square_light: Color::rgb(242.0 / 255.0, 229.0 / 255.0, 213.0 / 255.0),
	square_dark:  Color::rgb(217.0 / 255.0, 189.0 / 255.0, 156.0 / 255.0),
	piece_white:  Color::rgb(191.0 / 255.0, 146.0 / 255.0, 107.0 / 255.0),
	piece_black:  Color::rgb(64.0 / 255.0, 18.0 / 255.0, 1.0 / 255.0)
};
