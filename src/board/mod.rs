mod color;
mod spawn;
pub mod square;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use self::square::Square;
use crate::position::BoardPosition;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_board);
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
