use bevy::prelude::*;

use crate::position::Position;

/// Tracks whether or not there's a target for an en passant capture
#[derive(Resource, Default)]
pub struct EnPassantTracker(Option<Position>);
