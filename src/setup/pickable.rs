use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub fn spawn_pickable(mut commands: Commands) {
    commands.spawn((
        // The `bevy_picking_raycast` backend works with meshes
        PbrBundle::default(),
        // Makes the entity pickable
        PickableBundle::default(),
        // Marker for the `bevy_picking_raycast` backend
        RaycastPickTarget::default()
    ));

    commands.spawn((
        Camera3dBundle::default(),
        RaycastPickCamera::default() // Enable picking with this camera
    ));
}
