use std::collections::HashMap;

use bevy::prelude::*;

use super::TurnState;
use crate::{
	audio::PlaySoundGameStart,
	game::{
		piece::{SpawnPieces, SpawnPiecesDone, INITIAL_PIECE_POSITIONS},
		position::Position,
		resources::GameMode
	}
};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<GameStart>()
			.add_event::<GameReady>()
			.add_systems(
				Update,
				(
					handle_event_game_start,
					handle_event_game_ready,
					spawning_pieces_done
				)
			);
	}
}

#[derive(Event)]
pub struct GameStart;

fn handle_event_game_start(
	mut er_game_start: EventReader<GameStart>,
	mut ew_spawn_pieces: EventWriter<SpawnPieces>,
	game_mode: Res<GameMode>
) {
	if let Some(_event) = er_game_start.read().last() {
		use GameMode::*;
		match game_mode.as_ref() {
			LocalSinglePlayer => {
				// Spawn pieces in proper squares
				ew_spawn_pieces.send(SpawnPieces(
					INITIAL_PIECE_POSITIONS
						.into_iter()
						.map(|(&key, &piece)| {
							(*Position::ALL.get(key).unwrap(), piece)
						})
						.collect::<HashMap<_, _>>()
				));
			},
			OnlineMultiPlayer => {
				unimplemented!("online multiplayer not yet implemented")
			},
			Replay => unimplemented!("replay functionality not yet implemented")
		}
	}
}

#[derive(Event)]
pub struct GameReady;

fn handle_event_game_ready(
	mut er_game_ready: EventReader<GameStart>,
	mut ew_play_sound: EventWriter<PlaySoundGameStart>,
	mut turn_state: ResMut<NextState<TurnState>>
) {
	if let Some(_event) = er_game_ready.read().last() {
		ew_play_sound.send(PlaySoundGameStart);

		trace!("done spawning pieces, moving to {:?}", TurnState::Start);
		turn_state.set(TurnState::Start);
	}
}

fn spawning_pieces_done(
	mut er_spawn_pieces_done: EventReader<SpawnPiecesDone>,
	mut ew_game_ready: EventWriter<GameReady>
) {
	if let Some(_event) = er_spawn_pieces_done.read().last() {
		ew_game_ready.send(GameReady)
	}
}
