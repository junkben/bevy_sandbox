mod color;
pub mod position;
pub mod select;
mod spawn;
mod square;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use self::square::Square;
use crate::board::position::BoardPosition;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(select::SelectedSquare::default())
            .insert_resource(select::SelectedPiece::default())
            .add_systems(Startup, spawn::spawn_board)
            .add_systems(Update, select::select_square);
    }
}

#[derive(Bundle)]
struct SquareBundle {
    square:              Square,
    pbr_bundle:          PbrBundle,
    board_position:      BoardPosition,
    pickable_bundle:     PickableBundle,
    raycast_pick_target: RaycastPickTarget
}

impl SquareBundle {
    pub fn new(
        square: Square,
        pbr_bundle: PbrBundle,
        board_position: BoardPosition
    ) -> SquareBundle {
        SquareBundle {
            square,
            pbr_bundle,
            board_position,
            pickable_bundle: PickableBundle::default(),
            raycast_pick_target: RaycastPickTarget::default()
        }
    }
}
