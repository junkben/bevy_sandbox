use bevy::prelude::*;

use super::TurnState;
use crate::{
	physics::TranslationalMotionDone,
	piece::{MovePieceToBoardPosition, Piece},
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
				confirm_move.run_if(on_event::<MoveSelected>())
			)
		);
	}
}

#[derive(Event)]
pub struct MoveSelected {
	pub entity:    Entity,
	pub move_info: MoveInfo
}

fn confirm_move(
	mut commands: Commands,
	mut event_reader: EventReader<MoveSelected>,
	mut event_writer: EventWriter<MovePieceToBoardPosition>,
	mut move_history: ResMut<MoveHistory>,
	castle_availability: Res<CastleAvailability>
) {
	let Some(event) = event_reader.into_iter().last() else {
		error!("not exactly one MoveSelected event");
		return;
	};

	// TODO: Remove captured piece in more elegant way via animation?
	// If it's a capture, remove the captured entity
	if let MoveType::Capture { captured, .. } = event.move_info.move_type {
		commands.entity(captured).despawn();
	}

	// Send the event to physically move the piece to the new board position
	event_writer.send(MovePieceToBoardPosition {
		entity:      event.entity,
		destination: event.move_info.final_position
	});

	// Add the move to the MoveHistory resource
	move_history.append_move(event.move_info);

	// Conditionally do some other stuff
	match event.move_info.move_type {
		// If it's a move, we're done
		MoveType::Move => return,
		// If it's a pawn first move, we're done
		MoveType::FirstMove => return,
		// If it's a capture, despawn the captured entity
		MoveType::Capture { captured, .. } => {
			commands.entity(captured).despawn();
		},
		// If it's a capture en passant, despawn the captured entity
		MoveType::CaptureEnPassant { captured, .. } => {
			commands.entity(captured).despawn();
		},
		MoveType::PawnPromotion { .. } => todo!(),
		// If it's a castle, we need to move the rook too
		MoveType::Castle(castle_type) => {
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
			event_writer.send(MovePieceToBoardPosition {
				entity:      entities.rook,
				destination: entities.rook_destination
			});
		},
		MoveType::Check => todo!(),
		MoveType::Checkmate => todo!(),
		MoveType::DrawOffer => todo!()
	}
}

fn wait_for_piece_motion_to_complete(
	mut event_reader: EventReader<TranslationalMotionDone>,
	mut turn_state: ResMut<NextState<TurnState>>,
	piece_query: Query<Entity, With<Piece>>
) {
	for event in event_reader.into_iter() {
		let Ok(_entity) = piece_query.get(event.entity) else {
			return;
		};

		// Event entity handshake succeeded, move to next state
		debug!("moving to {:?}", TurnState::UpdateBoardState);
		turn_state.set(TurnState::UpdateBoardState);
	}
}
