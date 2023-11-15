use std::collections::HashSet;

use bevy::prelude::*;

use super::{ActiveColor, AvailableMoves};
use crate::game::position::Position;

pub struct AttackedPositionsPlugin;
impl Plugin for AttackedPositionsPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(AttackedPositions::default())
			.add_event::<UpdateAttackedPositions>()
			.add_event::<UpdateAttackedPositionsDone>()
			.add_systems(
				Update,
				handle_event.run_if(on_event::<UpdateAttackedPositions>())
			);
	}
}

#[derive(Event)]
pub struct UpdateAttackedPositions;

#[derive(Event)]
pub struct UpdateAttackedPositionsDone;

/// Tracks positions that are under attack by the opposing color this turn
#[derive(Resource, Default, Debug)]
pub struct AttackedPositions(pub HashSet<Position>);

fn handle_event(
	mut commands: Commands,
	mut er_do_update: EventReader<UpdateAttackedPositions>,
	mut ew_update_done: EventWriter<UpdateAttackedPositionsDone>,
	res_available_moves: Res<AvailableMoves>,
	res_active_color: Res<ActiveColor>
) {
	// Consume UpdateAttackedPositions
	let Some(_event) = er_do_update.into_iter().last() else {
		error!("not exactly one UpdateAttackedPositions event");
		return;
	};

	let attacked_positions = determine_attacked_positions(
		res_available_moves.as_ref(),
		res_active_color.as_ref()
	);

	debug!(?attacked_positions);
	commands.insert_resource(attacked_positions);
	ew_update_done.send(UpdateAttackedPositionsDone);
}

fn determine_attacked_positions(
	available_moves: &AvailableMoves,
	active_color: &ActiveColor
) -> AttackedPositions {
	let mut attacked_positions: HashSet<Position> = HashSet::new();

	for m in available_moves.all_moves() {
		// If the move's piece is the same color as the active one, that's not
		// an attack
		if m.piece.piece_color() == &active_color.0 {
			continue;
		}

		// If the move_type is not an attack, continue looking
		if !m.move_type.is_attack() {
			continue;
		}

		attacked_positions.insert(m.final_position);
	}

	AttackedPositions(attacked_positions)
}
