use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use super::TurnState;
use crate::{
    camera::SetCameraTargetAlpha,
    piece::{CalculateAvailableMoves, CalculateAvailableMovesDone, PieceColor},
    resources::ActiveColor,
    GameSettings
};

pub struct TurnStartPlugin;

impl Plugin for TurnStartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnStartChecklist::default())
            .add_systems(
                OnEnter(TurnState::Start),
                (start_calculate_available_moves, move_camera)
            )
            .add_systems(
                Update,
                update_checklist.run_if(in_state(TurnState::Start))
            );
    }
}

/// Tracks whose turn it is. White always goes first.
#[derive(Resource, Default)]
pub struct TurnStartChecklist {
    moved_camera:     bool,
    calculated_moves: bool
}

impl TurnStartChecklist {
    fn done(&mut self) -> bool { self.moved_camera && self.calculated_moves }

    fn reset(&mut self) {
        self.moved_camera = false;
        self.calculated_moves = false;
    }
}

fn update_checklist(
    mut event_reader_moves: EventReader<CalculateAvailableMovesDone>,
    mut start_turn_checklist: ResMut<TurnStartChecklist>,
    mut turn_state: ResMut<NextState<TurnState>>
) {
    if let Some(_) = event_reader_moves.iter().last() {
        start_turn_checklist.calculated_moves = true
    };

    if start_turn_checklist.done() {
        start_turn_checklist.reset();
        debug!("moving to {:?}", TurnState::SelectMove);
        turn_state.set(TurnState::SelectMove);
    } else {
        trace!("waiting for move calculations to finish")
    }
}

fn start_calculate_available_moves(
    mut event_writer: EventWriter<CalculateAvailableMoves>
) {
    event_writer.send(CalculateAvailableMoves)
}

const WHITE_ALPHA: f32 = 0.0;
const BLACK_ALPHA: f32 = std::f32::consts::TAU / 2.0;

fn move_camera(
    mut event_writer: EventWriter<SetCameraTargetAlpha>,
    active_color: Res<ActiveColor>,
    game_settings: Res<GameSettings>,
    mut start_turn_checklist: ResMut<TurnStartChecklist>,
    camera_query: Query<Entity, With<PanOrbitCamera>>
) {
    if game_settings.should_rotate_camera {
        let Some(entity) = camera_query.iter().last() else {
            error!("No camera found in query, cannot move camera");
            return;
        };

        use PieceColor::*;
        let target_alpha = match active_color.0 {
            White => WHITE_ALPHA,
            Black => BLACK_ALPHA
        };

        event_writer.send(SetCameraTargetAlpha {
            entity,
            target_alpha
        });
    }

    start_turn_checklist.moved_camera = true;
}
