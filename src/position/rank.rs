use bevy::prelude::*;

pub const ONE: Rank = Rank::One;
pub const TWO: Rank = Rank::Two;
pub const THREE: Rank = Rank::Three;
pub const FOUR: Rank = Rank::Four;
pub const FIVE: Rank = Rank::Five;
pub const SIX: Rank = Rank::Six;
pub const SEVEN: Rank = Rank::Seven;
pub const EIGHT: Rank = Rank::Eight;
pub const RANKS: [Rank; 8] = [EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO, ONE];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rank {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

impl Rank {
    pub fn char(&self) -> char {
        match self {
            Rank::One => '1',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8'
        }
    }

    pub fn isize(&self) -> isize { self as *const _ as isize }

    pub fn vec3(&self) -> Vec3 {
        let z = self.isize() as f32;
        Vec3::new(0.0, 0.0, -z + 0.5)
    }

    pub fn iter() -> impl Iterator<Item = &'static Rank> { RANKS.iter() }
}
