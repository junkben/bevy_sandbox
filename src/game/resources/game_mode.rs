use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Resource, Default)]
pub enum GameMode {
	#[default]
	Standard,
	Replay
}
