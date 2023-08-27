use bevy::prelude::{Color, *};

use crate::{resources::theme::Theme, square::*};

macro_rules! chess_pieces {
    ($($name:ident, $color:ident, $piece:ident);*) => {
        $(
            pub const $name: ChessPiece = ChessPiece {
                color: PColor::$color,
                piece: PType::$piece
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

pub enum PColor {
    White,
    Black
}

impl std::fmt::Display for PColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PColor::*;
        write!(f, "{}", match self {
            White => "w",
            Black => "b"
        })
    }
}

impl PColor {
    fn color(&self, theme: &Res<Theme>) -> Color {
        use PColor::*;
        match self {
            White => theme.data().piece_white,
            Black => theme.data().piece_black
        }
    }
}

pub enum PType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl PType {
    fn mesh_file_name(&self) -> &str {
        use PType::*;
        match self {
            King => "king.glb#Mesh0/Primitive0",
            Queen => "queen.glb#Mesh0/Primitive0",
            Rook => "rook.glb#Mesh0/Primitive0",
            Bishop => "bishop.glb#Mesh0/Primitive0",
            Knight => "knight.glb#Mesh0/Primitive0",
            Pawn => "pawn.glb#Mesh0/Primitive0"
        }
    }

    fn mesh_offset(&self) -> Vec3 {
        use PType::*;
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

pub struct ChessPiece {
    color: PColor,
    piece: PType
}

impl From<(PColor, PType)> for ChessPiece {
    fn from(value: (PColor, PType)) -> Self {
        let (color, piece) = value;
        ChessPiece { color, piece }
    }
}

impl std::fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PColor::*;
        use PType::*;
        write!(f, "{}", match (&self.color, &self.piece) {
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
impl ChessPiece {
    pub fn symbol(&self) -> &str {
        use PColor::*;
        use PType::*;
        match (&self.color, &self.piece) {
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
        let path = format!("models/chess/{}", self.piece.mesh_file_name());
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

    fn position(&self, board_position: SquareId) -> Vec3 {
        self.piece.mesh_offset() + Vec3::from(board_position)
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        board_position: &SquareId,
        theme: &Res<Theme>
    ) {
        info!("spawning {}{}", self, &board_position);

        let mesh = self.mesh_handle(asset_server);
        let material = self.material_handle(materials, theme);
        let translation = self.position(board_position.clone());
        let transform =
            Transform::from_translation(translation).with_scale(SCALE);

        commands.spawn(PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        });
    }
}
