use bevy::prelude::*;
use bevy_mod_picking::{backends::raycast::RaycastPickable, prelude::*};

pub struct PieceSelectPlugin;
impl Plugin for PieceSelectPlugin {
	fn build(&self, app: &mut App) { app.add_event::<UserSelectedPiece>(); }
}

#[derive(Bundle)]
pub struct PieceSelectionBundle {
	pickable:           Pickable,
	interaction:        PickingInteraction,
	selection:          PickSelection,
	highlight:          PickHighlight,
	raycast:            RaycastPickable,
	highlight_override: Highlight<StandardMaterial>
}

impl Default for PieceSelectionBundle {
	fn default() -> Self {
		Self {
			pickable:           Pickable::IGNORE,
			interaction:        Self::interaction(),
			selection:          Self::selection(),
			highlight:          Self::highlight(),
			raycast:            Self::raycast(),
			highlight_override: Self::highlight_override()
		}
	}
}

impl PieceSelectionBundle {
	const HIGHLIGHT_HOVERED_OFFSET: Vec4 = Vec4::new(0.3, 0.3, 0.3, 0.0);
	const HIGHLIGHT_PRESSED_OFFSET: Vec4 = Vec4::new(0.3, 0.3, 0.3, 0.0);
	const HIGHLIGHT_SELECTED_OFFSET: Vec4 = Vec4::new(0.3, 0.3, 0.3, 0.0);

	fn interaction() -> PickingInteraction { PickingInteraction::default() }

	fn selection() -> PickSelection { PickSelection::default() }

	fn highlight() -> PickHighlight { PickHighlight::default() }

	fn highlight_override() -> Highlight<StandardMaterial> {
		Highlight {
			hovered:  Some(HighlightKind::new_dynamic(|matl| {
				StandardMaterial {
					base_color: matl.base_color
						+ Self::HIGHLIGHT_HOVERED_OFFSET,
					..matl.to_owned()
				}
			})),
			pressed:  Some(HighlightKind::new_dynamic(|matl| {
				StandardMaterial {
					base_color: matl.base_color
						+ Self::HIGHLIGHT_PRESSED_OFFSET,
					..matl.to_owned()
				}
			})),
			selected: Some(HighlightKind::new_dynamic(|matl| {
				StandardMaterial {
					base_color: matl.base_color
						+ Self::HIGHLIGHT_SELECTED_OFFSET,
					..matl.to_owned()
				}
			}))
		}
	}

	fn raycast() -> RaycastPickable { RaycastPickable::default() }
}

#[derive(Event)]
pub struct UserSelectedPiece {
	pub entity: Entity
}

impl From<ListenerInput<Pointer<Click>>> for UserSelectedPiece {
	fn from(event: ListenerInput<Pointer<Click>>) -> Self {
		UserSelectedPiece {
			entity: event.target
		}
	}
}
