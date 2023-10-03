use bevy::prelude::*;

use crate::{move_info::MoveInfo, piece::Piece, position::Position, MoveType};

#[derive(Resource, Default, Debug)]
pub struct PendingMove {
    pub entity:           Option<Entity>,
    pub piece:            Option<Piece>,
    pub initial_position: Option<Position>,
    pub final_position:   Option<Position>,
    pub move_type:        Option<MoveType>
}

impl PendingMove {
    pub fn confirm(&mut self) -> Option<MoveInfo> {
        let Some(entity) = self.entity else {
            error!("cannot confirm PendingMove: no entity");
            return None;
        };

        let Some(piece) = self.piece else {
            error!("cannot confirm PendingMove: no piece");
            return None;
        };

        let Some(initial_position) = self.initial_position else {
            error!("cannot confirm PendingMove: no initial Position");
            return None;
        };

        let Some(final_position) = self.final_position else {
            error!("cannot confirm PendingMove: no destination Position");
            return None;
        };

        let Some(move_type) = self.move_type else {
            error!("cannot confirm PendingMove: no MoveType");
            return None;
        };

        self.clear();
        Some(MoveInfo {
            entity,
            piece,
            initial_position,
            final_position,
            move_type
        })
    }

    fn clear(&mut self) {
        self.entity = None;
        self.piece = None;
        self.initial_position = None;
        self.final_position = None;
        self.move_type = None;
    }
}
