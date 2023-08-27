use bevy::prelude::*;

use crate::resources::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

impl std::fmt::Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PieceColor::*;
        write!(f, "{}", match self {
            White => "w",
            Black => "b"
        })
    }
}

impl PieceColor {
    pub fn color(&self, theme: &Res<Theme>) -> Color {
        use PieceColor::*;
        match self {
            White => theme.data().piece_white,
            Black => theme.data().piece_black
        }
    }
}
