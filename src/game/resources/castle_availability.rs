use bevy::prelude::*;

use crate::game::{
	move_tracker::MoveTracker, piece::Piece, position::Position
};

pub struct CastleAvailabilityPlugin;
impl Plugin for CastleAvailabilityPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(CastleAvailability::default())
			.add_event::<CheckCastleAvailability>()
			.add_event::<CheckCastleAvailabilityDone>()
			.add_systems(
				Update,
				handle_event.run_if(on_event::<CheckCastleAvailability>())
			);
	}
}

#[derive(Event)]
pub struct CheckCastleAvailability;

#[derive(Event)]
pub struct CheckCastleAvailabilityDone;

fn handle_event(
	mut commands: Commands,
	mut er: EventReader<CheckCastleAvailability>,
	mut ew: EventWriter<CheckCastleAvailabilityDone>,
	query_piece: Query<(Entity, &Piece, &Position, &MoveTracker)>
) {
	// Consume CheckCastleAvailability
	let Some(_) = er.into_iter().last() else {
		error!("not exactly one CheckCastleAvailability event");
		return;
	};

	let castle_availability = determine_castle_availability(query_piece);

	debug!(?castle_availability);
	commands.insert_resource(castle_availability);
	ew.send(CheckCastleAvailabilityDone)
}

pub fn determine_castle_availability(
	query_piece: Query<(Entity, &Piece, &Position, &MoveTracker)>
) -> CastleAvailability {
	use CastleType::*;
	CastleAvailability {
		white_kingside:  WK.check_can_castle(&query_piece),
		white_queenside: WQ.check_can_castle(&query_piece),
		black_kingside:  BK.check_can_castle(&query_piece),
		black_queenside: BQ.check_can_castle(&query_piece)
	}
}

#[derive(Debug)]
pub struct CastleEntities {
	pub king:             Entity,
	pub rook:             Entity,
	pub rook_destination: Position
}

/// Tracks what players can castle and to what side
#[derive(Resource, Debug)]
pub struct CastleAvailability {
	pub white_kingside:  Option<CastleEntities>,
	pub white_queenside: Option<CastleEntities>,
	pub black_kingside:  Option<CastleEntities>,
	pub black_queenside: Option<CastleEntities>
}

impl CastleAvailability {}

impl std::fmt::Display for CastleAvailability {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.white_kingside.is_some() {
			write!(f, "K")?;
		}
		if self.white_queenside.is_some() {
			write!(f, "Q")?;
		}
		if self.black_kingside.is_some() {
			write!(f, "k")?;
		}
		if self.black_queenside.is_some() {
			write!(f, "q")?;
		}

		Ok(())
	}
}

impl Default for CastleAvailability {
	fn default() -> Self {
		Self {
			white_kingside:  None,
			white_queenside: None,
			black_kingside:  None,
			black_queenside: None
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleType {
	/// White Kingside
	WK,
	/// White Queenside
	WQ,
	/// Black Kingside
	BK,
	/// Black Queenside
	BQ
}

use CastleType::*;

impl CastleType {
	fn gap_positions<'a>(&'a self) -> impl Iterator<Item = &'a Position> + 'a {
		match self {
			WK => [Position::F1, Position::G1].iter(),
			WQ => [Position::B1, Position::C1, Position::D1].iter(),
			BK => [Position::F8, Position::G8].iter(),
			BQ => [Position::B8, Position::C8, Position::D8].iter()
		}
	}

	fn rook_position(&self) -> &Position {
		match self {
			WK => &Position::H1,
			WQ => &Position::A1,
			BK => &Position::H8,
			BQ => &Position::A8
		}
	}

	fn rook_destination(&self) -> &Position {
		match self {
			WK => &Position::F1,
			WQ => &Position::D1,
			BK => &Position::F8,
			BQ => &Position::D8
		}
	}

	fn rook_piece(&self) -> &Piece {
		match self {
			WK | WQ => &Piece::WHITE_ROOK,
			BK | BQ => &Piece::BLACK_ROOK
		}
	}

	fn king_position(&self) -> &Position {
		match self {
			WK | WQ => &Position::E1,
			BK | BQ => &Position::E8
		}
	}

	fn king_piece(&self) -> &Piece {
		match self {
			WK | WQ => &Piece::WHITE_KING,
			BK | BQ => &Piece::BLACK_KING
		}
	}

	fn rook_entity(
		&self,
		query_piece: &Query<(Entity, &Piece, &Position, &MoveTracker)>
	) -> Option<Entity> {
		for (entity, piece, position, _) in query_piece.iter() {
			if piece == self.rook_piece() && position == self.rook_position() {
				return Some(entity);
			}
		}

		return None;
	}

	fn king_entity(
		&self,
		query_piece: &Query<(Entity, &Piece, &Position, &MoveTracker)>
	) -> Option<Entity> {
		for (entity, piece, position, _) in query_piece.iter() {
			if piece == self.king_piece() && position == self.king_position() {
				return Some(entity);
			}
		}

		return None;
	}

	fn king_has_moved(
		&self,
		query_piece: &Query<(Entity, &Piece, &Position, &MoveTracker)>
	) -> Option<bool> {
		for (_, piece, position, move_tracker) in query_piece.iter() {
			if piece == self.king_piece() && position == self.king_position() {
				return Some(move_tracker.has_moved());
			}
		}

		return None;
	}

	fn rook_has_moved(
		&self,
		query_piece: &Query<(Entity, &Piece, &Position, &MoveTracker)>
	) -> Option<bool> {
		for (_, piece, position, move_tracker) in query_piece.iter() {
			if piece == self.rook_piece() && position == self.rook_position() {
				return Some(move_tracker.has_moved());
			}
		}

		return None;
	}

	fn path_is_clear(
		&self,
		query_piece: &Query<(Entity, &Piece, &Position, &MoveTracker)>
	) -> bool {
		for (_, _, position, _) in query_piece.iter() {
			if self.gap_positions().find(|&p| p == position).is_some() {
				return false;
			}
		}

		return true;
	}

	fn check_can_castle(
		&self,
		query_piece: &Query<(Entity, &Piece, &Position, &MoveTracker)>
	) -> Option<CastleEntities> {
		let rook_moved = self.rook_has_moved(query_piece)?;
		let king_moved = self.king_has_moved(query_piece)?;
		let path_is_clear = self.path_is_clear(query_piece);

		match !rook_moved && !king_moved && path_is_clear {
			true => Some(CastleEntities {
				king:             self.king_entity(query_piece)?,
				rook:             self.rook_entity(query_piece)?,
				rook_destination: self.rook_destination().clone()
			}),
			false => None
		}
	}
}
