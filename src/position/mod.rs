mod file;
mod rank;

use bevy::prelude::*;
pub use file::File;
pub use rank::Rank;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BoardPosition {
    pub file: &'static File,
    pub rank: &'static Rank
}

impl From<BoardPosition> for (isize, isize) {
    fn from(value: BoardPosition) -> Self {
        (value.file.isize(), value.rank.isize())
    }
}

impl std::fmt::Display for BoardPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl BoardPosition {
    pub fn rank(&self) -> &Rank { &self.rank }

    pub fn file(&self) -> &File { &self.file }

    pub fn xz(&self) -> (isize, isize) { <(isize, isize)>::from(*self) }

    pub fn iter() -> impl Iterator<Item = BoardPosition> {
        Self::iter_rank_file().map(|(rank, file)| BoardPosition { file, rank })
    }

    pub fn iter_rank_file(
    ) -> impl Iterator<Item = (&'static Rank, &'static File)> {
        Rank::iter().zip(File::iter())
    }

    pub fn iter_xz() -> impl Iterator<Item = (isize, isize)> {
        Self::iter_rank_file().map(|(&r, &f)| (r as isize, f as isize))
    }

    // pub fn from_xz(x: isize, z: isize) -> BoardPosition {
    //    BoardPosition {
    //        file: &x.into(),
    //        rank: &z.into()
    //    }
    //}

    pub fn vec3(&self) -> Vec3 { self.file.vec3() + self.rank.vec3() }

    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.vec3())
    }

    pub fn dist_max_x(&self) -> isize { 8 - self.file.isize() }

    pub fn dist_min_x(&self) -> isize { self.file.isize() - 1 }

    pub fn dist_max_z(&self) -> isize { 8 - self.rank.isize() }

    pub fn dist_min_z(&self) -> isize { self.rank.isize() - 1 }
}

// pub fn diagonal_positions(start: BoardPosition) -> Vec<BoardPosition> {
//    let (start_x, start_z) = start.xz();
//    let mut positions = Vec::new();
//    for (x, z) in BoardPosition::iter_xz() {
//        let x_diff = (start_x - x).abs();
//        let z_diff = (start_z - z).abs();
//
//        // Add if the absolute differences are equivalent and neither are zero
//        if x_diff == z_diff && x_diff != 0 {
//            positions.push(BoardPosition::from_xz(x, z))
//        }
//    }
//    positions
//}
