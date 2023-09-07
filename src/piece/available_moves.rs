use bevy::prelude::*;

use crate::position::*;

/// A component that tracks the available positions an entity can move to
#[derive(Component, Default, Debug)]
pub struct AvailableMoves(pub Vec<Position>);

impl std::fmt::Display for AvailableMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
