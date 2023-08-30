use bevy::prelude::*;

macro_rules! board_positions {
    ($($name:ident, $file:ident, $rank:ident);*) => {
        $(
            pub const $name: BoardPosition = BoardPosition {
                file: File::$file,
                rank: Rank::$rank
            };
        )*

        pub const BOARD_POSITIONS: [BoardPosition; 64] = [
            $($name),*
        ];
    };
}

board_positions!(
    A1, A, One;
    A2, A, Two;
    A3, A, Three;
    A4, A, Four;
    A5, A, Five;
    A6, A, Six;
    A7, A, Seven;
    A8, A, Eight;
    B1, B, One;
    B2, B, Two;
    B3, B, Three;
    B4, B, Four;
    B5, B, Five;
    B6, B, Six;
    B7, B, Seven;
    B8, B, Eight;
    C1, C, One;
    C2, C, Two;
    C3, C, Three;
    C4, C, Four;
    C5, C, Five;
    C6, C, Six;
    C7, C, Seven;
    C8, C, Eight;
    D1, D, One;
    D2, D, Two;
    D3, D, Three;
    D4, D, Four;
    D5, D, Five;
    D6, D, Six;
    D7, D, Seven;
    D8, D, Eight;
    E1, E, One;
    E2, E, Two;
    E3, E, Three;
    E4, E, Four;
    E5, E, Five;
    E6, E, Six;
    E7, E, Seven;
    E8, E, Eight;
    F1, F, One;
    F2, F, Two;
    F3, F, Three;
    F4, F, Four;
    F5, F, Five;
    F6, F, Six;
    F7, F, Seven;
    F8, F, Eight;
    G1, G, One;
    G2, G, Two;
    G3, G, Three;
    G4, G, Four;
    G5, G, Five;
    G6, G, Six;
    G7, G, Seven;
    G8, G, Eight;
    H1, H, One;
    H2, H, Two;
    H3, H, Three;
    H4, H, Four;
    H5, H, Five;
    H6, H, Six;
    H7, H, Seven;
    H8, H, Eight
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl From<File> for char {
    fn from(value: File) -> Self {
        use File::*;
        match value {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h'
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = char::from(*self);
        write!(f, "{c}")
    }
}

impl From<File> for Vec3 {
    fn from(value: File) -> Vec3 {
        use File::*;
        let x = match value {
            A => 1.0,
            B => 2.0,
            C => 3.0,
            D => 4.0,
            E => 5.0,
            F => 6.0,
            G => 7.0,
            H => 8.0
        };
        Vec3::new(x - 0.5, 0.0, 0.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        use Rank::*;
        match value {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8'
        }
    }
}

impl From<Rank> for Vec3 {
    fn from(value: Rank) -> Self {
        use Rank::*;
        let z = match value {
            One => 1.0,
            Two => 2.0,
            Three => 3.0,
            Four => 4.0,
            Five => 5.0,
            Six => 6.0,
            Seven => 7.0,
            Eight => 8.0
        };
        Vec3::new(0.0, 0.0, -z + 0.5)
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = char::from(*self);
        write!(f, "{c}")
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BoardPosition {
    file: File,
    rank: Rank
}

impl std::fmt::Display for BoardPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl BoardPosition {
    pub fn set(&mut self, rank: Rank, file: File) {
        self.rank = rank;
        self.file = file;
    }

    pub fn rank(&self) -> &Rank { &self.rank }

    pub fn file(&self) -> &File { &self.file }

    pub fn vec3(&self) -> Vec3 { Vec3::from(self.file) + Vec3::from(self.rank) }

    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.vec3())
    }
}
