pub mod color;
mod movement;
pub mod piece_type;
mod spawn;

use bevy::prelude::*;
use bevy_mod_picking::{prelude::RaycastPickTarget, PickableBundle};

use self::{color::PieceColor, piece_type::PieceType};
use crate::{board::position::BoardPosition, resources::Theme};

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
    board_position:      BoardPosition,
    piece:               Piece
}

macro_rules! chess_pieces {
    ($($name:ident, $color:ident, $piece_type:ident);*) => {
        $(
            pub const $name: Piece = Piece {
                color: PieceColor::$color,
                piece_type: PieceType::$piece_type
            };
        )*
    };
}

chess_pieces!(
    WHITE_KING, White, King;
    WHITE_QUEEN, White, Queen;
    WHITE_ROOK, White, Rook;
    WHITE_BISHOP, White, Bishop;
    WHITE_KNIGHT, White, Knight;
    WHITE_PAWN, White, Pawn;
    BLACK_KING, Black, King;
    BLACK_QUEEN, Black, Queen;
    BLACK_ROOK, Black, Rook;
    BLACK_BISHOP, Black, Bishop;
    BLACK_KNIGHT, Black, Knight;
    BLACK_PAWN, Black, Pawn
);

const SCALE: Vec3 = Vec3 {
    x: 0.012,
    y: 0.012,
    z: 0.012
};

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color:      PieceColor,
    pub piece_type: PieceType
}

impl From<(PieceColor, PieceType)> for Piece {
    fn from(value: (PieceColor, PieceType)) -> Self {
        let (color, piece_type) = value;
        Piece { color, piece_type }
    }
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
    pub fn symbol(&self) -> &str {
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

    fn position(&self, board_position: BoardPosition) -> Vec3 {
        self.piece_type.mesh_offset() + board_position.vec3()
    }

    pub fn pbr_bundle(
        &self,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        board_position: &BoardPosition,
        theme: &Res<Theme>
    ) -> PbrBundle {
        info!("spawning {}{}", self, &board_position);

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
