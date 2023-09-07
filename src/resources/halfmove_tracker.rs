use bevy::prelude::*;

/// The number of moves both players have made since the last pawn advance
/// or piece capture. Used to enforce the 50-move draw rule, where the
/// game ends in a draw after 100 half-moves
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalfmoveTracker(u32);

impl HalfmoveTracker {
    pub fn dec(&mut self) {
        self.0 = match self.0 == u32::MIN {
            true => u32::MAX,
            false => self.0 - 1
        }
    }

    pub fn inc(&mut self) {
        self.0 = match self.0 == u32::MAX {
            true => u32::MIN,
            false => self.0 + 1
        }
    }

    pub fn has_moved(&self) -> bool { self.0 > 0 }

    pub fn halfmoves(&self) -> u32 { self.0 }

    pub fn turns_completed(&self) -> u32 {
        (self.0 as f32 / 2.0).floor() as u32
    }
}
