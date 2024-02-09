mod movement;
mod piece_type;
mod selection;
mod spawn;

use bevy::prelude::*;
pub use movement::{
	MovePieceToBoardPosition, MovementType, PieceMovementBehavior
};
pub use piece_type::PieceType;
pub use selection::{PieceSelectionBundle, UserSelectedPiece};
pub use spawn::{
	PieceCaptured, SpawnPieces, SpawnPiecesDone, INITIAL_PIECE_POSITIONS
};

use self::{
	movement::PieceMovementPlugin, selection::PieceSelectPlugin,
	spawn::SpawnPiecePlugin
};
use super::team::TeamColor;
use crate::game::{position::Position, resources::Theme};

macro_rules! chess_pieces {
	($($name:ident, $color:ident, $piece_type:ident);*) => {
		$(
			pub const $name: Piece = Piece {
				color:	  TeamColor::$color,
				piece_type: PieceType::$piece_type
			};
		)*
	};
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			SpawnPiecePlugin,
			PieceSelectPlugin,
			PieceMovementPlugin
		));
	}
}

/// A component that represents a chess piece
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
	color:      TeamColor,
	piece_type: PieceType
}

impl std::fmt::Display for Piece {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use PieceType::*;
		write!(f, "{}", match &self.piece_type {
			King => 'K',
			Queen => 'Q',
			Rook => 'R',
			Bishop => 'B',
			Knight => 'N',
			Pawn => 'P'
		})
	}
}

#[allow(dead_code)]
impl Piece {
	const MESH_PATH: &'static str = "models/chess/";
	const SCALE: Vec3 = Vec3 {
		x: 0.012,
		y: 0.012,
		z: 0.012
	};

	chess_pieces!(
		WHITE_KING, White, KING;
		WHITE_QUEEN, White, QUEEN;
		WHITE_ROOK, White, ROOK;
		WHITE_BISHOP, White, BISHOP;
		WHITE_KNIGHT, White, KNIGHT;
		WHITE_PAWN, White, PAWN;
		BLACK_KING, Black, KING;
		BLACK_QUEEN, Black, QUEEN;
		BLACK_ROOK, Black, ROOK;
		BLACK_BISHOP, Black, BISHOP;
		BLACK_KNIGHT, Black, KNIGHT;
		BLACK_PAWN, Black, PAWN
	);

	pub fn new(color: TeamColor, piece_type: PieceType) -> Piece {
		Piece { color, piece_type }
	}

	pub fn piece_color(&self) -> &TeamColor { &self.color }

	pub fn piece_type(&self) -> &PieceType { &self.piece_type }

	pub fn symbol(&self) -> &'static str {
		use PieceType::*;
		use TeamColor::*;
		match (&self.color, &self.piece_type) {
			(White, King) => "♔",
			(White, Queen) => "♕",
			(White, Rook) => "♖",
			(White, Bishop) => "♗",
			(White, Knight) => "♘",
			(White, Pawn) => "♙",
			(Black, King) => "♚",
			(Black, Queen) => "♛",
			(Black, Rook) => "♜",
			(Black, Bishop) => "♝",
			(Black, Knight) => "♞",
			(Black, Pawn) => "♟︎"
		}
	}

	fn mesh_handle(&self, asset_server: &Res<AssetServer>) -> Handle<Mesh> {
		let path =
			format!("{}{}", Piece::MESH_PATH, self.piece_type.mesh_file_name());
		asset_server.load(path)
	}

	fn material_handle(
		&self,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		theme: &Res<Theme>
	) -> Handle<StandardMaterial> {
		let base_color: Color = self.color.piece_color(theme);
		materials.add(StandardMaterial {
			base_color,
			..default()
		})
	}

	fn translation(&self, position: Position) -> Vec3 {
		self.piece_type.mesh_translation_offset() + position.translation()
	}

	pub fn pbr_bundle(
		&self,
		asset_server: &Res<AssetServer>,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		position: &Position,
		theme: &Res<Theme>
	) -> PbrBundle {
		let mesh: Handle<Mesh> = self.mesh_handle(asset_server);
		let material: Handle<StandardMaterial> =
			self.material_handle(materials, theme);
		let translation: Vec3 = self.translation(position.clone());
		let transform: Transform =
			Transform::from_translation(translation).with_scale(Piece::SCALE);

		PbrBundle {
			mesh,
			material,
			transform,
			..default()
		}
	}
}
