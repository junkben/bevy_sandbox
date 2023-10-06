use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    move_info::MoveInfo,
    move_tracker::MoveTracker,
    piece::{
        Piece, PieceColor, PieceMovementBehavior, PieceType, SpecialMovement
    },
    position::*,
    MoveType
};

pub struct AvailableMovesPlugin;
impl Plugin for AvailableMovesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AvailableMoves::default())
            .add_event::<CalculateAvailableMoves>()
            .add_event::<CalculateAvailableMovesDone>()
            .add_systems(
                Update,
                calculate_available_moves
                    .run_if(on_event::<CalculateAvailableMoves>())
            );
    }
}

/// A component that tracks the available positions an entity can move to
#[derive(Resource, Default, Debug, Clone)]
pub struct AvailableMoves(pub HashMap<Entity, Vec<MoveInfo>>);

impl std::fmt::Display for AvailableMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl AvailableMoves {
    pub fn contains_move_to(
        &self,
        entity: &Entity,
        position: &Position
    ) -> bool {
        if let Some(moves) = self.0.get(entity) {
            for m in moves {
                if &m.final_position == position {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn get_move_to(
        &self,
        entity: &Entity,
        position: &Position
    ) -> Option<&MoveInfo> {
        if let Some(moves) = self.0.get(entity) {
            for m in moves {
                if &m.final_position == position {
                    return Some(m);
                }
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
    mut available_moves: ResMut<AvailableMoves>,
    mut piece_query: Query<(Entity, &Piece, &Position, &MoveTracker)>
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

    for (entity, &piece, &initial_position, &move_tracker) in
        piece_query.iter_mut()
    {
        let is_pawn = piece.piece_type() == &Pawn;

        // Gather piece default movement patterns
        use PieceType::*;
        let movement_patterns = match piece.piece_type() {
            King => PieceMovementBehavior::king(),
            Queen => PieceMovementBehavior::queen(),
            Rook => PieceMovementBehavior::rook(),
            Bishop => PieceMovementBehavior::bishop(),
            Knight => PieceMovementBehavior::knight(),
            Pawn => {
                // Grab piece color
                let piece_color = *piece.piece_color();

                // It is the pawn's first move if it hasn't moved yet
                let first_move = !move_tracker.has_moved();

                PieceMovementBehavior::pawn(piece_color, first_move)
            }
        };

        let (start_x, start_z) = initial_position.xz();
        let start_vec = Vec3::new(start_x as f32, 0.0, start_z as f32);

        let mut moves: Vec<MoveInfo> = Vec::new();

        for (direction, max_magnitude, special_opt) in movement_patterns.iter()
        {
            let mut l: u8 = 1u8;
            let pawn_capture =
                is_pawn && special_opt == &Some(SpecialMovement::PawnCapture);

            while l <= *max_magnitude {
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
                    let same_color = other_piece_color == piece.piece_color();

                    // We're breaking as we can't possibly go past this
                    // piece, but if the piece is of
                    // the opposite color, then we can still
                    // move there to capture it. If it's our piece, then we
                    // can't move there or past it.
                    // ALSO if we're a pawn
                    if !same_color || (!same_color && pawn_capture) {
                        moves.push(MoveInfo {
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

                l += 1;
                // If we're a pawn and we're checking capture condition, don't
                // include this as move
                if pawn_capture {
                    continue;
                }

                // Move is to a legitimate position and there's no piece
                // in the way
                moves.push(MoveInfo {
                    piece,
                    initial_position,
                    final_position,
                    move_type: MoveType::Move
                });
            } // end while
        } // end for
        available_moves.0.insert(entity, moves);
        trace!(?piece, ?initial_position, ?available_moves);
    }

    event_writer.send(CalculateAvailableMovesDone)
}
