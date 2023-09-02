use bevy::prelude::*;

use super::TurnState;
use crate::resources::BoardState;

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
    mut board_state: ResMut<BoardState>
) {
    board_state.next_turn();

    debug!("moving to {:?}", TurnState::End);
    turn_state.set(TurnState::End);
}
