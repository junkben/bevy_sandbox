use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{
    available_moves::AvailableMoves, selection::SelectPiece, Piece,
    PieceSelectionBundle
};
use crate::{
    move_tracker::MoveTracker, physics::TranslationalMotion,
    position::Position, resources::Theme
};

pub struct SpawnPiecePlugin;
impl Plugin for SpawnPiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPiece>()
            .add_systems(Update, read_spawn_piece_events);
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
    move_tracker:       MoveTracker,
    available_moves:    AvailableMoves
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
            on_pointer_click: On::<Pointer<Click>>::send_event::<SelectPiece>(),
            translation_motion: TranslationalMotion::new(
                position.translation()
            ),
            board_position: position,
            piece,
            move_tracker: MoveTracker::default(),
            available_moves: AvailableMoves::default()
        }
    }
}

#[derive(Event)]
pub struct SpawnPiece {
    pub piece:    Piece,
    pub position: Position
}

fn read_spawn_piece_events(
    mut commands: Commands,
    mut spawn_piece_reader: EventReader<SpawnPiece>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>
) {
    for spawn_piece_event in spawn_piece_reader.into_iter() {
        let piece = spawn_piece_event.piece;
        let position = spawn_piece_event.position;

        let pbr_bundle =
            piece.pbr_bundle(&asset_server, &mut materials, &position, &theme);

        let piece_bundle = PieceBundle::new(pbr_bundle, position, piece);
        commands.spawn(piece_bundle);
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
