use bevy::prelude::*;

use crate::move_info::MoveInfo;

#[derive(Resource, Default, Debug)]
pub struct MoveHistory(Vec<MoveInfo>);

impl std::fmt::Display for MoveHistory {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, moves) in self.0[..].chunks(2).enumerate() {
			// Start on newline, write turn number
			write!(f, "\n{}. ", i + 1)?;

			// Write White move
			write!(f, "{} ", moves[0])?;

			// Conditionally write Black move if it's there
			if moves.len() == 2 {
				write!(f, "{} ", moves[1])?;
			}
		}

		Ok(())
	}
}

impl MoveHistory {
	pub fn append_move(&mut self, move_info: MoveInfo) {
		self.0.push(move_info);
		info!("{}", self)
	}
}
