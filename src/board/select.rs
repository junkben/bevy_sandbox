use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::square::Square;
use crate::piece::{select::SelectedPiece, Piece};

#[derive(Resource, Default)]
pub struct SelectedSquare {
    pub entity: Option<Entity>
}

pub fn select_square(
    event: Listener<Pointer<Click>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &Piece, &mut BoardPosition)>
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the square under the cursor and set it as the selected
    if let Some((square_entity, _intersection)) =
        pick_state.top(Group::default())
    {
        // Get the actual square. This ensures it exists and is a square
        if let Ok(square) = squares_query.get(*square_entity) {
            // Mark it as selected
            selected_square.entity = Some(*square_entity);

            if let Some(selected_piece_entity) = selected_piece.entity {
                // Move the selected piece to the selected square
                if let Ok((_piece_entity, mut piece)) =
                    pieces_query.get_mut(selected_piece_entity)
                {
                    piece.x = square.x;
                    piece.y = square.y;
                }
                selected_square.entity = None;
                selected_piece.entity = None;
            } else {
                // Select the piece in the currently selected square
                for (piece_entity, piece) in pieces_query.iter_mut() {
                    if piece.x == square.x && piece.y == square.y {
                        // piece_entity is now the entity in the same square
                        selected_piece.entity = Some(piece_entity);
                        break;
                    }
                }
            }
        }
    } else {
        // Player clicked outside the board, deselect everything
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}
