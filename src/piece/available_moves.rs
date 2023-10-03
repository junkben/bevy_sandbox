use std::collections::HashMap;

use bevy::prelude::*;

use super::{Piece, PieceColor};
use crate::{
    move_info::MoveInfo,
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
#[derive(Component, Default, Debug)]
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
}

#[derive(Event)]
pub struct CalculateAvailableMoves;

#[derive(Event)]
pub struct CalculateAvailableMovesDone;

fn calculate_available_moves(
    mut event_reader: EventReader<CalculateAvailableMoves>,
    mut event_writer: EventWriter<CalculateAvailableMovesDone>,
    active_color: Res<ActiveColor>,
    mut piece_query: Query<(Entity, &Piece, &Position, &mut AvailableMoves)>
) {
    // Consume CalculateAvailableMoves
    let Some(_) = event_reader.into_iter().last() else {
        error!("not exactly one CalculateAvailableMoves event");
        return;
    };

    let occupied_positions = piece_query
        .iter()
        .map(|(_entity, other_piece, other_position, ..)| {
            (other_position.clone(), other_piece.piece_color().clone())
        })
        .collect::<HashMap<Position, PieceColor>>();

    for (entity, &piece, &initial_position, mut available_moves) in
        piece_query.iter_mut()
    {
        if piece.piece_color() != &active_color.0 {
            continue;
        }

        use PieceColor::*;
        use PieceType::*;
        let movement_pattern = match (piece.piece_color(), piece.piece_type()) {
            (_, King) => PieceMovementBehavior::KING,
            (_, Queen) => PieceMovementBehavior::QUEEN,
            (_, Rook) => PieceMovementBehavior::ROOK,
            (_, Bishop) => PieceMovementBehavior::BISHOP,
            (_, Knight) => PieceMovementBehavior::KNIGHT,
            (White, Pawn) => PieceMovementBehavior::PAWN_WHITE,
            (Black, Pawn) => PieceMovementBehavior::PAWN_BLACK
        };
        let (start_x, start_z) = initial_position.xz();
        let start_vec = Vec3::new(start_x as f32, 0.0, start_z as f32);

        let mut moves: Vec<MoveInfo> = Vec::new();

        for direction in movement_pattern.directions() {
            let mut l: u8 = 1u8;

            while l <= movement_pattern.length() {
                let vector = start_vec + (direction.clone() * l as f32);

                // If the proposed Position can't exist, break
                let final_position = match Position::try_from_vec3(vector) {
                    Some(bp) => bp,
                    None => break
                };

                // Check if there is a piece at the end position. If there is,
                // we'll record it's color
                if let Some(other_piece_color) =
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
                                is_en_passant: false
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
        available_moves.0 = moves;
        debug!(?piece, ?initial_position, ?available_moves);
    }

    event_writer.send(CalculateAvailableMovesDone)
}
