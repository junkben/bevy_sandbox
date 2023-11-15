use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{selection::UserSelectedPiece, Piece, PieceSelectionBundle};
use crate::game::{
	move_tracker::MoveTracker, physics::TranslationalMotion,
	position::Position, resources::Theme
};

pub struct SpawnPiecePlugin;
impl Plugin for SpawnPiecePlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<SpawnPiece>()
			.add_event::<PieceCaptured>()
			.add_systems(
				Update,
				(
					handle_event_spawn_piece.run_if(on_event::<SpawnPiece>()),
					handle_event_piece_captured
						.run_if(on_event::<PieceCaptured>())
				)
			);
	}
}

#[derive(Bundle)]
pub struct PieceBundle {
	pbr_bundle:         PbrBundle,
	selection:          PieceSelectionBundle,
	on_pointer_click:   On<Pointer<Click>>,
	translation_motion: TranslationalMotion,
	board_position:     Position,
	piece:              Piece,
	move_tracker:       MoveTracker
}

impl PieceBundle {
	pub fn new(
		pbr_bundle: PbrBundle,
		position: Position,
		piece: Piece
	) -> PieceBundle {
		PieceBundle {
			pbr_bundle,
			selection: PieceSelectionBundle::default(),
			on_pointer_click: On::<Pointer<Click>>::send_event::<
				UserSelectedPiece
			>(),
			translation_motion: TranslationalMotion::new(
				position.translation()
			),
			board_position: position,
			piece,
			move_tracker: MoveTracker::default()
		}
	}
}

#[derive(Event)]
pub struct SpawnPiece {
	pub piece:    Piece,
	pub position: Position
}

fn handle_event_spawn_piece(
	mut commands: Commands,
	mut er_spawn_piece: EventReader<SpawnPiece>,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	theme: Res<Theme>
) {
	for spawn_piece_event in er_spawn_piece.into_iter() {
		let piece = spawn_piece_event.piece;
		let position = spawn_piece_event.position;

		let pbr_bundle =
			piece.pbr_bundle(&asset_server, &mut materials, &position, &theme);

		let piece_bundle = PieceBundle::new(pbr_bundle, position, piece);
		commands.spawn(piece_bundle);
	}
}

#[derive(Event)]
pub struct PieceCaptured {
	pub entity: Entity
}

fn handle_event_piece_captured(
	mut commands: Commands,
	mut er_piece_captured: EventReader<PieceCaptured>
) {
	for event in er_piece_captured.into_iter() {
		commands.entity(event.entity).despawn();
	}
}

pub const INITIAL_PIECE_POSITIONS: [(Position, Piece); 32] = [
	(Position::A8, Piece::BLACK_ROOK),
	(Position::B8, Piece::BLACK_KNIGHT),
	(Position::C8, Piece::BLACK_BISHOP),
	(Position::D8, Piece::BLACK_QUEEN),
	(Position::E8, Piece::BLACK_KING),
	(Position::F8, Piece::BLACK_BISHOP),
	(Position::G8, Piece::BLACK_KNIGHT),
	(Position::H8, Piece::BLACK_ROOK),
	(Position::A7, Piece::BLACK_PAWN),
	(Position::B7, Piece::BLACK_PAWN),
	(Position::C7, Piece::BLACK_PAWN),
	(Position::D7, Piece::BLACK_PAWN),
	(Position::E7, Piece::BLACK_PAWN),
	(Position::F7, Piece::BLACK_PAWN),
	(Position::G7, Piece::BLACK_PAWN),
	(Position::H7, Piece::BLACK_PAWN),
	(Position::A2, Piece::WHITE_PAWN),
	(Position::B2, Piece::WHITE_PAWN),
	(Position::C2, Piece::WHITE_PAWN),
	(Position::D2, Piece::WHITE_PAWN),
	(Position::E2, Piece::WHITE_PAWN),
	(Position::F2, Piece::WHITE_PAWN),
	(Position::G2, Piece::WHITE_PAWN),
	(Position::H2, Piece::WHITE_PAWN),
	(Position::A1, Piece::WHITE_ROOK),
	(Position::B1, Piece::WHITE_KNIGHT),
	(Position::C1, Piece::WHITE_BISHOP),
	(Position::D1, Piece::WHITE_QUEEN),
	(Position::E1, Piece::WHITE_KING),
	(Position::F1, Piece::WHITE_BISHOP),
	(Position::G1, Piece::WHITE_KNIGHT),
	(Position::H1, Piece::WHITE_ROOK)
];
