use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::prelude::*;

#[derive(Bundle)]
pub struct PieceSelectionBundle {
    pickable:    Pickable,
    interaction: PickingInteraction,
    selection:   PickSelection,
    highlight:   PickHighlight,
    raycast:     RaycastPickTarget
}

impl Default for PieceSelectionBundle {
    fn default() -> Self {
        Self {
            pickable:    pickable(),
            interaction: interaction(),
            selection:   selection(),
            highlight:   highlight(),
            raycast:     raycast()
        }
    }
}

impl PieceSelectionBundle {
    pub fn add_selection(commands: &mut Commands, entity: Entity) {
        commands
            .entity(entity)
            .insert((PieceSelectionBundle::default(), highlight_override()));
    }

    pub fn remove_selection(commands: &mut Commands, entity: Entity) {
        commands
            .entity(entity)
            .remove::<Pickable>()
            .remove::<PickingInteraction>()
            .remove::<PickSelection>()
            .remove::<PickHighlight>()
            .remove::<RaycastPickTarget>();
    }
}

fn pickable() -> Pickable { Pickable::default() }

fn interaction() -> PickingInteraction { PickingInteraction::default() }

fn selection() -> PickSelection { PickSelection::default() }

fn highlight() -> PickHighlight { PickHighlight::default() }

fn highlight_override() -> Highlight<StandardMaterial> {
    Highlight {
        hovered:  Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.2, -0.2, 0.4, 0.0),
            ..matl.to_owned()
        })),
        pressed:  Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.3, -0.3, 0.5, 0.0),
            ..matl.to_owned()
        })),
        selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
            base_color: matl.base_color + vec4(-0.3, 0.2, -0.3, 0.0),
            ..matl.to_owned()
        }))
    }
}

fn raycast() -> RaycastPickTarget { RaycastPickTarget::default() }
