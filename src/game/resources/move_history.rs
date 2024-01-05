use bevy::prelude::*;

use crate::game::move_info::MoveInfo;

#[derive(Resource, Default, Debug)]
pub struct MoveHistory(Vec<MoveInfo>);

impl std::fmt::Display for MoveHistory {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, moves) in self.0[..].chunks(2).enumerate() {
			// Start on newline, write turn number
			write!(f, "{}.", i + 1)?;

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
	pub fn len(&self) -> usize { self.0.len() }

	pub fn get(&self, index: usize) -> Option<&MoveInfo> { self.0.get(index) }

	pub fn last(&self) -> Option<&MoveInfo> { self.get(self.len() - 1) }

	pub fn append_move(&mut self, move_info: MoveInfo) {
		self.0.push(move_info);
		info!("{}", self)
	}

	pub fn latest_move(&self) -> Option<&MoveInfo> {
		match self.len() {
			0 => None,
			_ => self.get(self.len() - 1)
		}
	}
}
