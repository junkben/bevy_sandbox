use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum File {
    _A = 1,
    _B,
    _C,
    _D,
    _E,
    _F,
    _G,
    _H
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

impl File {
    pub const A: File = File::_A;
    pub const B: File = File::_B;
    pub const C: File = File::_C;
    pub const D: File = File::_D;
    pub const E: File = File::_E;
    pub const F: File = File::_F;
    pub const G: File = File::_G;
    pub const H: File = File::_H;

    pub fn try_from_isize(i: isize) -> Option<File> {
        File::iter().filter(|&f| f.isize() == i).next().copied()
    }

    pub fn char(&self) -> char {
        match self {
            File::_A => 'a',
            File::_B => 'b',
            File::_C => 'c',
            File::_D => 'd',
            File::_E => 'e',
            File::_F => 'f',
            File::_G => 'g',
            File::_H => 'h'
        }
    }

    pub fn isize(&self) -> isize { self as *const _ as isize }

    pub fn translation(&self) -> Vec3 {
        let x = self.isize() as f32;
        Vec3::new(x - 0.5, 0.0, 0.0)
    }

    pub fn iter() -> impl Iterator<Item = &'static File> {
        [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H
        ]
        .iter()
    }
}
