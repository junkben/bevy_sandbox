use bevy::prelude::*;

mod board_state;
pub use board_state::BoardState;

mod game_mode;
pub use game_mode::GameMode;

mod scoreboard;
pub use scoreboard::Scoreboard;

mod theme;
pub use theme::Theme;

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardState::default())
            .insert_resource(GameMode::default())
            .insert_resource(Scoreboard::default())
            .insert_resource(Theme::default());
    }
}
