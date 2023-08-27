mod color;
pub mod position;
pub mod select;
mod spawn;
mod square;

use bevy::prelude::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(select::SelectedSquare::default())
            .add_systems(Startup, spawn::spawn_board)
            .add_systems(Update, select::select_square);
    }
}
