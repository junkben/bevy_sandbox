use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use super::TurnState;
use crate::{piece::color::PieceColor, resources::BoardState};

pub struct TurnStartPlugin;

impl Plugin for TurnStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera.run_if(in_state(TurnState::Start)));
    }
}

const WHITE_ALPHA: f32 = 0.0;
const BLACK_ALPHA: f32 = TAU / 2.0;

fn move_camera(
    board_state: Res<BoardState>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut camera_query: Query<&mut PanOrbitCamera>
) {
    let mut camera = camera_query.iter_mut().next().unwrap();

    use PieceColor::*;
    camera.target_alpha = match board_state.active_color {
        White => WHITE_ALPHA,
        Black => BLACK_ALPHA
    };

    turn_state.set(TurnState::SelectPiece);
}
