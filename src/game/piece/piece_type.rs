use bevy::prelude::*;
use PieceType::*;

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
	pub const BISHOP: PieceType = Bishop;
	pub const KING: PieceType = King;
	pub const KNIGHT: PieceType = Knight;
	pub const PAWN: PieceType = Pawn;
	pub const QUEEN: PieceType = Queen;
	pub const ROOK: PieceType = Rook;

	pub const fn mesh_file_name(&self) -> &'static str {
		match self {
			King => "king.glb#Mesh0/Primitive0",
			Queen => "queen.glb#Mesh0/Primitive0",
			Rook => "rook.glb#Mesh0/Primitive0",
			Bishop => "bishop.glb#Mesh0/Primitive0",
			Knight => "knight.glb#Mesh0/Primitive0",
			Pawn => "pawn.glb#Mesh0/Primitive0"
		}
	}

	pub const fn mesh_translation_offset(&self) -> Vec3 {
		match self {
			King => Vec3::new(0., 0., 0.),
			Queen => Vec3::new(0., 0., 0.),
			Rook => Vec3::new(0., 0., 0.),
			Bishop => Vec3::new(0., 0., 0.),
			Knight => Vec3::new(0., 0., 0.),
			Pawn => Vec3::new(0., 0., 0.)
		}
	}
}
