use bevy::prelude::*;
use PieceColor::*;

use crate::resources::Theme;

pub const WHITE: PieceColor = White;
pub const BLACK: PieceColor = Black;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceColor {
    White,
    Black
}

impl PieceColor {
    pub fn color(&self, theme: &Res<Theme>) -> Color {
        match self {
            White => theme.data().piece_white,
            Black => theme.data().piece_black
        }
    }
}
