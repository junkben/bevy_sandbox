use bevy::prelude::*;

mod active_color;
mod castle_availability;
mod en_passant_tracker;
mod game_mode;
mod halfmove_tracker;
mod scoreboard;
mod theme;

pub use active_color::ActiveColor;
pub use castle_availability::CastlingAvailability;
pub use en_passant_tracker::EnPassantTracker;
pub use game_mode::GameMode;
pub use halfmove_tracker::HalfmoveTracker;
pub use scoreboard::Scoreboard;
pub use theme::Theme;

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveColor::default())
            .insert_resource(CastlingAvailability::default())
            .insert_resource(EnPassantTracker::default())
            .insert_resource(GameMode::default())
            .insert_resource(HalfmoveTracker::default())
            .insert_resource(Scoreboard::default())
            .insert_resource(Theme::default());
    }
}
