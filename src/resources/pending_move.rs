use bevy::prelude::*;

use crate::{piece::Piece, position::Position};

#[derive(Resource, Default, Debug)]
pub struct PendingMove {
    pub entity:      Option<Entity>,
    pub piece:       Option<Piece>,
    pub destination: Option<Position>
}

impl PendingMove {
    pub fn confirm(&mut self) -> Option<(Entity, Piece, Position)> {
        let Some(entity) = self.entity else {
            error!("cannot confirm PendingMove: no entity");
            return None;
        };

        let Some(piece) = self.piece else {
            error!("cannot confirm PendingMove: no piece");
            return None;
        };

        let Some(destination) = self.destination else {
            error!("cannot confirm PendingMove: no destination Position");
            return None;
        };

        self.clear();
        Some((entity, piece, destination))
    }

    fn clear(&mut self) {
        self.entity = None;
        self.piece = None;
        self.destination = None;
    }
}
