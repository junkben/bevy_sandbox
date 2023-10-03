use bevy::prelude::*;

use super::TurnState;

pub struct EndTurnPlugin;

impl Plugin for EndTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, end_turn.run_if(in_state(TurnState::End)));
    }
}

fn end_turn(mut turn_state: ResMut<NextState<TurnState>>) {
    trace!("moving to {:?}", TurnState::Start);
    turn_state.set(TurnState::Start);
}
