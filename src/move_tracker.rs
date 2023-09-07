use bevy::prelude::*;

/// Component that tracks how many times something has moved
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MoveTracker(u8);

impl MoveTracker {
    pub fn dec(&mut self) {
        self.0 = match self.0 == u8::MIN {
            true => u8::MAX,
            false => self.0 - 1
        }
    }

    pub fn inc(&mut self) {
        self.0 = match self.0 == u8::MAX {
            true => u8::MIN,
            false => self.0 + 1
        }
    }

    pub fn has_moved(&self) -> bool { self.0 > 0 }

    pub fn move_count(&self) -> u8 { self.0 }
}
