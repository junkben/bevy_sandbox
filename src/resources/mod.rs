use bevy::prelude::*;

mod board_state;
pub use board_state::BoardState;

mod game_state;
pub use game_state::{GameStage, GameState};

mod theme;
pub use theme::Theme;

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardState::default())
            .insert_resource(GameState::default())
            .insert_resource(Theme::default())
            .add_systems(Update, game_state::game_stage_toggle);
    }
}
