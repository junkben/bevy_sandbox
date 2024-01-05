use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use super::TurnState;
use crate::game::{
	camera::SetCameraTargetAlpha,
	piece::PieceColor,
	resources::{
		ActiveColor, CalculateAvailableMoves, CalculateAvailableMovesDone,
		CheckCastleAvailability, CheckCastleAvailabilityDone, CheckEnPassant,
		CheckEnPassantDone, UpdateAttackedPositions,
		UpdateAttackedPositionsDone
	},
	GameSettings
};

pub struct TurnStartPlugin;

impl Plugin for TurnStartPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(TurnStartChecklist::default())
			.add_event::<StartTurn>()
			.add_event::<MoveCamera>()
			.add_systems(OnEnter(TurnState::Start), on_start_turn)
			.add_systems(
				Update,
				check_start_turn_done.run_if(in_state(TurnState::Start))
			)
			.add_systems(
				Update,
				(
					handle_event_start_turn,
					handle_event_check_en_passant_done,
					handle_event_check_available_moves_done,
					handle_event_check_castle_availability_done,
					handle_event_update_attacked_positions_done,
					handle_event_move_camera
				)
			);
	}
}

/// Tracks whose turn it is. White always goes first.
#[derive(Resource, Default)]
pub struct TurnStartChecklist {
	moved_camera:              bool,
	check_castle_availability: bool,
	check_en_passant:          bool,
	calculated_moves:          bool,
	update_attacked_positions: bool
}

impl TurnStartChecklist {
	fn done(&self) -> bool {
		self.moved_camera
			&& self.check_castle_availability
			&& self.check_en_passant
			&& self.calculated_moves
			&& self.update_attacked_positions
	}

	fn reset(&mut self) {
		self.moved_camera = false;
		self.check_castle_availability = false;
		self.check_en_passant = false;
		self.calculated_moves = false;
		self.update_attacked_positions = false;
	}
}

fn on_start_turn(mut ew_start_turn: EventWriter<StartTurn>) {
	ew_start_turn.send(StartTurn)
}

#[derive(Event)]
pub struct StartTurn;

/// Step 1: Need to check en_passant as available moves depends on it.
fn handle_event_start_turn(
	mut er_start_turn: EventReader<StartTurn>,
	mut ew_en_passant: EventWriter<CheckEnPassant>
) {
	if let Some(_) = er_start_turn.read().last() {
		ew_en_passant.send(CheckEnPassant);
	}
}

/// Step 2: Check available moves
fn handle_event_check_en_passant_done(
	mut er_en_passant: EventReader<CheckEnPassantDone>,
	mut ew_moves: EventWriter<CalculateAvailableMoves>,
	mut checklist: ResMut<TurnStartChecklist>
) {
	if let Some(_) = er_en_passant.read().last() {
		checklist.check_en_passant = true;
		ew_moves.send(CalculateAvailableMoves);
	}
}

/// Step 3: Do the rest of the start turn process in no particular order
fn handle_event_check_available_moves_done(
	mut er_moves: EventReader<CalculateAvailableMovesDone>,
	mut ew_check: EventWriter<CheckCastleAvailability>,
	mut ew_attacked: EventWriter<UpdateAttackedPositions>,
	mut ew_move_camera: EventWriter<MoveCamera>,
	mut checklist: ResMut<TurnStartChecklist>
) {
	if let Some(_) = er_moves.read().last() {
		checklist.calculated_moves = true;
		ew_check.send(CheckCastleAvailability);
		ew_attacked.send(UpdateAttackedPositions);
		ew_move_camera.send(MoveCamera);
	}
}

fn handle_event_check_castle_availability_done(
	mut er_castle_done: EventReader<CheckCastleAvailabilityDone>,
	mut checklist: ResMut<TurnStartChecklist>
) {
	if let Some(_) = er_castle_done.read().last() {
		checklist.check_castle_availability = true;
	}
}

fn handle_event_update_attacked_positions_done(
	mut er_attacked_done: EventReader<UpdateAttackedPositionsDone>,
	mut checklist: ResMut<TurnStartChecklist>
) {
	if let Some(_) = er_attacked_done.read().last() {
		checklist.update_attacked_positions = true;
	}
}

#[derive(Event)]
pub struct StartTurnDone;

fn check_start_turn_done(
	mut checklist: ResMut<TurnStartChecklist>,
	mut turn_state: ResMut<NextState<TurnState>>
) {
	if !checklist.done() {
		return;
	}

	checklist.reset();

	debug!("moving to {:?}", TurnState::SelectMove);
	turn_state.set(TurnState::SelectMove);
}

#[derive(Event)]
pub struct MoveCamera;

const WHITE_ALPHA: f32 = 0.0;
const BLACK_ALPHA: f32 = std::f32::consts::TAU / 2.0;

fn handle_event_move_camera(
	mut er_move_camera: EventReader<MoveCamera>,
	mut ew_set: EventWriter<SetCameraTargetAlpha>,
	active_color: Res<ActiveColor>,
	game_settings: Res<GameSettings>,
	mut start_turn_checklist: ResMut<TurnStartChecklist>,
	query_camera: Query<Entity, With<PanOrbitCamera>>
) {
	if let Some(_event) = er_move_camera.read().last() {
		start_turn_checklist.moved_camera = true;

		if game_settings.should_rotate_camera {
			info!("Rotating camera...");

			let Some(entity) = query_camera.iter().last() else {
				error!("No camera found in query, cannot move camera");
				return;
			};

			use PieceColor::*;
			let target_alpha = match active_color.0 {
				White => WHITE_ALPHA,
				Black => BLACK_ALPHA
			};

			ew_set.send(SetCameraTargetAlpha {
				entity,
				target_alpha
			});
		} else {
			info!("Skipping rotating camera...")
		}
	}
}
