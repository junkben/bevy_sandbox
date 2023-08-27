use bevy::prelude::*;

use crate::resources::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SquareColor {
    White,
    Black
}

impl SquareColor {
    pub fn color(&self, theme: &Res<Theme>) -> Color {
        use SquareColor::*;
        match self {
            White => theme.data().square_white,
            Black => theme.data().square_black
        }
    }
}
