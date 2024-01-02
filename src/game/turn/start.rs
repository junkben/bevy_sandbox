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
			.add_event::<CalculateAvailableMoves>()
			.add_event::<CalculateAvailableMovesDone>()
			.add_systems(
				OnEnter(TurnState::Start),
				(check_castle_availability, move_camera)
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
	moved_camera:              bool,
	check_castle_availability: bool,
	check_en_passant:          bool,
	calculated_moves:          bool,
	update_attacked_positions: bool
}

impl TurnStartChecklist {
	fn done(&mut self) -> bool {
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

fn update_checklist(
	mut er_castle: EventReader<CheckCastleAvailabilityDone>,
	mut ew_moves: EventWriter<CalculateAvailableMoves>,
	mut er_moves: EventReader<CalculateAvailableMovesDone>,
	mut ew_attacked: EventWriter<UpdateAttackedPositions>,
	mut er_attacked: EventReader<UpdateAttackedPositionsDone>,
	mut ew_en_passant: EventWriter<CheckEnPassant>,
	mut er_en_passant: EventReader<CheckEnPassantDone>,
	mut start_turn_checklist: ResMut<TurnStartChecklist>,
	mut turn_state: ResMut<NextState<TurnState>>
) {
	if let Some(_event) = er_attacked.read().last() {
		start_turn_checklist.update_attacked_positions = true;
		debug!("consumed UpdateAttackedPositionsDone");
	};

	if let Some(_event) = er_moves.read().last() {
		start_turn_checklist.calculated_moves = true;
		debug!("consumed CalculateAvailableMovesDone");

		ew_attacked.send(UpdateAttackedPositions)
	};

	if let Some(_event) = er_en_passant.read().last() {
		start_turn_checklist.check_en_passant = true;
		debug!("consumed CheckEnPassantDone");

		ew_moves.send(CalculateAvailableMoves)
	};

	if let Some(_event) = er_castle.read().last() {
		start_turn_checklist.check_castle_availability = true;
		debug!("consumed CheckCastleAvailabilityDone");

		ew_en_passant.send(CheckEnPassant)
	};

	if start_turn_checklist.done() {
		start_turn_checklist.reset();
		debug!("moving to {:?}", TurnState::SelectMove);
		turn_state.set(TurnState::SelectMove);
	} else {
		debug!("not done with start turn checklist yet...")
	}
}

fn check_castle_availability(
	mut ew_check: EventWriter<CheckCastleAvailability>
) {
	ew_check.send(CheckCastleAvailability)
}

const WHITE_ALPHA: f32 = 0.0;
const BLACK_ALPHA: f32 = std::f32::consts::TAU / 2.0;

fn move_camera(
	mut ew_set: EventWriter<SetCameraTargetAlpha>,
	active_color: Res<ActiveColor>,
	game_settings: Res<GameSettings>,
	mut start_turn_checklist: ResMut<TurnStartChecklist>,
	query_camera: Query<Entity, With<PanOrbitCamera>>
) {
	if game_settings.should_rotate_camera {
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
	}

	start_turn_checklist.moved_camera = true;
}
