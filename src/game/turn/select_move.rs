use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{move_piece::MoveSelected, TurnState};
use crate::game::{
	board::{Square, UserSelectedSquare},
	piece::{Piece, UserSelectedPiece},
	position::Position,
	resources::{ActiveColor, AvailableMoves},
	MoveInfo
};

pub struct SelectMovePlugin;

#[derive(Event)]
pub struct UpdateSquareSelection {
	pub positions: Vec<Position>
}

#[derive(Resource, Default)]
pub struct SelectedPiece(pub Option<Entity>);

impl Plugin for SelectMovePlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(SelectedPiece::default())
			.add_event::<UpdateSquareSelection>()
			.add_systems(OnEnter(TurnState::SelectMove), enable_piece_selection)
			.add_systems(
				Update,
				(
					handle_event_user_select_piece
						.run_if(on_event::<UserSelectedPiece>()),
					handle_event_user_select_square
						.run_if(on_event::<UserSelectedSquare>()),
					handle_event_update_square_selection
						.run_if(on_event::<UpdateSquareSelection>())
				)
			)
			.add_systems(
				OnExit(TurnState::SelectMove),
				(disable_square_selection, disable_piece_selection)
			);
	}
}

fn handle_event_user_select_piece(
	mut commands: Commands,
	mut er_user_selected: EventReader<UserSelectedPiece>,
	mut ew_selection_update: EventWriter<UpdateSquareSelection>,
	res_available_moves: Res<AvailableMoves>,
	query_piece: Query<Entity, With<Piece>>
) {
	let Some(event) = er_user_selected.into_iter().last() else {
		error!("not exactly one SelectPiece event");
		return;
	};

	// Store selected piece entity
	commands.insert_resource(SelectedPiece(Some(event.entity)));

	// Update square selection based on the final positions of moves for the
	// entity
	let moves = get_moves_for_piece_entity(
		event.entity,
		res_available_moves.as_ref(),
		query_piece
	);

	let positions = moves.iter().map(|m| m.final_position).collect::<Vec<_>>();

	ew_selection_update.send(UpdateSquareSelection { positions })
}

fn handle_event_user_select_square(
	mut commands: Commands,
	mut er_user_selected: EventReader<UserSelectedSquare>,
	mut ew_move_selected: EventWriter<MoveSelected>,
	mut res_turn_state: ResMut<NextState<TurnState>>,
	res_selected_piece: Res<SelectedPiece>,
	res_available_moves: Res<AvailableMoves>,
	query_square: Query<&Position, With<Square>>
) {
	let Some(event) = er_user_selected.into_iter().last() else {
		error!("not exactly one SelectSquare event");
		return;
	};

	let position = get_position_from_square(event.entity, query_square);

	let move_info = select_move(
		&position,
		res_selected_piece.as_ref(),
		res_available_moves.as_ref()
	);

	commands.insert_resource(SelectedPiece(None));
	ew_move_selected.send(MoveSelected { move_info });

	debug!("moving to {:?}", TurnState::MovePiece);
	res_turn_state.set(TurnState::MovePiece);
}

fn handle_event_update_square_selection(
	mut er_update_selection: EventReader<UpdateSquareSelection>,
	query_square: Query<(&Position, &mut Pickable), With<Square>>
) {
	// get event from EventReader
	let Some(event) = er_update_selection.into_iter().last() else {
		error!("not exactly one UpdateSquareSelection event");
		return;
	};

	update_square_selection(&event.positions, query_square)
}

fn update_square_selection(
	positions: &Vec<Position>,
	mut query_square: Query<(&Position, &mut Pickable), With<Square>>
) {
	// Give Selection components to square entities
	for (p, mut pickable) in query_square.iter_mut() {
		// Add selection if the square's position is an available move
		pickable.should_emit_events = positions.contains(p);
	}
}

fn get_position_from_square(
	square_entity: Entity,
	query_square: Query<&Position, With<Square>>
) -> Position {
	let Ok(&position) = query_square.get(square_entity) else {
		panic!("no matching entity in square query");
	};

	position
}

fn get_moves_for_piece_entity(
	entity: Entity,
	available_moves: &AvailableMoves,
	query_piece: Query<Entity, With<Piece>>
) -> Vec<MoveInfo> {
	let Ok(entity) = query_piece.get(entity) else {
		panic!("no matching entity in piece query");
	};

	// if AvailableMoves is empty, then panic (shouldn't have been selectable)
	let Some(moves) = available_moves.get(&entity) else {
		panic!("no moves found for piece entity")
	};

	moves.clone()
}

fn select_move(
	position: &Position,
	selected_piece: &SelectedPiece,
	available_moves: &AvailableMoves
) -> MoveInfo {
	let Some(piece_entity) = selected_piece.0 else {
		panic!("cannot select move, no piece entity")
	};

	let Some(&move_info) = available_moves.get_move_to(&piece_entity, position)
	else {
		panic!("no matching MoveInfo in AvailableMoves");
	};

	move_info
}

fn disable_square_selection(
	mut commands: Commands,
	mut query_pickable: Query<(Entity, &mut PickSelection), With<Square>>
) {
	// Remove Selection components from square entities
	for (entity, mut selection) in query_pickable.iter_mut() {
		selection.is_selected = false;

		trace!("disabling pickable for square entity {:?}", entity);
		commands.entity(entity).insert(Pickable::IGNORE);
	}
}

fn disable_piece_selection(
	mut commands: Commands,
	query_pickable: Query<Entity, With<Piece>>
) {
	// Remove Selection components from piece entities
	for entity in query_pickable.iter() {
		trace!("disabling pickable for piece entity {:?}", entity);
		commands.entity(entity).insert(Pickable::IGNORE);
	}
}

fn enable_piece_selection(
	mut commands: Commands,
	active_color: Res<ActiveColor>,
	available_moves: Res<AvailableMoves>,
	query_nonpickable: Query<(Entity, &Piece)>
) {
	// Give Selection components to pieces whose color matches the active one
	for (entity, piece) in query_nonpickable.iter() {
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
