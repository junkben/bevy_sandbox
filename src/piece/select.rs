use bevy::prelude::*;
use bevy_mod_picking::{prelude::RaycastPickTarget, selection::PickSelection};

use super::Piece;
use crate::{
    piece::color::PieceColor,
    resources::{GameStage, GameState}
};

#[derive(Resource, Default)]
pub struct SelectedPiece {
    pub entity: Option<Entity>
}

pub fn toggle_piece_selectability(
    mut commands: Commands,
    game_state: Res<GameState>,
    pickable_query: Query<(Entity, &Piece), With<PickSelection>>,
    nonpickable_query: Query<(Entity, &Piece), Without<PickSelection>>
) {
    if !game_state.is_changed() {
        return;
    }

    use GameStage::*;
    use PieceColor::*;
    let stage = &game_state.stage;
    for (entity, piece) in pickable_query.iter() {
        if (stage != &TurnBlack && piece.color == Black)
            || (stage != &TurnWhite && piece.color == White)
        {
            commands
                .entity(entity)
                .remove::<PickSelection>()
                .remove::<RaycastPickTarget>();
        }
    }

    for (entity, piece) in nonpickable_query.iter() {
        if (stage == &TurnBlack && piece.color == Black)
            || (stage == &TurnWhite && piece.color == White)
        {
            commands.entity(entity).insert((
                PickSelection::default(),
                RaycastPickTarget::default()
            ));
        }
    }
}
