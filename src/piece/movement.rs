use bevy::prelude::*;

use crate::{physics::TranslationalMotionStart, position::Position};

pub struct PieceMovementPlugin;
impl Plugin for PieceMovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<MovePieceToBoardPosition>().add_systems(
			Update,
			move_piece_to_board_position
				.run_if(on_event::<MovePieceToBoardPosition>())
		);
	}
}

#[derive(Event)]
pub struct MovePieceToBoardPosition {
	pub entity:      Entity,
	pub destination: Position
}

/// Turn "move entity to board position" event into a "translation motion start"
/// event
fn move_piece_to_board_position(
	mut events: EventReader<MovePieceToBoardPosition>,
	mut event_writer: EventWriter<TranslationalMotionStart>,
	entities: Query<Entity>
) {
	for event in events.into_iter() {
		let Ok(entity) = entities.get(event.entity) else {
			error!("no matching entity");
			return;
		};

		let destination = event.destination.translation();
		event_writer.send(TranslationalMotionStart {
			entity,
			destination
		});
	}
}

#[derive(Debug, PartialEq)]
pub enum MovementType {
	Move,
	PawnMove,
	PawnCapture,
	EnPassantCapture
}

#[derive(Debug)]
pub struct PieceMovementBehavior(Vec<(Vec3, u8, MovementType)>);

impl PieceMovementBehavior {
	/// Kings can move 1 square vertically, horizontally, or diagonally
	pub fn king() -> PieceMovementBehavior {
		PieceMovementBehavior(vec![
			(N.vec3(), 1, MovementType::Move),
			(NE.vec3(), 1, MovementType::Move),
			(E.vec3(), 1, MovementType::Move),
			(SE.vec3(), 1, MovementType::Move),
			(S.vec3(), 1, MovementType::Move),
			(SW.vec3(), 1, MovementType::Move),
			(W.vec3(), 1, MovementType::Move),
			(NW.vec3(), 1, MovementType::Move),
		])
	}

	/// Knights can either move 1 square horizontally and 2 squares vertically
	/// or move 2 squares horizontally and 1 square vertically
	pub fn knight() -> PieceMovementBehavior {
		PieceMovementBehavior(vec![
			(NNE.vec3(), 1, MovementType::Move),
			(ENE.vec3(), 1, MovementType::Move),
			(ESE.vec3(), 1, MovementType::Move),
			(SSE.vec3(), 1, MovementType::Move),
			(SSW.vec3(), 1, MovementType::Move),
			(WSW.vec3(), 1, MovementType::Move),
			(WNW.vec3(), 1, MovementType::Move),
			(NNW.vec3(), 1, MovementType::Move),
		])
	}

	/// Pawn movement is complicated
	pub fn pawn(color: PieceColor, first_move: bool) -> PieceMovementBehavior {
		let mut move_vec: Vec<(Vec3, u8, MovementType)> = vec![];

		let max_magnitude: u8 = if first_move { 2 } else { 1 };
		match color {
			PieceColor::White => {
				move_vec.push((
					N.vec3(),
					max_magnitude,
					MovementType::PawnMove
				));
				move_vec.push((NE.vec3(), 1, MovementType::PawnCapture));
				move_vec.push((NW.vec3(), 1, MovementType::PawnCapture));
			},
			PieceColor::Black => {
				move_vec.push((
					S.vec3(),
					max_magnitude,
					MovementType::PawnMove
				));
				move_vec.push((SE.vec3(), 1, MovementType::PawnCapture));
				move_vec.push((SW.vec3(), 1, MovementType::PawnCapture));
			}
		};

		PieceMovementBehavior(move_vec)
	}

	/// Queens can move any number of squares vertically, horizontally, or
	/// diagonally
	pub fn queen() -> PieceMovementBehavior {
		PieceMovementBehavior(vec![
			(N.vec3(), u8::MAX, MovementType::Move),
			(NE.vec3(), u8::MAX, MovementType::Move),
			(E.vec3(), u8::MAX, MovementType::Move),
			(SE.vec3(), u8::MAX, MovementType::Move),
			(S.vec3(), u8::MAX, MovementType::Move),
			(SW.vec3(), u8::MAX, MovementType::Move),
			(W.vec3(), u8::MAX, MovementType::Move),
			(NW.vec3(), u8::MAX, MovementType::Move),
		])
	}

	/// Rooks can move any number of squares vertically or horizontally
	pub fn rook() -> PieceMovementBehavior {
		PieceMovementBehavior(vec![
			(N.vec3(), u8::MAX, MovementType::Move),
			(E.vec3(), u8::MAX, MovementType::Move),
			(S.vec3(), u8::MAX, MovementType::Move),
			(W.vec3(), u8::MAX, MovementType::Move),
		])
	}

	/// Bishops can move any number of squares diagonally
	pub fn bishop() -> PieceMovementBehavior {
		PieceMovementBehavior(vec![
			(NE.vec3(), u8::MAX, MovementType::Move),
			(SE.vec3(), u8::MAX, MovementType::Move),
			(SW.vec3(), u8::MAX, MovementType::Move),
			(NW.vec3(), u8::MAX, MovementType::Move),
		])
	}

	pub fn iter<'a>(
		&'a self
	) -> impl Iterator<Item = &'a (Vec3, u8, MovementType)> + 'a {
		self.0.iter()
	}
}

// TODO: Castling behavior for king and rook
enum Direction {
	N,
	NNE,
	NE,
	ENE,
	E,
	ESE,
	SE,
	SSE,
	S,
	SSW,
	SW,
	WSW,
	W,
	WNW,
	NW,
	NNW
}

use Direction::*;

use super::PieceColor;

impl Direction {
	pub const fn vec3(&self) -> Vec3 {
		let (x, z) = match self {
			N => (0.0, 1.0),
			NNE => (1.0, 2.0),
			NE => (1.0, 1.0),
			ENE => (2.0, 1.0),
			E => (1.0, 0.0),
			ESE => (2.0, -1.0),
			SE => (1.0, -1.0),
			SSE => (1.0, -2.0),
			S => (0.0, -1.0),
			SSW => (-1.0, -2.0),
			SW => (-1.0, -1.0),
			WSW => (-2.0, -1.0),
			W => (0.0, -1.0),
			WNW => (-2.0, 1.0),
			NW => (-1.0, 1.0),
			NNW => (-1.0, 2.0)
		};

		Vec3::new(x, 0.0, z)
	}
}
