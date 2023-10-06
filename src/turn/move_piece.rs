use bevy::prelude::*;

use super::TurnState;
use crate::{
    move_tracker::MoveTracker,
    physics::TranslationalMotionDone,
    piece::{MovePieceToBoardPosition, Piece},
    position::Position,
    resources::MoveHistory,
    MoveInfo, MoveType
};

pub struct PieceMovementPlugin;

impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveSelected>().add_systems(
            Update,
            (
                wait_for_piece_motion_to_complete
                    .run_if(in_state(TurnState::MovePiece)),
                confirm_move.run_if(on_event::<MoveSelected>())
            )
        );
    }
}

#[derive(Event)]
pub struct MoveSelected {
    pub entity:    Entity,
    pub move_info: MoveInfo
}

fn confirm_move(
    mut commands: Commands,
    mut event_reader: EventReader<MoveSelected>,
    mut event_writer: EventWriter<MovePieceToBoardPosition>,
    mut move_history: ResMut<MoveHistory>,
    mut piece_query: Query<(&mut Position, &mut MoveTracker), With<Piece>>
) {
    let Some(event) = event_reader.into_iter().last() else {
        error!("not exactly one MoveSelected event");
        return;
    };

    let Ok((mut position, mut move_tracker)) =
        piece_query.get_mut(event.entity)
    else {
        error!("no entity matches piece query");
        return;
    };

    // TODO: Remove captured piece in more elegant way via animation?
    // If it's a capture, remove the captured entity
    if let MoveType::Capture { captured, .. } = event.move_info.move_type {
        commands.entity(captured).despawn();
    }

    // Update position of piece to new position
    position.set_rank(*event.move_info.final_position.rank());
    position.set_file(*event.move_info.final_position.file());

    // Increment the move tracker
    move_tracker.inc();

    // Send the event to physically move the piece to the new board position
    event_writer.send(MovePieceToBoardPosition {
        entity:      event.entity,
        destination: event.move_info.final_position
    });

    // Add the move to the MoveHistory resource
    move_history.append_move(event.move_info);
}

fn wait_for_piece_motion_to_complete(
    mut event_reader: EventReader<TranslationalMotionDone>,
    mut turn_state: ResMut<NextState<TurnState>>,
    piece_query: Query<Entity, With<Piece>>
) {
    for event in event_reader.into_iter() {
        let Ok(_entity) = piece_query.get(event.entity) else {
            return;
        };

        // Event entity handshake succeeded, move to next state
        debug!("moving to {:?}", TurnState::UpdateBoardState);
        turn_state.set(TurnState::UpdateBoardState);
    }
}
