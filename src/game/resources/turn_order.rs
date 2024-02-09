use bevy::prelude::*;

use crate::game::{player::Player, team::TeamColor};

/// Tracks whose turn it is. White always goes first.
#[derive(Resource)]
pub struct TurnOrder {
	index:   usize,
	players: Vec<Player>
}

impl Default for TurnOrder {
	fn default() -> Self {
		use TeamColor::*;
		Self {
			index:   0,
			players: vec![Player::new(White), Player::new(Black)]
		}
	}
}

impl TurnOrder {
	pub fn next(&mut self) -> Player {
		self.index = (self.index + 1) % self.players.len();
		self.players[self.index]
	}

	pub fn current(&self) -> Player { self.players[self.index] }
}
