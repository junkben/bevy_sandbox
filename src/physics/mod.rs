use bevy::prelude::*;

mod motion;

pub use motion::{
    TranslationalMotion, TranslationalMotionDone, TranslationalMotionStart
};

use self::motion::{read_translational_motion_start_events, update_motion};

pub struct MotionPlugin;
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TranslationalMotionStart>()
            .add_event::<TranslationalMotionDone>()
            .add_systems(
                Update,
                (update_motion, read_translational_motion_start_events)
            );
    }
}
