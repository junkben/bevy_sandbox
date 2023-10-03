use std::collections::HashMap;

use bevy::prelude::*;

use super::{Piece, PieceColor};
use crate::{
    move_info::MoveInfo,
    move_tracker::MoveTracker,
    piece::{PieceMovementBehavior, PieceType},
    position::*,
    resources::ActiveColor,
    MoveType
};

pub struct AvailableMovesPlugin;
impl Plugin for AvailableMovesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CalculateAvailableMoves>()
            .add_event::<CalculateAvailableMovesDone>()
            .add_systems(
                Update,
                calculate_available_moves
                    .run_if(on_event::<CalculateAvailableMoves>())
            );
    }
}

/// A component that tracks the available positions an entity can move to
#[derive(Component, Default, Debug, Clone)]
pub struct AvailableMoves(pub Vec<MoveInfo>);

impl std::fmt::Display for AvailableMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl AvailableMoves {
    pub fn contains_move_to(&self, position: &Position) -> bool {
        for m in &self.0 {
            if &m.final_position == position {
                return true;
            }
        }

        return false;
    }

    pub fn get_move_to(&self, position: &Position) -> Option<&MoveInfo> {
        for m in &self.0 {
            if &m.final_position == position {
                return Some(m);
            }
        }

        return None;
    }
}

#[derive(Event)]
pub struct CalculateAvailableMoves;

#[derive(Event)]
pub struct CalculateAvailableMovesDone;

fn calculate_available_moves(
    mut event_reader: EventReader<CalculateAvailableMoves>,
    mut event_writer: EventWriter<CalculateAvailableMovesDone>,
    active_color: Res<ActiveColor>,
    mut piece_query: Query<(
        Entity,
        &Piece,
        &Position,
        &MoveTracker,
        &mut AvailableMoves
    )>
) {
    // Consume CalculateAvailableMoves
    let Some(_) = event_reader.into_iter().last() else {
        error!("not exactly one CalculateAvailableMoves event");
        return;
    };

    let occupied_positions = piece_query
        .iter()
        .map(|(entity, other_piece, other_position, ..)| {
            (
                other_position.clone(),
                (entity, other_piece.piece_color().clone())
            )
        })
        .collect::<HashMap<Position, (Entity, PieceColor)>>();

    for (
        entity,
        &piece,
        &initial_position,
        &move_tracker,
        mut available_moves
    ) in piece_query.iter_mut()
    {
        if piece.piece_color() != &active_color.0 {
            continue;
        }

        // Gather piece default movement patterns
        use PieceColor::*;
        use PieceType::*;
        let mut movement_patterns = vec![match piece.piece_type() {
            King => PieceMovementBehavior::KING,
            Queen => PieceMovementBehavior::QUEEN,
            Rook => PieceMovementBehavior::ROOK,
            Bishop => PieceMovementBehavior::BISHOP,
            Knight => PieceMovementBehavior::KNIGHT,
            Pawn => match piece.piece_color() {
                White => PieceMovementBehavior::PAWN_WHITE,
                Black => PieceMovementBehavior::PAWN_BLACK
            }
        }];

        // If it's a pawn that hasn't moved, it can move another space forward
        if piece.piece_type() == &Pawn && !move_tracker.has_moved() {
            movement_patterns.push(match piece.piece_color() {
                White => PieceMovementBehavior::PAWN_FIRSTMOVE_WHTIE,
                Black => PieceMovementBehavior::PAWN_FIRSTMOVE_BLACK
            });
        }

        let (start_x, start_z) = initial_position.xz();
        let start_vec = Vec3::new(start_x as f32, 0.0, start_z as f32);

        let mut moves: Vec<MoveInfo> = Vec::new();

        for mp in movement_patterns {
            for direction in mp.directions() {
                let mut l: u8 = 1u8;

                while l <= mp.length() {
                    let vector = start_vec + (direction.clone() * l as f32);

                    // If the proposed Position can't exist, break
                    let final_position = match Position::try_from_vec3(vector) {
                        Some(bp) => bp,
                        None => break
                    };

                    // Check if there is a piece at the end position. If there
                    // is, we'll record it's color
                    if let Some((other_entity, other_piece_color)) =
                        occupied_positions.get(&final_position)
                    {
                        // We're breaking as we can't possibly go past this
                        // piece, but if the piece is of
                        // the opposite color, then we can still
                        // move there to capture it. If it's our piece, then we
                        // can't move there or past it.
                        if other_piece_color != piece.piece_color() {
                            moves.push(MoveInfo {
                                entity,
                                piece,
                                initial_position,
                                final_position,
                                move_type: MoveType::Capture {
                                    is_en_passant: false,
                                    captured:      *other_entity
                                }
                            });
                        }

                        // break while loop
                        break;
                    }

                    // Move is to a legitimate position and there's no piece
                    // in the way
                    moves.push(MoveInfo {
                        entity,
                        piece,
                        initial_position,
                        final_position,
                        move_type: MoveType::Move
                    });
                    l += 1;
                }
            }
        }
        available_moves.0 = moves;
        trace!(?piece, ?initial_position, ?available_moves);
    }

    event_writer.send(CalculateAvailableMovesDone)
}
