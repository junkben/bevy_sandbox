use std::collections::HashMap;

use bevy::prelude::*;

use super::{CastleAvailability, EnPassantTracker};
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
	en_passant_tracker: Res<EnPassantTracker>,
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
				PieceMovementBehavior::pawn(piece_color)
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
					entity,
					&initial_position,
					&final_position,
					&piece,
					&move_tracker,
					special_move,
					&castle_availability,
					&en_passant_tracker,
					&opposing_piece_query
				);

				if let Some(move_info) = move_info_opt {
					if let MoveType::Capture { captured: _ } =
						move_info.move_type
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
	entity: Entity,
	initial_position: &Position,
	final_position: &Position,
	piece: &Piece,
	move_tracker: &MoveTracker,
	special_move: &MovementType,
	castle_availability: &Res<CastleAvailability>,
	en_passant_tracker: &Res<EnPassantTracker>,
	other_piece_query: &Query<(Entity, &Position, &Piece)>
) -> Option<MoveInfo> {
	if special_move == &MovementType::CastleKingside
		|| special_move == &MovementType::CastleQueenside
	{
		return determine_castle_move(
			entity,
			initial_position,
			final_position,
			piece,
			special_move,
			castle_availability
		);
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

	if piece.piece_type() == &PieceType::Pawn {
		return determine_pawn_move(
			entity,
			initial_position,
			final_position,
			piece,
			move_tracker,
			special_move,
			en_passant_tracker,
			&square_state
		);
	}

	use SquareState::*;
	let move_type_opt = match square_state {
		Vacant => Some(MoveType::Move),
		Opposing(captured) => Some(MoveType::Capture { captured }),
		Friendly => None
	};

	return Some(MoveInfo {
		entity,
		piece: *piece,
		initial_position: *initial_position,
		final_position: *final_position,
		move_type: move_type_opt?
	});
}

fn determine_pawn_move(
	entity: Entity,
	initial_position: &Position,
	final_position: &Position,
	piece: &Piece,
	move_tracker: &MoveTracker,
	special_move: &MovementType,
	en_passant_tracker: &Res<EnPassantTracker>,
	square_state: &SquareState
) -> Option<MoveInfo> {
	if piece.piece_type() != &PieceType::Pawn {
		return None;
	}

	use EnPassantTracker::*;
	use MovementType::*;
	use SquareState::*;
	let move_type = match (square_state, special_move) {
		(Vacant, PawnMove) => Some(MoveType::Move),
		(Vacant, PawnFirstMove) => match move_tracker.has_moved() {
			true => None,
			false => Some(MoveType::FirstMove)
		},
		(Vacant, EnPassantCapture) => match en_passant_tracker.as_ref() {
			&Unavailable => None,
			&Available { position, captured } => {
				match &position == final_position {
					true => Some(MoveType::CaptureEnPassant { captured }),
					false => None
				}
			}
		},
		(&Opposing(captured), PawnCapture) => {
			Some(MoveType::Capture { captured })
		},
		(..) => None
	}?;

	return Some(MoveInfo {
		entity,
		piece: *piece,
		initial_position: *initial_position,
		final_position: *final_position,
		move_type
	});
}

fn determine_castle_move(
	entity: Entity,
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
		entity,
		piece: *piece,
		initial_position: *initial_position,
		final_position: *final_position,
		move_type
	});
}
