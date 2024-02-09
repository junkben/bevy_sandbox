mod attacked_positions;
mod available_moves;
mod bundle;

pub use attacked_positions::AttackedPositions;
pub use available_moves::{
	AvailableMoves, CalculateAvailableMoves, CalculateAvailableMovesDone
};
use bevy::prelude::*;

use self::{
	attacked_positions::AttackedPositionsPlugin,
	available_moves::AvailableMovesPlugin
};
use super::team::TeamColor;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((AttackedPositionsPlugin, AvailableMovesPlugin));
	}
}

/// A component that represents a Player
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player(TeamColor);

impl Player {
	pub fn new(team_color: TeamColor) -> Player { Player(team_color) }

	pub fn team_color(&self) -> TeamColor { self.0 }

	pub fn set_team_color(&mut self, value: TeamColor) { self.0 = value }
}
