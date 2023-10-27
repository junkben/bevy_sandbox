mod color;
mod selection;
mod spawn;
mod square;

use bevy::prelude::*;
pub use color::SquareColor;
pub use selection::{SquareSelectionBundle, UserSelectedSquare};
pub use square::Square;

use self::selection::SquareSelectPlugin;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(SquareSelectPlugin)
			.add_systems(Startup, spawn::spawn_board);
	}
}
