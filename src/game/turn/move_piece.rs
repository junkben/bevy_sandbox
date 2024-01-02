use bevy::prelude::*;

use super::TurnState;
use crate::game::{
	physics::TranslationalMotionDone,
	piece::{MovePieceToBoardPosition, Piece, PieceCaptured},
	resources::{CastleAvailability, CastleType, MoveHistory},
	MoveInfo, MoveType
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<MoveSelected>().add_systems(
			Update,
			(
				wait_for_piece_motion_to_complete
					.run_if(in_state(TurnState::MovePiece)),
				handle_event_move_selected.run_if(on_event::<MoveSelected>())
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

	// Send the event to physically move the piece to the new board position
	ew_piece_move.send(MovePieceToBoardPosition {
		entity:      event.move_info.entity,
		destination: event.move_info.final_position
	});

	// Add the move to the MoveHistory resource
	move_history.append_move(event.move_info);

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

fn wait_for_piece_motion_to_complete(
	mut er_motion_done: EventReader<TranslationalMotionDone>,
	mut turn_state: ResMut<NextState<TurnState>>,
	query_piece: Query<Entity, With<Piece>>
) {
	for event in er_motion_done.read() {
		let Ok(_entity) = query_piece.get(event.entity) else {
			return;
		};

		// Event entity handshake succeeded, move to next state
		debug!("moving to {:?}", TurnState::UpdateBoardState);
		turn_state.set(TurnState::UpdateBoardState);
	}
}
