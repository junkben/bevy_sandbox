use std::collections::HashMap;

use bevy::prelude::*;

use super::TurnState;
use crate::{
	audio::*,
	game::{
		physics::TranslationalMotionDone,
		piece::{MovePieceToBoardPosition, Piece, PieceCaptured, SpawnPieces},
		position::Position,
		resources::{CastleAvailability, CastleType, MoveHistory},
		MoveInfo, MoveType
	}
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<PlaySoundOnMoveComplete>()
			.add_event::<MoveSelected>()
			.add_event::<ChangePiece>()
			.add_systems(
				Update,
				(
					wait_for_piece_motion_to_complete
						.run_if(in_state(TurnState::MovePiece)),
					handle_event_move_selected
						.run_if(on_event::<MoveSelected>()),
					handle_event_play_sound_on_move_complete
						.run_if(on_event::<PlaySoundOnMoveComplete>()),
					handle_event_change_piece.run_if(on_event::<ChangePiece>())
				)
			);
	}
}

#[derive(Event)]
pub struct MoveSelected {
	pub move_info: MoveInfo
}

fn handle_event_move_selected(
	mut er_move_selected: EventReader<MoveSelected>,
	mut ew_piece_move: EventWriter<MovePieceToBoardPosition>,
	mut ew_piece_captured: EventWriter<PieceCaptured>,
	mut move_history: ResMut<MoveHistory>,
	castle_availability: Res<CastleAvailability>
) {
	let Some(event) = er_move_selected.read().last() else {
		error!("not exactly one MoveSelected event");
		return;
	};

	// Add the move to the MoveHistory resource
	move_history.append_move(event.move_info);

	// Send the event to physically move the piece to the new board position
	ew_piece_move.send(MovePieceToBoardPosition {
		entity:      event.move_info.entity,
		destination: event.move_info.final_position
	});

	// Conditionally do some other stuff
	use MoveType::*;
	match event.move_info.move_type {
		// If it's a move, we're done
		Move | FirstMove => return,
		// If it's a capture, despawn the captured entity
		Capture { captured, .. } | CaptureEnPassant { captured, .. } => {
			ew_piece_captured.send(PieceCaptured { entity: captured });
		},
		// If it's a castle, we need to move the rook too
		Castle(castle_type) => {
			let entities_opt = match castle_type {
				CastleType::WK => &castle_availability.white_kingside,
				CastleType::WQ => &castle_availability.white_queenside,
				CastleType::BK => &castle_availability.black_kingside,
				CastleType::BQ => &castle_availability.black_queenside
			};

			let Some(entities) = entities_opt else {
				unreachable!()
			};

			// Send the event to physically move the piece to the new board
			// position
			ew_piece_move.send(MovePieceToBoardPosition {
				entity:      entities.rook,
				destination: entities.rook_destination
			});
		}
	}
}

#[derive(Event)]
pub struct PlaySoundOnMoveComplete;

fn wait_for_piece_motion_to_complete(
	mut er_motion_done: EventReader<TranslationalMotionDone>,
	mut ew_play_sound: EventWriter<PlaySoundOnMoveComplete>,
	mut ew_change_piece: EventWriter<ChangePiece>,
	mut turn_state: ResMut<NextState<TurnState>>,
	move_history: Res<MoveHistory>
) {
	let Some(event) = er_motion_done.read().last() else {
		return;
	};

	// Do handshake
	let Some(last_move) = move_history.last() else {
		error!("no last move entity");
		return;
	};

	if last_move.entity != event.entity {
		error!("translational motion entity does not match last move entity");
		return;
	}

	// Handle promotion piece change now if necessary
	if let Some(piece) = last_move.promoted_to {
		ew_change_piece.send(ChangePiece {
			from: last_move.entity,
			to:   piece,
			at:   last_move.final_position
		})
	}

	ew_play_sound.send(PlaySoundOnMoveComplete);

	// Event entity handshake succeeded, move to next state
	debug!("moving to {:?}", TurnState::UpdateBoardState);
	turn_state.set(TurnState::UpdateBoardState);
}

fn handle_event_play_sound_on_move_complete(
	mut er_play_sound: EventReader<PlaySoundOnMoveComplete>,
	mut ew_move_self: EventWriter<PlaySoundMoveSelf>,
	mut ew_move_oppo: EventWriter<PlaySoundMoveOpponent>,
	mut ew_move_check: EventWriter<PlaySoundMoveCheck>,
	mut ew_capture: EventWriter<PlaySoundCapture>,
	mut ew_castle: EventWriter<PlaySoundCastle>,
	mut ew_promote: EventWriter<PlaySoundPromote>,
	move_history: Res<MoveHistory>
) {
	let Some(_) = er_play_sound.read().last() else {
		return;
	};

	let Some(last_move) = move_history.last() else {
		error!("no move to play sound for");
		return;
	};

	if last_move.is_check {
		ew_move_check.send(PlaySoundMoveCheck);
		return;
	}

	if let Some(_) = last_move.promoted_to {
		ew_promote.send(PlaySoundPromote);
		return;
	}

	if last_move.is_capture() {
		ew_capture.send(PlaySoundCapture);
		return;
	}

	if last_move.is_castle() {
		ew_castle.send(PlaySoundCastle);
		return;
	}

	if last_move.piece.piece_color().is_white() {
		ew_move_self.send(PlaySoundMoveSelf);
	} else {
		ew_move_oppo.send(PlaySoundMoveOpponent);
	}
}

#[derive(Event)]
pub struct ChangePiece {
	from: Entity,
	to:   Piece,
	at:   Position
}

fn handle_event_change_piece(
	mut commands: Commands,
	mut er_change_piece: EventReader<ChangePiece>,
	mut ew_spawn_pieces: EventWriter<SpawnPieces>
) {
	let Some(event) = er_change_piece.read().last() else {
		return;
	};

	commands.entity(event.from).despawn_recursive();
	ew_spawn_pieces.send(SpawnPieces(HashMap::from([(event.at, event.to)])));
}
