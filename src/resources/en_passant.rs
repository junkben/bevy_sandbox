use bevy::prelude::*;

use super::MoveHistory;
use crate::{
	piece::PieceColor,
	position::{File, Position},
	MoveType
};

pub struct EnPassantPlugin;
impl Plugin for EnPassantPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(EnPassantTracker::default())
			.add_event::<CheckEnPassant>()
			.add_event::<CheckEnPassantDone>()
			.add_systems(
				Update,
				check_en_passant.run_if(on_event::<CheckEnPassant>())
			);
	}
}

#[derive(Event)]
pub struct CheckEnPassant;

#[derive(Event)]
pub struct CheckEnPassantDone;

/// Tracks whether or not there's a target for an en passant capture
#[derive(Resource, Default, Debug)]
pub enum EnPassantTracker {
	#[default]
	Unavailable,
	Available {
		position: Position,
		captured: Entity
	}
}

fn check_en_passant(
	mut commands: Commands,
	mut event_reader: EventReader<CheckEnPassant>,
	mut event_writer: EventWriter<CheckEnPassantDone>,
	move_history: Res<MoveHistory>
) {
	// Consume CheckEnPassant
	let Some(_) = event_reader.into_iter().last() else {
		error!("not exactly one CheckEnPassant event");
		return;
	};

	let ept = determine_en_passant(move_history);

	debug!(?ept);
	commands.insert_resource(ept);
	event_writer.send(CheckEnPassantDone);
}

fn determine_en_passant(move_history: Res<MoveHistory>) -> EnPassantTracker {
	let Some(latest_move) = move_history.latest_move() else {
		return EnPassantTracker::Unavailable;
	};

	if latest_move.move_type != MoveType::FirstMove {
		return EnPassantTracker::Unavailable;
	}

	// Determine the position that needs to be moved to to capture the pawn
	use File::*;
	use PieceColor::*;
	EnPassantTracker::Available {
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
