use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::square::Square;

#[derive(Resource, Debug, Default)]
pub struct SelectedSquare {
    pub entity: Option<Entity>
}

#[derive(Resource, Debug, Default)]
pub struct SelectedPiece {
    pub entity: Option<Entity>
}

pub fn select_square(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    // mut selected_piece: ResMut<SelectedPiece>,
    mut square_query: Query<
        (Option<&PickingInteraction>, Entity),
        With<Square>
    > /* mut piece_query: Query<(Option<&PickingInteraction>, Entity),
       * With<Piece>> */
) {
    // Run only if the left mouse was just pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Run through all squares
    for (interaction, entity) in &mut square_query {
        // Go next if the picking interaction is not pressed
        if interaction != Some(&PickingInteraction::Pressed) {
            continue;
        }

        selected_square.entity = Some(entity);
        break;
    }
}
