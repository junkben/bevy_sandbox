mod file;
mod rank;

use bevy::prelude::*;
pub use file::File;
pub use rank::Rank;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    file: &'static File,
    rank: &'static Rank
}

impl From<Position> for (isize, isize) {
    fn from(value: Position) -> Self {
        (value.file.isize(), value.rank.isize())
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

macro_rules! positions {
    ($($name:ident, $file:ident, $rank:ident);*) => {
        $(
            pub const $name: Position = Position {
                file: &File::$file,
                rank: &Rank::$rank
            };
        )*
    }
}

impl Position {
    positions!(
        A8, A, Eight;
        B8, B, Eight;
        C8, C, Eight;
        D8, D, Eight;
        E8, E, Eight;
        F8, F, Eight;
        G8, G, Eight;
        H8, H, Eight;
        A7, A, Seven;
        B7, B, Seven;
        C7, C, Seven;
        D7, D, Seven;
        E7, E, Seven;
        F7, F, Seven;
        G7, G, Seven;
        H7, H, Seven;
        A6, A, Six;
        B6, B, Six;
        C6, C, Six;
        D6, D, Six;
        E6, E, Six;
        F6, F, Six;
        G6, G, Six;
        H6, H, Six;
        A5, A, Five;
        B5, B, Five;
        C5, C, Five;
        D5, D, Five;
        E5, E, Five;
        F5, F, Five;
        G5, G, Five;
        H5, H, Five;
        A4, A, Four;
        B4, B, Four;
        C4, C, Four;
        D4, D, Four;
        E4, E, Four;
        F4, F, Four;
        G4, G, Four;
        H4, H, Four;
        A3, A, Three;
        B3, B, Three;
        C3, C, Three;
        D3, D, Three;
        E3, E, Three;
        F3, F, Three;
        G3, G, Three;
        H3, H, Three;
        A2, A, Two;
        B2, B, Two;
        C2, C, Two;
        D2, D, Two;
        E2, E, Two;
        F2, F, Two;
        G2, G, Two;
        H2, H, Two;
        A1, A, One;
        B1, B, One;
        C1, C, One;
        D1, D, One;
        E1, E, One;
        F1, F, One;
        G1, G, One;
        H1, H, One
    );

    pub fn try_from_xz(x: isize, z: isize) -> Option<Position> {
        let file = File::try_from_isize(x)?;
        let rank = Rank::try_from_isize(z)?;
        Some(Position { file, rank })
    }

    pub fn try_from_vec3(vector: Vec3) -> Option<Position> {
        if vector.y != 0.0 {
            return None;
        }

        Position::try_from_xz(vector.x as isize, vector.z as isize)
    }

    pub fn file(&self) -> &'static File { self.file }

    pub fn rank(&self) -> &'static Rank { self.rank }

    pub fn xz(&self) -> (isize, isize) { <(isize, isize)>::from(*self) }

    pub fn iter() -> impl Iterator<Item = Position> {
        Self::iter_rank_file().map(|(rank, file)| Position { file, rank })
    }

    pub fn iter_rank_file(
    ) -> impl Iterator<Item = (&'static Rank, &'static File)> {
        Rank::iter().zip(File::iter())
    }

    pub fn iter_xz() -> impl Iterator<Item = (isize, isize)> {
        Self::iter_rank_file().map(|(&r, &f)| (r as isize, f as isize))
    }

    pub fn vec3(&self) -> Vec3 { self.file.vec3() + self.rank.vec3() }

    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.vec3())
    }
}
