use bevy::prelude::*;

#[derive(Debug)]
pub struct PieceMovementBehavior {
    directions: &'static [Vec3],
    length:     u8
}

impl PieceMovementBehavior {
    /// Bishops can move any number of squares diagonally
    pub const BISHOP: PieceMovementBehavior = PieceMovementBehavior {
        directions: &DIRECTIONS_DIAG,
        length:     u8::MAX
    };
    /// Kings can move 1 square vertically, horizontally, or diagonally
    pub const KING: PieceMovementBehavior = PieceMovementBehavior {
        directions: &DIRECTIONS_DIAG_ORTHOG,
        length:     1
    };
    /// Knights can either move 1 square horizontally and 2 squares vertically
    /// or move 2 squares horizontally and 1 square vertically
    pub const KNIGHT: PieceMovementBehavior = PieceMovementBehavior {
        directions: &DIRECTIONS_LSHAPE,
        length:     1
    };
    pub const PAWN_BLACK: PieceMovementBehavior = PieceMovementBehavior {
        directions: &[Vec3::NEG_Z],
        length:     1
    };
    pub const PAWN_CAPTURE_BLACK: PieceMovementBehavior =
        PieceMovementBehavior {
            directions: &[PX_NZ, NX_NZ],
            length:     1
        };
    // PAWN CAPTURE
    pub const PAWN_CAPTURE_WHITE: PieceMovementBehavior =
        PieceMovementBehavior {
            directions: &[PX_PZ, NX_PZ],
            length:     1
        };
    pub const PAWN_FIRSTMOVE_BLACK: PieceMovementBehavior =
        PieceMovementBehavior {
            directions: &[Vec3::NEG_Z],
            length:     2
        };
    // PAWN FIRST MOVE
    pub const PAWN_FIRSTMOVE_WHTIE: PieceMovementBehavior =
        PieceMovementBehavior {
            directions: &[Vec3::Z],
            length:     2
        };
    // PAWNS
    pub const PAWN_WHITE: PieceMovementBehavior = PieceMovementBehavior {
        directions: &[Vec3::Z],
        length:     1
    };
    /// Queens can move any number of squares vertically, horizontally, or
    /// diagonally
    pub const QUEEN: PieceMovementBehavior = PieceMovementBehavior {
        directions: &DIRECTIONS_DIAG_ORTHOG,
        length:     u8::MAX
    };
    /// Rooks can move any number of squares vertically or horizontally
    pub const ROOK: PieceMovementBehavior = PieceMovementBehavior {
        directions: &DIRECTIONS_ORTHOG,
        length:     u8::MAX
    };

    pub fn directions(&self) -> &'static [Vec3] { self.directions }

    pub fn length(&self) -> u8 { self.length }
}

// TODO: Castling behavior for king and rook

const PX_PZ: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 1.0
};
const PX_NZ: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: -1.0
};
const NX_PZ: Vec3 = Vec3 {
    x: -1.0,
    y: 0.0,
    z: 1.0
};
const NX_NZ: Vec3 = Vec3 {
    x: -1.0,
    y: 0.0,
    z: -1.0
};

const DIRECTIONS_LSHAPE: [Vec3; 8] = [
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 2.0
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: -2.0
    },
    Vec3 {
        x: -1.0,
        y: 0.0,
        z: 2.0
    },
    Vec3 {
        x: -1.0,
        y: 0.0,
        z: -2.0
    },
    Vec3 {
        x: 2.0,
        y: 0.0,
        z: 1.0
    },
    Vec3 {
        x: 2.0,
        y: 0.0,
        z: -1.0
    },
    Vec3 {
        x: -2.0,
        y: 0.0,
        z: 1.0
    },
    Vec3 {
        x: -2.0,
        y: 0.0,
        z: -1.0
    }
];
const DIRECTIONS_ORTHOG: [Vec3; 4] =
    [Vec3::X, Vec3::Z, Vec3::NEG_X, Vec3::NEG_Z];
const DIRECTIONS_DIAG: [Vec3; 4] = [
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 1.0
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: -1.0
    },
    Vec3 {
        x: -1.0,
        y: 0.0,
        z: 1.0
    },
    Vec3 {
        x: -1.0,
        y: 0.0,
        z: -1.0
    }
];
const DIRECTIONS_DIAG_ORTHOG: [Vec3; 8] = [
    Vec3::X,
    Vec3::Z,
    Vec3::NEG_X,
    Vec3::NEG_Z,
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 1.0
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: -1.0
    },
    Vec3 {
        x: -1.0,
        y: 0.0,
        z: 1.0
    },
    Vec3 {
        x: -1.0,
        y: 0.0,
        z: -1.0
    }
];
