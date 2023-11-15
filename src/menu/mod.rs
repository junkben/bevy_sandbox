mod main_menu;
mod settings;

use bevy::prelude::*;

use self::{main_menu::MainMenuPlugin, settings::SettingsMenuPlugin};
use super::{despawn_screen, GameState, TEXT_COLOR};

// This plugin manages the menu, with 4 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			// At start, the menu is not enabled. This will be changed in
			// `menu_setup` when entering the `GameState::Menu` state.
			// Current screen in the menu is handled by an independent state
			// from `GameState`
			.add_state::<MenuState>()
			.add_systems(OnEnter(GameState::Menu), menu_setup)
			// Common systems to all screens that handles buttons behavior
			.add_systems(
				Update,
				button_system.run_if(in_state(GameState::Menu))
			)
			.add_plugins((MainMenuPlugin, SettingsMenuPlugin));
	}
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
	menu_state.set(MenuState::Main);
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
	Main,
	Replay,
	Settings,
	SettingsSound,
	SettingsDisplay,
	#[default]
	Disabled
}

const NORMAL_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.50);
const HOVERED_BUTTON: Color = Color::rgba(0.25, 0.25, 0.25, 0.50);
const HOVERED_PRESSED_BUTTON: Color = Color::rgba(0.25, 0.65, 0.25, 0.50);
const PRESSED_BUTTON: Color = Color::rgba(0.35, 0.75, 0.35, 0.50);
const MENU_PANEL: Color = Color::rgba(0.15, 0.15, 0.15, 0.50);

// Handle which colors are mapped to which states of button interaction
pub fn button_color(
	interaction: &Interaction,
	selected: Option<&SelectedOption>
) -> Color {
	use Interaction::*;
	match (*interaction, selected) {
		(Pressed, _) | (None, Some(_)) => PRESSED_BUTTON,
		(Hovered, Some(_)) => HOVERED_PRESSED_BUTTON,
		(Hovered, Option::None) => HOVERED_BUTTON,
		(None, Option::None) => NORMAL_BUTTON
	}
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
fn button_system(
	mut query_button: Query<
		(&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
		(Changed<Interaction>, With<Button>)
	>
) {
	for (interaction, mut color, selected) in &mut query_button {
		*color = button_color(interaction, selected).into();
	}
}
