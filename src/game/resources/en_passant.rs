use bevy::prelude::*;

use super::MoveHistory;
use crate::game::{
	piece::PieceColor,
	position::{File, Position},
	MoveType
};

pub struct EnPassantPlugin;
impl Plugin for EnPassantPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(EnPassantState::default())
			.add_event::<CheckEnPassant>()
			.add_event::<CheckEnPassantDone>()
			.add_systems(
				Update,
				handle_event.run_if(on_event::<CheckEnPassant>())
			);
	}
}

#[derive(Event)]
pub struct CheckEnPassant;

#[derive(Event)]
pub struct CheckEnPassantDone;

/// Tracks whether or not there's a target for an en passant capture
#[derive(Resource, Default, Debug)]
pub enum EnPassantState {
	#[default]
	Unavailable,
	Available {
		position: Position,
		captured: Entity
	}
}

fn handle_event(
	mut commands: Commands,
	mut er_do_check: EventReader<CheckEnPassant>,
	mut ew_check_done: EventWriter<CheckEnPassantDone>,
	res_move_history: Res<MoveHistory>
) {
	// Consume CheckEnPassant
	let Some(_event) = er_do_check.read().last() else {
		error!("not exactly one CheckEnPassant event");
		return;
	};

	let en_passant_state =
		determine_en_passant_state(res_move_history.as_ref());

	debug!(?en_passant_state);
	commands.insert_resource(en_passant_state);
	ew_check_done.send(CheckEnPassantDone);
}

fn determine_en_passant_state(move_history: &MoveHistory) -> EnPassantState {
	// If there hasn't been a move yet, then there's no en passant
	let Some(latest_move) = move_history.latest_move() else {
		return EnPassantState::Unavailable;
	};

	// If the latest move wasn't a first move, then there's no en passant
	if latest_move.move_type != MoveType::FirstMove {
		return EnPassantState::Unavailable;
	}

	// Determine the position that needs to be moved to to capture the pawn
	use File::*;
	use PieceColor::*;
	EnPassantState::Available {
		position: match (
			latest_move.piece.piece_color(),
			latest_move.final_position.file()
		) {
			(White, _A) => Position::A3,
			(White, _B) => Position::B3,
			(White, _C) => Position::C3,
			(White, _D) => Position::D3,
			(White, _E) => Position::E3,
			(White, _F) => Position::F3,
			(White, _G) => Position::G3,
			(White, _H) => Position::H3,
			(Black, _A) => Position::A6,
			(Black, _B) => Position::B6,
			(Black, _C) => Position::C6,
			(Black, _D) => Position::D6,
			(Black, _E) => Position::E6,
			(Black, _F) => Position::F6,
			(Black, _G) => Position::G6,
			(Black, _H) => Position::H6
		},
		captured: latest_move.entity
	}
}
