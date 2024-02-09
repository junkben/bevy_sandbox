use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::position::Position;

pub struct AttackedPositionsPlugin;
impl Plugin for AttackedPositionsPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(AttackedPositions::default());
	}
}

/// Tracks positions that are under attack by the opposing color this turn
#[derive(Resource, Debug)]
pub struct AttackedPositions(pub HashMap<Position, bool>);

impl Default for AttackedPositions {
	fn default() -> Self {
		Self(
			Position::ALL
				.into_iter()
				.map(|(_, p)| (*p, false))
				.collect::<HashMap<Position, bool>>()
		)
	}
}
