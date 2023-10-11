use bevy::prelude::*;

#[derive(Resource, Default)]
pub enum GameMode {
	#[default]
	Standard,
	Replay
}
