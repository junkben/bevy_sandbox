pub mod color;
pub mod movement;
pub mod piece_type;
mod spawn;

use bevy::prelude::*;
use bevy_mod_picking::{prelude::RaycastPickTarget, PickableBundle};

use self::{color::*, piece_type::*};
use crate::{position::Position, resources::Theme};

macro_rules! chess_pieces {
    ($($name:ident, $color:ident, $piece_type:ident);*) => {
        $(
            pub const $name: Piece = Piece {
                color: &PieceColor::$color,
                piece_type: &PieceType::$piece_type
            };
        )*
    };
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_pieces)
            .add_systems(Update, movement::move_pieces);
    }
}

#[derive(Bundle)]
pub struct PieceBundle {
    pbr_bundle:          PbrBundle,
    pickable_bundle:     PickableBundle,
    raycast_pick_target: RaycastPickTarget,
    board_position:      Position,
    piece:               Piece
}

const SCALE: Vec3 = Vec3 {
    x: 0.012,
    y: 0.012,
    z: 0.012
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    color:      &'static PieceColor,
    piece_type: &'static PieceType
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PieceColor::*;
        use PieceType::*;
        write!(f, "{}", match (&self.color, &self.piece_type) {
            (White, King) => 'K',
            (White, Queen) => 'Q',
            (White, Rook) => 'R',
            (White, Bishop) => 'B',
            (White, Knight) => 'N',
            (White, Pawn) => 'P',
            (Black, King) => 'k',
            (Black, Queen) => 'q',
            (Black, Rook) => 'r',
            (Black, Bishop) => 'b',
            (Black, Knight) => 'k',
            (Black, Pawn) => 'p'
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

    pub fn piece_color(&self) -> &'static PieceColor { self.color }

    pub fn piece_type(&self) -> &'static PieceType { self.piece_type }

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
        let base_color = self.color.color(theme);
        materials.add(StandardMaterial {
            base_color,
            ..default()
        })
    }

    fn position(&self, board_position: Position) -> Vec3 {
        self.piece_type.mesh_offset() + board_position.vec3()
    }

    pub fn pbr_bundle(
        &self,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        board_position: &Position,
        theme: &Res<Theme>
    ) -> PbrBundle {
        info!("spawning {}{}", self, board_position);

        let mesh = self.mesh_handle(asset_server);
        let material = self.material_handle(materials, theme);
        let translation = self.position(board_position.clone());
        let transform =
            Transform::from_translation(translation).with_scale(SCALE);

        PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        }
    }
}
