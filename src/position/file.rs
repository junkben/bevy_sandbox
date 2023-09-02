use bevy::prelude::*;

pub const A: File = File::A;
pub const B: File = File::B;
pub const C: File = File::C;
pub const D: File = File::D;
pub const E: File = File::E;
pub const F: File = File::F;
pub const G: File = File::G;
pub const H: File = File::H;
pub const FILES: [File; 8] = [A, B, C, D, E, F, G, H];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum File {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

impl File {
    pub fn try_from_isize(i: isize) -> Option<&'static File> {
        File::iter().filter(|&f| f.isize() == i).next()
    }

    pub fn char(&self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h'
        }
    }

    pub fn isize(&self) -> isize { self as *const _ as isize }

    pub fn vec3(&self) -> Vec3 {
        let x = self.isize() as f32;
        Vec3::new(x - 0.5, 0.0, 0.0)
    }

    pub fn iter() -> impl Iterator<Item = &'static File> { FILES.iter() }
}
