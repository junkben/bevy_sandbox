use bevy::prelude::*;

use crate::{physics::TranslationalMotionStart, position::Position};

pub struct PieceMovementPlugin;
impl Plugin for PieceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovePieceToBoardPosition>().add_systems(
            Update,
            move_piece_to_board_position
                .run_if(on_event::<MovePieceToBoardPosition>())
        );
    }
}

#[derive(Event)]
pub struct MovePieceToBoardPosition {
    pub entity:      Entity,
    pub destination: Position
}

/// Turn "move entity to board position" event into a "translation motion start"
/// event
fn move_piece_to_board_position(
    mut events: EventReader<MovePieceToBoardPosition>,
    mut event_writer: EventWriter<TranslationalMotionStart>,
    entities: Query<Entity>
) {
    for event in events.into_iter() {
        let Ok(entity) = entities.get(event.entity) else {
            error!("no matching entity");
            return;
        };

        let destination = event.destination.translation();
        event_writer.send(TranslationalMotionStart {
            entity,
            destination
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum SpecialMovement {
    PawnCapture
}

#[derive(Debug)]
pub struct PieceMovementBehavior(Vec<(Vec3, u8, Option<SpecialMovement>)>);

impl PieceMovementBehavior {
    /// Kings can move 1 square vertically, horizontally, or diagonally
    pub fn king() -> PieceMovementBehavior {
        PieceMovementBehavior(vec![
            (N.vec3(), 1, None),
            (NE.vec3(), 1, None),
            (E.vec3(), 1, None),
            (SE.vec3(), 1, None),
            (S.vec3(), 1, None),
            (SW.vec3(), 1, None),
            (W.vec3(), 1, None),
            (NW.vec3(), 1, None),
        ])
    }

    /// Knights can either move 1 square horizontally and 2 squares vertically
    /// or move 2 squares horizontally and 1 square vertically
    pub fn knight() -> PieceMovementBehavior {
        PieceMovementBehavior(vec![
            (NNE.vec3(), 1, None),
            (ENE.vec3(), 1, None),
            (ESE.vec3(), 1, None),
            (SSE.vec3(), 1, None),
            (SSW.vec3(), 1, None),
            (WSW.vec3(), 1, None),
            (WNW.vec3(), 1, None),
            (NNW.vec3(), 1, None),
        ])
    }

    /// Pawn movement is complicated
    pub fn pawn(color: PieceColor, first_move: bool) -> PieceMovementBehavior {
        let mut move_vec: Vec<(Vec3, u8, Option<SpecialMovement>)> = vec![];

        let max_magnitude: u8 = if first_move { 2 } else { 1 };
        match color {
            PieceColor::White => {
                move_vec.push((N.vec3(), max_magnitude, None));
                move_vec.push((
                    NE.vec3(),
                    1,
                    Some(SpecialMovement::PawnCapture)
                ));
                move_vec.push((
                    NW.vec3(),
                    1,
                    Some(SpecialMovement::PawnCapture)
                ));
            },
            PieceColor::Black => {
                move_vec.push((S.vec3(), max_magnitude, None));
                move_vec.push((
                    SE.vec3(),
                    1,
                    Some(SpecialMovement::PawnCapture)
                ));
                move_vec.push((
                    SW.vec3(),
                    1,
                    Some(SpecialMovement::PawnCapture)
                ));
            }
        };

        PieceMovementBehavior(move_vec)
    }

    /// Queens can move any number of squares vertically, horizontally, or
    /// diagonally
    pub fn queen() -> PieceMovementBehavior {
        PieceMovementBehavior(vec![
            (N.vec3(), u8::MAX, None),
            (NE.vec3(), u8::MAX, None),
            (E.vec3(), u8::MAX, None),
            (SE.vec3(), u8::MAX, None),
            (S.vec3(), u8::MAX, None),
            (SW.vec3(), u8::MAX, None),
            (W.vec3(), u8::MAX, None),
            (NW.vec3(), u8::MAX, None),
        ])
    }

    /// Rooks can move any number of squares vertically or horizontally
    pub fn rook() -> PieceMovementBehavior {
        PieceMovementBehavior(vec![
            (N.vec3(), u8::MAX, None),
            (E.vec3(), u8::MAX, None),
            (S.vec3(), u8::MAX, None),
            (W.vec3(), u8::MAX, None),
        ])
    }

    /// Bishops can move any number of squares diagonally
    pub fn bishop() -> PieceMovementBehavior {
        PieceMovementBehavior(vec![
            (NE.vec3(), u8::MAX, None),
            (SE.vec3(), u8::MAX, None),
            (SW.vec3(), u8::MAX, None),
            (NW.vec3(), u8::MAX, None),
        ])
    }

    pub fn iter<'a>(
        &'a self
    ) -> impl Iterator<Item = &'a (Vec3, u8, Option<SpecialMovement>)> + 'a
    {
        self.0.iter()
    }
}

// TODO: Castling behavior for king and rook
enum Direction {
    N,
    NNE,
    NE,
    ENE,
    E,
    ESE,
    SE,
    SSE,
    S,
    SSW,
    SW,
    WSW,
    W,
    WNW,
    NW,
    NNW
}

use Direction::*;

use super::PieceColor;

impl Direction {
    pub const fn vec3(&self) -> Vec3 {
        let (x, z) = match self {
            N => (0.0, 1.0),
            NNE => (1.0, 2.0),
            NE => (1.0, 1.0),
            ENE => (2.0, 1.0),
            E => (1.0, 0.0),
            ESE => (2.0, -1.0),
            SE => (1.0, -1.0),
            SSE => (1.0, -2.0),
            S => (0.0, -1.0),
            SSW => (-1.0, -2.0),
            SW => (-1.0, -1.0),
            WSW => (-2.0, -1.0),
            W => (0.0, -1.0),
            WNW => (-2.0, 1.0),
            NW => (-1.0, 1.0),
            NNW => (-1.0, 2.0)
        };

        Vec3::new(x, 0.0, z)
    }
}
