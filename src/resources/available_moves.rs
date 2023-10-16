use std::collections::HashMap;

use bevy::prelude::*;

use super::CastleAvailability;
use crate::{
	move_info::MoveInfo,
	move_tracker::MoveTracker,
	piece::{
		MovementType, Piece, PieceColor, PieceMovementBehavior, PieceType
	},
	position::*,
	resources::CastleType,
	MoveType
};

pub struct AvailableMovesPlugin;
impl Plugin for AvailableMovesPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(AvailableMoves::default())
			.add_event::<CalculateAvailableMoves>()
			.add_event::<CalculateAvailableMovesDone>()
			.add_systems(
				Update,
				calculate_available_moves
					.run_if(on_event::<CalculateAvailableMoves>())
			);
	}
}

/// A component that tracks the available positions an entity can move to
#[derive(Resource, Default, Debug, Clone)]
pub struct AvailableMoves(pub HashMap<Entity, Vec<MoveInfo>>);

impl std::fmt::Display for AvailableMoves {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl AvailableMoves {
	pub fn contains_move_to(
		&self,
		entity: &Entity,
		position: &Position
	) -> bool {
		if let Some(moves) = self.0.get(entity) {
			for m in moves {
				if &m.final_position == position {
					return true;
				}
			}
		}

		return false;
	}

	pub fn get_move_to(
		&self,
		entity: &Entity,
		position: &Position
	) -> Option<&MoveInfo> {
		if let Some(moves) = self.0.get(entity) {
			for m in moves {
				if &m.final_position == position {
					return Some(m);
				}
			}
		}

		return None;
	}
}

pub enum SquareState {
	Vacant,
	Opposing(Entity),
	Friendly
}

#[derive(Event)]
pub struct CalculateAvailableMoves;

#[derive(Event)]
pub struct CalculateAvailableMovesDone;

fn calculate_available_moves(
	mut event_reader: EventReader<CalculateAvailableMoves>,
	mut event_writer: EventWriter<CalculateAvailableMovesDone>,
	mut available_moves: ResMut<AvailableMoves>,
	castle_availability: Res<CastleAvailability>,
	mut piece_query: Query<(Entity, &Piece, &Position, &MoveTracker)>,
	opposing_piece_query: Query<(Entity, &Position, &Piece)>
) {
	// Consume CalculateAvailableMoves
	let Some(_) = event_reader.into_iter().last() else {
		error!("not exactly one CalculateAvailableMoves event");
		return;
	};

	for (entity, &piece, &initial_position, &move_tracker) in
		piece_query.iter_mut()
	{
		// Gather piece default movement patterns
		use PieceType::*;
		let movement_patterns = match piece.piece_type() {
			King => PieceMovementBehavior::king(),
			Queen => PieceMovementBehavior::queen(),
			Rook => PieceMovementBehavior::rook(),
			Bishop => PieceMovementBehavior::bishop(),
			Knight => PieceMovementBehavior::knight(),
			Pawn => {
				// Grab piece color
				let piece_color = *piece.piece_color();

				// It is the pawn's first move if it hasn't moved yet
				let first_move = !move_tracker.has_moved();

				PieceMovementBehavior::pawn(piece_color, first_move)
			}
		};

		let (start_x, start_z) = initial_position.xz();
		let start_vec = Vec3::new(start_x as f32, 0.0, start_z as f32);

		let mut moves: Vec<MoveInfo> = Vec::new();

		for (direction, max_magnitude, special_move) in movement_patterns.iter()
		{
			let mut magnitude: u8 = 0u8;

			while magnitude < *max_magnitude {
				magnitude += 1;

				let vector = start_vec + (direction.clone() * magnitude as f32);

				// If the proposed Position can't exist, break
				let final_position = match Position::try_from_vec3(vector) {
					Some(bp) => bp,
					None => break
				};

				// Check if there is a piece at the end position. If there
				// is, we'll record it's color
				let move_info_opt = determine_move(
					&initial_position,
					&final_position,
					&piece,
					special_move,
					&castle_availability,
					&opposing_piece_query
				);

				if let Some(move_info) = move_info_opt {
					if let MoveType::Capture {
						is_en_passant: _,
						captured: _
					} = move_info.move_type
					{
						magnitude = *max_magnitude;
					}

					// Add move to possible moves
					moves.push(move_info);
				} else {
					break;
				}
			} // end while
		} // end for

		available_moves.0.insert(entity, moves);
		trace!(?piece, ?initial_position, ?available_moves);
	}

	event_writer.send(CalculateAvailableMovesDone)
}

fn determine_move(
	initial_position: &Position,
	final_position: &Position,
	piece: &Piece,
	special_move: &MovementType,
	castle_availability: &Res<CastleAvailability>,
	other_piece_query: &Query<(Entity, &Position, &Piece)>
) -> Option<MoveInfo> {
	if let Some(castle_move) = determine_castle_move(
		initial_position,
		final_position,
		piece,
		special_move,
		castle_availability
	) {
		return Some(castle_move);
	}

	// Find query result
	let query_result = other_piece_query
		.iter()
		.filter(|&(_, position, _)| {
			position == final_position && position != initial_position
		})
		.last();

	// Find out the space
	let square_state = match query_result {
		Some((entity, _, other_piece)) => {
			match piece.piece_color() == other_piece.piece_color() {
				true => SquareState::Friendly,
				false => SquareState::Opposing(entity)
			}
		},
		None => SquareState::Vacant
	};

	let move_type = match square_state {
		SquareState::Vacant => {
			if piece.piece_type() == &PieceType::Pawn
				&& special_move != &MovementType::PawnMove
			{
				return None;
			}

			MoveType::Move
		},
		SquareState::Opposing(captured) => {
			if piece.piece_type() == &PieceType::Pawn
				&& special_move != &MovementType::PawnCapture
			{
				return None;
			}

			let is_en_passant = special_move == &MovementType::EnPassantCapture;

			MoveType::Capture {
				is_en_passant,
				captured
			}
		},
		SquareState::Friendly => {
			return None;
		}
	};

	Some(MoveInfo {
		piece: *piece,
		initial_position: *initial_position,
		final_position: *final_position,
		move_type
	})
}

fn determine_castle_move(
	initial_position: &Position,
	final_position: &Position,
	piece: &Piece,
	special_move: &MovementType,
	castle_availability: &Res<CastleAvailability>
) -> Option<MoveInfo> {
	if piece.piece_type() != &PieceType::King {
		return None;
	}

	use MovementType::*;
	use PieceColor::*;
	let move_type = match (special_move, piece.piece_color()) {
		(CastleKingside, White) => match castle_availability.white_kingside {
			Some(_) => MoveType::Castle(CastleType::WK),
			None => return None
		},
		(CastleKingside, Black) => match castle_availability.black_kingside {
			Some(_) => MoveType::Castle(CastleType::BK),
			None => return None
		},
		(CastleQueenside, White) => match castle_availability.white_queenside {
			Some(_) => MoveType::Castle(CastleType::WQ),
			None => return None
		},
		(CastleQueenside, Black) => match castle_availability.black_queenside {
			Some(_) => MoveType::Castle(CastleType::BQ),
			None => return None
		},
		_ => return None
	};

	return Some(MoveInfo {
		piece: *piece,
		initial_position: *initial_position,
		final_position: *final_position,
		move_type
	});
}
