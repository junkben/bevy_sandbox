use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectedPiece {
    pub entity: Option<Entity>
}
