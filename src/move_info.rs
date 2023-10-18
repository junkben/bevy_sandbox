use bevy::prelude::*;

use crate::{
	piece::{Piece, PieceType},
	position::Position,
	resources::CastleType
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum MoveType {
	/// A piece is moved to an unoccupied square
	#[default]
	Move,

	/// A pawn can move differently if it hasn't moved yet
	FirstMove,

	/// A piece is moved to a space occupied by an opponent's piece, which is
	/// captured and removed from play. With the sole exception of en passant,
	/// all pieces capture by moving to the square that the opponent's piece
	/// occupies.
	Capture {
		captured: Entity
	},

	CaptureEnPassant {
		captured: Entity
	},

	Castle(CastleType)
}

#[derive(Debug, Copy, Clone)]
pub struct MoveInfo {
	pub entity:           Entity,
	pub piece:            Piece,
	pub initial_position: Position,
	pub final_position:   Position,
	pub move_type:        MoveType,
	pub promoted_to:      Option<Piece>,
	pub is_check:         bool,
	pub is_checkmate:     bool,
	pub draw_offered:     bool
}

impl std::fmt::Display for MoveInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let (p, fp) = (self.piece, self.final_position);
		use MoveType::*;
		write!(f, "{}", match self.move_type {
			Move | FirstMove => match p.piece_type() {
				PieceType::Pawn => format!("{}", fp),
				_ => format!("{}{}", p, fp)
			},
			Capture { .. } => format!("{}x{}", p, fp),
			CaptureEnPassant { .. } => format!("{}x{} e.p.", p, fp),
			Castle(castle_type) => match castle_type {
				CastleType::WK | CastleType::BK => format!("0-0"),
				CastleType::WQ | CastleType::BQ => format!("0-0-0")
			}
		})?;

		if let Some(piece) = self.promoted_to {
			write!(f, "{piece}")?;
		}

		if self.is_checkmate {
			write!(f, "#")?;
		} else {
			if self.is_check {
				write!(f, "+")?;
			}

			if self.draw_offered {
				write!(f, "=")?;
			}
		}

		Ok(())
	}
}

impl MoveType {
	pub fn is_attack(&self) -> bool {
		use MoveType::*;
		match self {
			Move => true,
			FirstMove => false,
			Capture { .. } => true,
			CaptureEnPassant { .. } => true,
			Castle(_) => false
		}
	}
}
