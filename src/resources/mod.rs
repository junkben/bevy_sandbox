use bevy::prelude::*;

macro_rules! expose_mod_resource {
	($($m:ident::$r:ident),*) => {
		$(
			mod $m;
			pub use $m::$r;
		)*

		pub struct ResourcesPlugin;
		impl Plugin for ResourcesPlugin {
			fn build(&self, app: &mut App) {
				app.add_plugins(available_moves::AvailableMovesPlugin);
				app.add_plugins(castle_availability::CastleAvailabilityPlugin);
				$(
					app.insert_resource($r::default());
				)*
			}
		}
	};
}

expose_mod_resource!(
	active_color::ActiveColor,
	available_moves::AvailableMoves,
	castle_availability::CastleAvailability,
	en_passant_tracker::EnPassantTracker,
	game_mode::GameMode,
	halfmove_tracker::HalfmoveTracker,
	move_history::MoveHistory,
	scoreboard::Scoreboard,
	theme::Theme
);

pub use available_moves::{
	CalculateAvailableMoves, CalculateAvailableMovesDone
};
pub use castle_availability::{
	CastleType, CheckCastleAvailability, CheckCastleAvailabilityDone
};
