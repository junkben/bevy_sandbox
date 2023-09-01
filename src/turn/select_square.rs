use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{move_piece::PendingMove, TurnState};
use crate::{board::square::Square, position::BoardPosition};

#[derive(Resource, Debug, Default)]
pub struct SelectedBoardPosition(pub Option<BoardPosition>);

pub struct SelectSquarePlugin;

impl Plugin for SelectSquarePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedBoardPosition::default())
            .add_systems(
                OnEnter(TurnState::SelectDestinationSquare),
                enable_square_selection
            )
            .add_systems(
                OnExit(TurnState::SelectDestinationSquare),
                disable_square_selection
            )
            .add_systems(
                Update,
                select_square
                    .run_if(in_state(TurnState::SelectDestinationSquare))
            );
    }
}

pub fn select_square(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut pending_move: ResMut<PendingMove>,
    mut square_query: Query<
        (Option<&PickingInteraction>, &BoardPosition),
        With<Square>
    >
) {
    // Run only if the left mouse was just pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Run through all squares
    for (interaction, board_position) in &mut square_query {
        // Go next if the picking interaction is not pressed
        if interaction != Some(&PickingInteraction::Pressed) {
            continue;
        }

        pending_move.end = Some(*board_position);
        turn_state.set(TurnState::MovePiece);
        break;
    }
}

fn enable_square_selection(
    mut commands: Commands,
    nonpickable_query: Query<Entity, (With<Square>, Without<PickSelection>)>
) {
    // Give Selection components to square entities
    for entity in nonpickable_query.iter() {
        commands
            .entity(entity)
            .insert((PickSelection::default(), RaycastPickTarget::default()));
    }
}

fn disable_square_selection(
    mut commands: Commands,
    pickable_query: Query<Entity, (With<Square>, With<PickSelection>)>
) {
    // Remove Selection components from square entities
    for entity in pickable_query.iter() {
        commands
            .entity(entity)
            .remove::<PickSelection>()
            .remove::<RaycastPickTarget>();
    }
}
