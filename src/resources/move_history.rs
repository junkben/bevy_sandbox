use bevy::prelude::*;

use crate::move_info::MoveInfo;

#[derive(Resource, Default, Debug)]
pub struct MoveHistory(Vec<MoveInfo>);

impl MoveHistory {
    pub fn append_move(&mut self, move_info: MoveInfo) {
        self.0.push(move_info);
        debug!(?self)
    }
}
