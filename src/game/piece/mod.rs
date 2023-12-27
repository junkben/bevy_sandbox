mod color;
mod movement;
mod piece_type;
mod selection;
mod spawn;

use bevy::prelude::*;
pub use color::PieceColor;
pub use movement::{
	MovePieceToBoardPosition, MovementType, PieceMovementBehavior
};
pub use piece_type::PieceType;
pub use selection::{PieceSelectionBundle, UserSelectedPiece};
pub use spawn::{PieceCaptured, SpawnPiece, INITIAL_PIECE_POSITIONS};

use self::{
	movement::PieceMovementPlugin, selection::PieceSelectPlugin,
	spawn::SpawnPiecePlugin
};
use crate::game::{position::Position, resources::Theme};

macro_rules! chess_pieces {
	($($name:ident, $color:ident, $piece_type:ident);*) => {
		$(
			pub const $name: Piece = Piece {
				color:	  PieceColor::$color,
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

const SCALE: Vec3 = Vec3 {
	x: 0.012,
	y: 0.012,
	z: 0.012
};

/// A component that represents a chess piece
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
	color:      PieceColor,
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
	chess_pieces!(
		WHITE_KING, WHITE, KING;
		WHITE_QUEEN, WHITE, QUEEN;
		WHITE_ROOK, WHITE, ROOK;
		WHITE_BISHOP, WHITE, BISHOP;
		WHITE_KNIGHT, WHITE, KNIGHT;
		WHITE_PAWN, WHITE, PAWN;
		BLACK_KING, BLACK, KING;
		BLACK_QUEEN, BLACK, QUEEN;
		BLACK_ROOK, BLACK, ROOK;
		BLACK_BISHOP, BLACK, BISHOP;
		BLACK_KNIGHT, BLACK, KNIGHT;
		BLACK_PAWN, BLACK, PAWN
	);

	pub fn piece_color(&self) -> &PieceColor { &self.color }

	pub fn piece_type(&self) -> &PieceType { &self.piece_type }

	pub fn symbol(&self) -> &'static str {
		use PieceColor::*;
		use PieceType::*;
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
		let path = format!("models/chess/{}", self.piece_type.mesh_file_name());
		asset_server.load(path.as_str())
	}

	fn material_handle(
		&self,
		materials: &mut ResMut<Assets<StandardMaterial>>,
		theme: &Res<Theme>
	) -> Handle<StandardMaterial> {
		let base_color: Color = self.color.color(theme);
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
			Transform::from_translation(translation).with_scale(SCALE);

		PbrBundle {
			mesh,
			material,
			transform,
			..default()
		}
	}
}
