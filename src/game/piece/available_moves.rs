use std::collections::HashMap;

use bevy::prelude::*;

use super::{AttackedPositions, CastleAvailability, EnPassantState};
use crate::game::{
	move_info::MoveInfo,
	move_tracker::MoveTracker,
	piece::{MovementType, Piece, PieceMovementBehavior, PieceType},
	position::*,
	resources::CastleType,
	team::TeamColor,
	MoveType
};

const BLACK_PAWN_PROMOTION_RANK: Rank = Rank::ONE;
const WHITE_PAWN_PROMOTION_RANK: Rank = Rank::EIGHT;

pub struct AvailableMovesPlugin;
impl Plugin for AvailableMovesPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(AvailableMoves::default())
			.add_event::<CalculateAvailableMoves>()
			.add_event::<CalculateAvailableMovesDone>()
			.add_systems(
				Update,
				handle_event.run_if(on_event::<CalculateAvailableMoves>())
			);
	}
}

#[derive(Event)]
pub struct CalculateAvailableMoves;

#[derive(Event)]
pub struct CalculateAvailableMovesDone;

fn handle_event(
	mut commands: Commands,
	mut er_calculate_moves: EventReader<CalculateAvailableMoves>,
	mut ew_calculate_done: EventWriter<CalculateAvailableMovesDone>,
	res_castle_availability: Res<CastleAvailability>,
	res_en_passant_tracker: Res<EnPassantState>,
	query_piece: Query<(Entity, &Piece, &Position, &MoveTracker)>,
	query_opposing_piece: Query<(Entity, &Position, &Piece)>
) {
	// Consume CalculateAvailableMoves
	let Some(_event) = er_calculate_moves.read().last() else {
		error!("not exactly one CalculateAvailableMoves event");
		return;
	};

	info!("Calculating available moves...");

	let (am, ap) = calculate_available_moves_and_attacked_positions(
		res_castle_availability.as_ref(),
		res_en_passant_tracker.as_ref(),
		query_piece,
		query_opposing_piece
	);

	commands.insert_resource(am);
	debug!("{:?}", ap);
	commands.insert_resource(ap);
	ew_calculate_done.send(CalculateAvailableMovesDone)
}

/// A component that tracks the available positions an entity can move to
#[derive(Component, Default, Debug, Clone)]
pub struct AvailableMoves(pub HashMap<Entity, Vec<MoveInfo>>);

impl std::fmt::Display for AvailableMoves {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

#[allow(dead_code)]
impl AvailableMoves {
	pub fn get(&self, k: &Entity) -> Option<&Vec<MoveInfo>> { self.0.get(k) }

	pub fn all_moves(&self) -> impl Iterator<Item = &MoveInfo> {
		self.0.values().flatten()
	}

	pub fn contains_move_to(&self, position: &Position) -> bool {
		for m in self.0.values().flatten() {
			if &m.final_position == position {
				return true;
			}
		}

		return false;
	}

	pub fn entity_has_move_to(
		&self,
		entity: Entity,
		position: &Position
	) -> bool {
		if let Some(moves) = self.0.get(&entity) {
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

fn calculate_available_moves_and_attacked_positions(
	castle_availability: &CastleAvailability,
	en_passant_tracker: &EnPassantState,
	mut query_piece: Query<(Entity, &Piece, &Position, &MoveTracker)>,
	query_opposing_piece: Query<(Entity, &Position, &Piece)>
) -> (AvailableMoves, AttackedPositions) {
	let mut am = AvailableMoves::default();
	let mut ap = AttackedPositions::default();

	for (entity, piece, initial_position, move_tracker) in
		query_piece.iter_mut()
	{
		let (moves, attacked_positions) =
			calculate_moves_and_attack_positions_for_piece(
				entity,
				piece,
				initial_position,
				move_tracker,
				castle_availability,
				en_passant_tracker,
				&query_opposing_piece
			);
		am.0.insert(entity, moves);
		for position in attacked_positions {
			ap.0.insert(position, true);
		}
	}

	(am, ap)
}

fn calculate_moves_and_attack_positions_for_piece(
	entity: Entity,
	piece: &Piece,
	initial_position: &Position,
	move_tracker: &MoveTracker,
	castle_availability: &CastleAvailability,
	en_passant_tracker: &EnPassantState,
	query_opposing_piece: &Query<(Entity, &Position, &Piece)>
) -> (Vec<MoveInfo>, Vec<Position>) {
	let mut moves: Vec<MoveInfo> = Vec::new();
	let mut attacked_positions: Vec<Position> = Vec::new();

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

	for (direction, max_magnitude, special_move) in movement_patterns.iter() {
		let mut magnitude: u8 = 0u8;

		while magnitude < *max_magnitude {
			magnitude += 1;

			let vector = start_vec + (direction.clone() * magnitude as f32);

			// If the proposed Position can't exist, break
			let final_position = match Position::try_from_vec3(vector) {
				Some(bp) => bp,
				None => break
			};

			let square_state = determine_square_state(
				piece,
				initial_position,
				&final_position,
				&query_opposing_piece
			);

			// Check if there is a piece at the end position. If there
			// is, we'll record it's color
			let move_info_opt = determine_move(
				entity,
				&initial_position,
				&final_position,
				&piece,
				&move_tracker,
				special_move,
				castle_availability,
				en_passant_tracker,
				&square_state
			);

			if let Some(move_info) = move_info_opt {
				if let MoveType::Capture { captured: _ } = move_info.move_type {
					magnitude = *max_magnitude;
				}

				// Add move to possible moves
				moves.push(move_info);

				// Add attacked positions
				if move_info.is_attack() {
					attacked_positions.push(move_info.final_position)
				}
			} else {
				break;
			}
		} // end while
	} // end for

	trace!(?piece, ?initial_position, ?moves);
	(moves, attacked_positions)
}

fn determine_square_state(
	piece: &Piece,
	initial_position: &Position,
	final_position: &Position,
	query_other_piece: &Query<(Entity, &Position, &Piece)>
) -> SquareState {
	for (o_entity, o_position, o_piece) in query_other_piece.iter() {
		if o_position != final_position || o_position == initial_position {
			continue;
		}

		return match piece.piece_color() == o_piece.piece_color() {
			true => SquareState::Friendly,
			false => SquareState::Opposing(o_entity)
		};
	}

	SquareState::Vacant
}

fn determine_move(
	entity: Entity,
	initial_position: &Position,
	final_position: &Position,
	piece: &Piece,
	move_tracker: &MoveTracker,
	special_move: &MovementType,
	castle_availability: &CastleAvailability,
	en_passant_tracker: &EnPassantState,
	square_state: &SquareState
) -> Option<MoveInfo> {
	// Handle move type
	let move_type = determine_move_type(
		final_position,
		piece,
		move_tracker,
		special_move,
		castle_availability,
		en_passant_tracker,
		square_state
	)?;

	// Handle promotion
	let promoted_to = determine_promoted_to(final_position, piece);

	// Return move
	return Some(MoveInfo {
		entity,
		piece: *piece,
		initial_position: *initial_position,
		final_position: *final_position,
		move_type,
		promoted_to,
		is_check: false,
		is_checkmate: false,
		draw_offered: false
	});
}

fn determine_move_type(
	final_position: &Position,
	piece: &Piece,
	move_tracker: &MoveTracker,
	special_move: &MovementType,
	castle_availability: &CastleAvailability,
	en_passant_tracker: &EnPassantState,
	square_state: &SquareState
) -> Option<MoveType> {
	use MovementType::*;
	use PieceType::*;
	use SquareState::*;
	match (piece.piece_type(), special_move, square_state) {
		// Handle Pawn moves
		(Pawn, PawnMove, Vacant) => Some(MoveType::Move),
		(Pawn, PawnFirstMove, Vacant) => match move_tracker.has_moved() {
			true => None,
			false => Some(MoveType::FirstMove)
		},
		(Pawn, EnPassantCapture, Vacant) => match en_passant_tracker {
			&EnPassantState::Unavailable => None,
			&EnPassantState::Available { position, captured } => {
				match &position == final_position {
					true => Some(MoveType::CaptureEnPassant { captured }),
					false => None
				}
			},
		},
		(Pawn, PawnCapture, Opposing(captured)) => Some(MoveType::Capture {
			captured: *captured
		}),
		// Handle King moves
		(King, CastleKingside, _) => match piece.piece_color() {
			TeamColor::White => match castle_availability.white_kingside {
				Some(_) => Some(MoveType::Castle(CastleType::WK)),
				None => None
			},
			TeamColor::Black => match castle_availability.black_kingside {
				Some(_) => Some(MoveType::Castle(CastleType::BK)),
				None => None
			}
		},
		(King, CastleQueenside, _) => match piece.piece_color() {
			TeamColor::White => match castle_availability.white_queenside {
				Some(_) => Some(MoveType::Castle(CastleType::WQ)),
				None => None
			},
			TeamColor::Black => match castle_availability.black_queenside {
				Some(_) => Some(MoveType::Castle(CastleType::BQ)),
				None => None
			}
		},
		// Handle other moves
		(_, Move, Vacant) => Some(MoveType::Move),
		(_, Move, &Opposing(captured)) => Some(MoveType::Capture { captured }),
		(_, Move, Friendly) => None,
		_ => None
	}
}

fn determine_promoted_to(
	final_position: &Position,
	piece: &Piece
) -> Option<Piece> {
	if piece.piece_type() != &PieceType::Pawn {
		return None;
	}

	// TODO: don't default to queen
	match piece
		.piece_color()
		.is_at_promotion_rank(final_position.rank())
	{
		true => Some(Piece::new(*piece.piece_color(), PieceType::QUEEN)),
		false => None
	}
}
