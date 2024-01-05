use std::collections::HashMap;

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
		app.add_event::<SpawnPieces>()
			.add_event::<SpawnPiecesDone>()
			.add_event::<PieceCaptured>()
			.add_systems(
				Update,
				(
					handle_event_spawn_pieces.run_if(on_event::<SpawnPieces>()),
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
pub struct SpawnPieces(pub HashMap<Position, Piece>);

#[derive(Event)]
pub struct SpawnPiecesDone;

fn handle_event_spawn_pieces(
	mut commands: Commands,
	mut er_spawn_piece: EventReader<SpawnPieces>,
	mut ew_done: EventWriter<SpawnPiecesDone>,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	theme: Res<Theme>
) {
	if let Some(event) = er_spawn_piece.read().last() {
		for (position, piece) in &event.0 {
			let pbr_bundle = piece.pbr_bundle(
				&asset_server,
				&mut materials,
				&position,
				&theme
			);

			commands.spawn(PieceBundle::new(pbr_bundle, *position, *piece));
		}

		ew_done.send(SpawnPiecesDone)
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
	for event in er_piece_captured.read() {
		commands.entity(event.entity).despawn();
	}
}

pub const INITIAL_PIECE_POSITIONS: phf::Map<&'static str, Piece> = phf::phf_map! {
	"A8" => Piece::BLACK_ROOK,
	"B8" => Piece::BLACK_KNIGHT,
	"C8" => Piece::BLACK_BISHOP,
	"D8" => Piece::BLACK_QUEEN,
	"E8" => Piece::BLACK_KING,
	"F8" => Piece::BLACK_BISHOP,
	"G8" => Piece::BLACK_KNIGHT,
	"H8" => Piece::BLACK_ROOK,
	"A7" => Piece::BLACK_PAWN,
	"B7" => Piece::BLACK_PAWN,
	"C7" => Piece::BLACK_PAWN,
	"D7" => Piece::BLACK_PAWN,
	"E7" => Piece::BLACK_PAWN,
	"F7" => Piece::BLACK_PAWN,
	"G7" => Piece::BLACK_PAWN,
	"H7" => Piece::BLACK_PAWN,
	"A2" => Piece::WHITE_PAWN,
	"B2" => Piece::WHITE_PAWN,
	"C2" => Piece::WHITE_PAWN,
	"D2" => Piece::WHITE_PAWN,
	"E2" => Piece::WHITE_PAWN,
	"F2" => Piece::WHITE_PAWN,
	"G2" => Piece::WHITE_PAWN,
	"H2" => Piece::WHITE_PAWN,
	"A1" => Piece::WHITE_ROOK,
	"B1" => Piece::WHITE_KNIGHT,
	"C1" => Piece::WHITE_BISHOP,
	"D1" => Piece::WHITE_QUEEN,
	"E1" => Piece::WHITE_KING,
	"F1" => Piece::WHITE_BISHOP,
	"G1" => Piece::WHITE_KNIGHT,
	"H1" => Piece::WHITE_ROOK
};
