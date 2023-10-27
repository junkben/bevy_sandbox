use bevy::prelude::*;

use super::TurnState;
use crate::game::resources::{ActiveColor, HalfmoveTracker};

pub struct UpdateBoardStatePlugin;

impl Plugin for UpdateBoardStatePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			OnEnter(TurnState::UpdateBoardState),
			update_board_state
		);
	}
}

fn update_board_state(
	mut turn_state: ResMut<NextState<TurnState>>,
	mut active_color: ResMut<ActiveColor>,
	mut halfmove_tracker: ResMut<HalfmoveTracker>
) {
	// Increment halfmoves
	halfmove_tracker.as_mut().inc();

	// Switch active color
	active_color.as_mut().next();

	debug!("moving to {:?}", TurnState::End);
	turn_state.set(TurnState::End);
}
