use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::{backends::raycast::RaycastPickable, prelude::*};

pub struct SquareSelectPlugin;
impl Plugin for SquareSelectPlugin {
	fn build(&self, app: &mut App) { app.add_event::<UserSelectedSquare>(); }
}

#[derive(Bundle)]
pub struct SquareSelectionBundle {
	pickable:           Pickable,
	interaction:        PickingInteraction,
	selection:          PickSelection,
	highlight:          PickHighlight,
	raycast:            RaycastPickable,
	highlight_override: Highlight<StandardMaterial>
}

impl Default for SquareSelectionBundle {
	fn default() -> Self {
		Self {
			pickable:           Pickable::IGNORE,
			interaction:        interaction(),
			selection:          selection(),
			highlight:          highlight(),
			raycast:            raycast(),
			highlight_override: highlight_override()
		}
	}
}

fn interaction() -> PickingInteraction { PickingInteraction::default() }

fn selection() -> PickSelection { PickSelection::default() }

fn highlight() -> PickHighlight { PickHighlight::default() }

fn highlight_override() -> Highlight<StandardMaterial> {
	Highlight {
		hovered:  Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
			base_color: matl.base_color + vec4(0.3, 0.3, 0.3, 0.0),
			..matl.to_owned()
		})),
		pressed:  Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
			base_color: matl.base_color + vec4(0.6, 0.6, 0.6, 0.0),
			..matl.to_owned()
		})),
		selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
			base_color: matl.base_color + vec4(0.3, 0.3, 0.3, 0.0),
			..matl.to_owned()
		}))
	}
}

// fn raycast() -> RaycastPickTarget { RaycastPickTarget::default() }
fn raycast() -> RaycastPickable { RaycastPickable::default() }

#[derive(Event)]
pub struct UserSelectedSquare {
	pub entity: Entity
}

impl From<ListenerInput<Pointer<Click>>> for UserSelectedSquare {
	fn from(event: ListenerInput<Pointer<Click>>) -> Self {
		UserSelectedSquare {
			entity: event.target
		}
	}
}
