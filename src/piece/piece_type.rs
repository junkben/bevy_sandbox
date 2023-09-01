use bevy::prelude::*;
use PieceType::*;

pub const KING: PieceType = King;
pub const QUEEN: PieceType = Queen;
pub const ROOK: PieceType = Rook;
pub const BISHOP: PieceType = Bishop;
pub const KNIGHT: PieceType = Knight;
pub const PAWN: PieceType = Pawn;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl PieceType {
    pub fn mesh_file_name(&self) -> &'static str {
        match self {
            King => "king.glb#Mesh0/Primitive0",
            Queen => "queen.glb#Mesh0/Primitive0",
            Rook => "rook.glb#Mesh0/Primitive0",
            Bishop => "bishop.glb#Mesh0/Primitive0",
            Knight => "knight.glb#Mesh0/Primitive0",
            Pawn => "pawn.glb#Mesh0/Primitive0"
        }
    }

    pub fn mesh_offset(&self) -> Vec3 {
        let (x, y, z): (f32, f32, f32) = match self {
            King => (0., 0., 0.),
            Queen => (0., 0., 0.),
            Rook => (0., 0., 0.),
            Bishop => (0., 0., 0.),
            Knight => (0., 0., 0.),
            Pawn => (0., 0., 0.)
        };
        Vec3 { x, y, z }
    }
}
