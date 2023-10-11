use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{move_piece::MoveSelected, TurnState};
use crate::{
	board::{Square, UserSelectedSquare},
	piece::{Piece, UserSelectedPiece},
	position::Position,
	resources::{ActiveColor, AvailableMoves}
};

pub struct SelectMovePlugin;

#[derive(Event)]
pub struct PieceSelected {
	entity: Entity
}

#[derive(Resource, Default)]
pub struct SelectedPieceEntity(pub Option<Entity>);

impl Plugin for SelectMovePlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(SelectedPieceEntity::default())
			.add_event::<PieceSelected>()
			.add_systems(OnEnter(TurnState::SelectMove), enable_piece_selection)
			.add_systems(
				Update,
				(
					select_piece.run_if(on_event::<UserSelectedPiece>()),
					select_square.run_if(on_event::<UserSelectedSquare>()),
					update_square_selection.run_if(on_event::<PieceSelected>())
				)
			)
			.add_systems(
				OnExit(TurnState::SelectMove),
				(disable_square_selection, disable_piece_selection)
			);
	}
}

fn select_piece(
	mut event_reader: EventReader<UserSelectedPiece>,
	mut event_writer: EventWriter<PieceSelected>,
	available_moves: Res<AvailableMoves>,
	mut selected_piece: ResMut<SelectedPieceEntity>,
	piece_query: Query<Entity, With<Piece>>
) {
	let Some(event) = event_reader.into_iter().last() else {
		error!("not exactly one SelectPiece event");
		return;
	};

	let Ok(entity) = piece_query.get(event.entity) else {
		error!("no matching entity in piece query");
		return;
	};

	// if AvailableMoves is empty, then panic (shouldn't have been selectable)
	if available_moves.0.get(&entity).unwrap().is_empty() {
		panic!("available move vector empty")
	}

	selected_piece.0 = Some(entity);
	event_writer.send(PieceSelected { entity })
}

fn select_square(
	mut event_reader: EventReader<UserSelectedSquare>,
	mut event_writer: EventWriter<MoveSelected>,
	mut turn_state: ResMut<NextState<TurnState>>,
	mut selected_piece: ResMut<SelectedPieceEntity>,
	available_moves: Res<AvailableMoves>,
	square_query: Query<&Position, With<Square>>
) {
	let Some(event) = event_reader.into_iter().last() else {
		error!("not exactly one SelectSquare event");
		return;
	};

	let Ok(position) = square_query.get(event.entity) else {
		error!("no matching entity in square query");
		return;
	};
	let piece_entity = selected_piece.0.unwrap();

	let Some(move_info) = available_moves.get_move_to(&piece_entity, position)
	else {
		panic!("no matching MoveInfo in AvailableMoves");
	};

	event_writer.send(MoveSelected {
		entity:    piece_entity,
		move_info: *move_info
	});
	selected_piece.0 = None;

	debug!("moving to {:?}", TurnState::MovePiece);
	turn_state.set(TurnState::MovePiece);
}

fn update_square_selection(
	mut event_reader: EventReader<PieceSelected>,
	available_moves: Res<AvailableMoves>,
	mut square_query: Query<(&Position, &mut Pickable), With<Square>>
) {
	// get event from EventReader
	let Some(event) = event_reader.into_iter().last() else {
		error!("not exactly one PieceSelected event");
		return;
	};

	// Give Selection components to square entities
	for (position, mut pickable) in square_query.iter_mut() {
		// Add selection if the square's position is an available move
		pickable.should_emit_events =
			available_moves.contains_move_to(&event.entity, position);
	}
}

fn disable_square_selection(
	mut commands: Commands,
	mut pickable_query: Query<(Entity, &mut PickSelection), With<Square>>
) {
	// Remove Selection components from square entities
	for (entity, mut selection) in pickable_query.iter_mut() {
		selection.is_selected = false;

		trace!("disabling pickable for square entity {:?}", entity);
		commands.entity(entity).insert(Pickable::IGNORE);
	}
}

fn disable_piece_selection(
	mut commands: Commands,
	pickable_query: Query<Entity, With<Piece>>
) {
	// Remove Selection components from piece entities
	for entity in pickable_query.iter() {
		trace!("disabling pickable for piece entity {:?}", entity);
		commands.entity(entity).insert(Pickable::IGNORE);
	}
}

fn enable_piece_selection(
	mut commands: Commands,
	active_color: Res<ActiveColor>,
	available_moves: Res<AvailableMoves>,
	nonpickable_query: Query<(Entity, &Piece)>
) {
	// Give Selection components to pieces whose color matches the active one
	for (entity, piece) in nonpickable_query.iter() {
		// Continue if the entity has no available moves
		let Some(moves) = available_moves.0.get(&entity) else {
			error!("piece entity not found in AvailableMoves, skipping...");
			continue;
		};

		// Add selection if the piece has available moves and its color matches
		// the active one
		if !moves.is_empty() && piece.piece_color() == &active_color.0 {
			trace!("enabling pickable for piece entity {:?}", entity);
			commands.entity(entity).insert(Pickable::default());
		}
	}
}
