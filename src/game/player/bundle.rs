use bevy::prelude::*;

use super::{AttackedPositions, AvailableMoves, Player};

#[derive(Bundle)]
pub struct PlayerBundle {
	player:             Player,
	moves:              AvailableMoves,
	attacked_positions: AttackedPositions
}
