mod file;
mod rank;

use bevy::prelude::*;
pub use file::File;
pub use rank::Rank;

/// A component that represents a position on a chess board
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
	file: File,
	rank: Rank
}

impl From<Position> for (i8, i8) {
	fn from(value: Position) -> Self { (value.file.i8(), value.rank.i8()) }
}

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.file, self.rank)
	}
}

macro_rules! positions {
	($($str:expr => $name:ident, $file:ident, $rank:ident);*) => {
		$(
			pub const $name: Position = Position {
				file: File::$file,
				rank: Rank::$rank
			};
		)*

		pub const ALL: phf::Map<&'static str, Position> = phf::phf_map!(
			$($str => Position::$name),*
		);
	}
}

impl Position {
	positions!(
		"A8" => A8, A, EIGHT;
		"B8" => B8, B, EIGHT;
		"C8" => C8, C, EIGHT;
		"D8" => D8, D, EIGHT;
		"E8" => E8, E, EIGHT;
		"F8" => F8, F, EIGHT;
		"G8" => G8, G, EIGHT;
		"H8" => H8, H, EIGHT;
		"A7" => A7, A, SEVEN;
		"B7" => B7, B, SEVEN;
		"C7" => C7, C, SEVEN;
		"D7" => D7, D, SEVEN;
		"E7" => E7, E, SEVEN;
		"F7" => F7, F, SEVEN;
		"G7" => G7, G, SEVEN;
		"H7" => H7, H, SEVEN;
		"A6" => A6, A, SIX;
		"B6" => B6, B, SIX;
		"C6" => C6, C, SIX;
		"D6" => D6, D, SIX;
		"E6" => E6, E, SIX;
		"F6" => F6, F, SIX;
		"G6" => G6, G, SIX;
		"H6" => H6, H, SIX;
		"A5" => A5, A, FIVE;
		"B5" => B5, B, FIVE;
		"C5" => C5, C, FIVE;
		"D5" => D5, D, FIVE;
		"E5" => E5, E, FIVE;
		"F5" => F5, F, FIVE;
		"G5" => G5, G, FIVE;
		"H5" => H5, H, FIVE;
		"A4" => A4, A, FOUR;
		"B4" => B4, B, FOUR;
		"C4" => C4, C, FOUR;
		"D4" => D4, D, FOUR;
		"E4" => E4, E, FOUR;
		"F4" => F4, F, FOUR;
		"G4" => G4, G, FOUR;
		"H4" => H4, H, FOUR;
		"A3" => A3, A, THREE;
		"B3" => B3, B, THREE;
		"C3" => C3, C, THREE;
		"D3" => D3, D, THREE;
		"E3" => E3, E, THREE;
		"F3" => F3, F, THREE;
		"G3" => G3, G, THREE;
		"H3" => H3, H, THREE;
		"A2" => A2, A, TWO;
		"B2" => B2, B, TWO;
		"C2" => C2, C, TWO;
		"D2" => D2, D, TWO;
		"E2" => E2, E, TWO;
		"F2" => F2, F, TWO;
		"G2" => G2, G, TWO;
		"H2" => H2, H, TWO;
		"A1" => A1, A, ONE;
		"B1" => B1, B, ONE;
		"C1" => C1, C, ONE;
		"D1" => D1, D, ONE;
		"E1" => E1, E, ONE;
		"F1" => F1, F, ONE;
		"G1" => G1, G, ONE;
		"H1" => H1, H, ONE
	);

	pub fn set_rank(&mut self, rank: Rank) { self.rank = rank; }

	pub fn set_file(&mut self, file: File) { self.file = file }

	pub fn try_from_xz(x: i8, z: i8) -> Option<Position> {
		let file = File::try_from_isize(x)?;
		let rank = Rank::try_from_isize(z)?;
		Some(Position { file, rank })
	}

	pub fn try_from_vec3(vector: Vec3) -> Option<Position> {
		if vector.y != 0.0 {
			return None;
		}

		Position::try_from_xz(vector.x as i8, vector.z as i8)
	}

	pub fn file(&self) -> &File { &self.file }

	pub fn rank(&self) -> &Rank { &self.rank }

	pub fn xz(&self) -> (i8, i8) { <(i8, i8)>::from(*self) }

	pub fn iter() -> impl Iterator<Item = &'static Position> {
		Self::ALL.values()
	}

	pub fn iter_rank_file(
	) -> impl Iterator<Item = (&'static Rank, &'static File)> {
		Rank::iter().zip(File::iter())
	}

	pub fn iter_xz() -> impl Iterator<Item = (i8, i8)> {
		Self::iter_rank_file().map(|(&r, &f)| (r as i8, f as i8))
	}

	pub fn translation(&self) -> Vec3 {
		self.file.translation() + self.rank.translation()
	}

	pub fn transform(&self) -> Transform {
		Transform::from_translation(self.translation())
	}
}
