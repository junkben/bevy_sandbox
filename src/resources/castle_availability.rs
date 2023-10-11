use bevy::prelude::*;

/// Tracks what players can castle and to what side
#[derive(Resource, Debug)]
pub struct CastlingAvailability {
	white_kingside:  bool,
	white_queenside: bool,
	black_kingside:  bool,
	black_queenside: bool
}

impl CastlingAvailability {
	pub fn white_castled_kingside(&mut self) { self.white_kingside = false }

	pub fn white_castled_queenside(&mut self) { self.white_queenside = false }

	pub fn black_castled_kingside(&mut self) { self.black_kingside = false }

	pub fn black_castled_queenside(&mut self) { self.black_queenside = false }

	pub fn white_can_castle(&self) -> bool {
		self.white_kingside && self.white_queenside
	}

	pub fn black_can_castle(&self) -> bool {
		self.black_kingside && self.black_queenside
	}
}

impl std::fmt::Display for CastlingAvailability {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let wk = if self.white_kingside { "K" } else { "" };
		let wq = if self.white_queenside { "Q" } else { "" };
		let bk = if self.black_kingside { "k" } else { "" };
		let bq = if self.black_queenside { "q" } else { "" };
		write!(f, "{wk}{wq}{bk}{bq}")
	}
}

impl Default for CastlingAvailability {
	fn default() -> Self {
		Self {
			white_kingside:  true,
			white_queenside: true,
			black_kingside:  true,
			black_queenside: true
		}
	}
}
