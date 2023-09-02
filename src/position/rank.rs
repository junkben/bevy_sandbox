use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    _One = 1,
    _Two,
    _Three,
    _Four,
    _Five,
    _Six,
    _Seven,
    _Eight
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

impl Rank {
    pub const EIGHT: Rank = Rank::_Eight;
    pub const FIVE: Rank = Rank::_Five;
    pub const FOUR: Rank = Rank::_Four;
    pub const ONE: Rank = Rank::_One;
    pub const SEVEN: Rank = Rank::_Seven;
    pub const SIX: Rank = Rank::_Six;
    pub const THREE: Rank = Rank::_Three;
    pub const TWO: Rank = Rank::_Two;

    pub fn try_from_isize(i: isize) -> Option<Rank> {
        Rank::iter().filter(|&r| r.isize() == i).next().copied()
    }

    pub fn char(&self) -> char {
        match self {
            Rank::_One => '1',
            Rank::_Two => '2',
            Rank::_Three => '3',
            Rank::_Four => '4',
            Rank::_Five => '5',
            Rank::_Six => '6',
            Rank::_Seven => '7',
            Rank::_Eight => '8'
        }
    }

    pub fn isize(&self) -> isize { self as *const _ as isize }

    pub fn translation(&self) -> Vec3 {
        let z = self.isize() as f32;
        Vec3::new(0.0, 0.0, -z + 0.5)
    }

    pub fn iter() -> impl Iterator<Item = &'static Rank> {
        [
            Rank::EIGHT,
            Rank::SEVEN,
            Rank::SIX,
            Rank::FIVE,
            Rank::FOUR,
            Rank::THREE,
            Rank::TWO,
            Rank::ONE
        ]
        .iter()
    }
}
