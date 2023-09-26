use std::{collections::HashMap, f32::consts::TAU};

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use super::TurnState;
use crate::{
    camera::SetCameraTargetAlpha,
    piece::{
        AvailableMoves, Piece, PieceColor, PieceMovementBehavior, PieceType
    },
    position::Position,
    resources::ActiveColor
};

pub struct TurnStartPlugin;

impl Plugin for TurnStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(TurnState::Start), calculate_available_moves)
            .add_systems(
                Update,
                move_camera.run_if(in_state(TurnState::Start))
            );
    }
}

const WHITE_ALPHA: f32 = 0.0;
const BLACK_ALPHA: f32 = TAU / 2.0;

fn move_camera(
    mut turn_state: ResMut<NextState<TurnState>>,
    active_color: Res<ActiveColor>,
    camera_query: Query<Entity, With<PanOrbitCamera>>,
    mut event_writer: EventWriter<SetCameraTargetAlpha>
) {
    let Some(entity) = camera_query.iter().last() else {
        error!("No camera found in query, cannot move camera");
        return;
    };

    use PieceColor::*;
    let target_alpha = match active_color.0 {
        White => WHITE_ALPHA,
        Black => BLACK_ALPHA
    };

    event_writer.send(SetCameraTargetAlpha {
        entity,
        target_alpha
    });

    debug!("moving to {:?}", TurnState::SelectPiece);
    turn_state.set(TurnState::SelectPiece);
}

fn calculate_available_moves(
    active_color: Res<ActiveColor>,
    mut piece_query: Query<(&Piece, &Position, &mut AvailableMoves)>
) {
    let occupied_positions = piece_query
        .iter()
        .map(|(other_piece, other_position, ..)| {
            (other_position.clone(), other_piece.piece_color().clone())
        })
        .collect::<HashMap<Position, PieceColor>>();

    for (piece, position, mut available_moves) in piece_query.iter_mut() {
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
        let (start_x, start_z) = position.xz();
        let start_vec = Vec3::new(start_x as f32, 0.0, start_z as f32);

        let mut moves: Vec<Position> = Vec::new();

        for direction in movement_pattern.directions() {
            let mut l: u8 = 1u8;

            while l <= movement_pattern.length() {
                let vector = start_vec + (direction.clone() * l as f32);

                // If the proposed Position can't exist, break
                let new_move = match Position::try_from_vec3(vector) {
                    Some(bp) => bp,
                    None => break
                };

                // Check if there is a piece at the end position. If there is,
                // we'll record it's color
                if let Some(other_piece_color) =
                    occupied_positions.get(&new_move)
                {
                    // We're breaking as we can't possibly go past this
                    // piece, but if the piece is of
                    // the opposite color, then we can still
                    // move there to capture it. If it's our piece, then we
                    // can't move there or past it.
                    if other_piece_color != piece.piece_color() {
                        moves.push(new_move);
                    }

                    // break while loop
                    break;
                }

                // Move is to a legitimate position and there's no piece
                // in the way
                moves.push(new_move);
                l += 1;
            }
        }
        available_moves.0 = moves;
        debug!(?piece, ?position, ?available_moves);
    }
}
