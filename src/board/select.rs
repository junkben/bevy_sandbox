use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{position::BoardPosition, square::Square};

#[derive(Resource, Default)]
pub struct SelectedSquare {
    pub entity: Option<Entity>
}

pub fn select_square(
    mut selected_square: ResMut<SelectedSquare>,
    mut square_query: Query<(
        Option<&PickingInteraction>,
        &mut Square,
        &BoardPosition,
        Entity
    )>
) {
    for (interaction_opt, mut _square, board_position, entity) in
        &mut square_query
    {
        if interaction_opt != Some(&PickingInteraction::Pressed) {
            continue;
        }

        if selected_square.entity != Some(entity) {
            selected_square.entity = Some(entity);
            info!("selected square at {board_position}")
        }
    }
}
