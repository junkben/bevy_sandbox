mod color;
mod selection;
mod spawn;
mod square;

use bevy::prelude::*;
pub use color::SquareColor;
pub use selection::SquareSelectionBundle;
pub use square::Square;

use crate::position::Position;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_board);
    }
}

#[derive(Bundle)]
struct SquareBundle {
    square:         Square,
    pbr_bundle:     PbrBundle,
    board_position: Position
}

impl SquareBundle {
    pub fn new(
        square: Square,
        pbr_bundle: PbrBundle,
        board_position: Position
    ) -> SquareBundle {
        SquareBundle {
            square,
            pbr_bundle,
            board_position
        }
    }
}
