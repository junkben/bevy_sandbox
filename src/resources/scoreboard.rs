use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub white_wins: u32,
    pub black_wins: u32
}
