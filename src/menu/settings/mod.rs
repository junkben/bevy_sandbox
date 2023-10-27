mod display;
mod sound;

use bevy::prelude::*;

use self::{
	display::{DisplaySettingsMenuPlugin, DisplaySettingsMenuState},
	sound::{SoundSettingsMenuPlugin, SoundSettingsMenuState}
};
use super::{
	despawn_screen, MenuState, SelectedOption, NORMAL_BUTTON, TEXT_COLOR
};

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<SettingsMenuState>()
			.add_plugins((DisplaySettingsMenuPlugin, SoundSettingsMenuPlugin))
			// Systems to handle the settings menu screen
			.add_systems(OnEnter(SettingsMenuState::Main), settings_menu_setup)
			.add_systems(
				OnExit(SettingsMenuState::Main),
				despawn_screen::<OnSettingsMenuScreen>
			)
			.add_systems(
				Update,
				settings_menu_action.run_if(in_state(SettingsMenuState::Main))
			);
	}
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SettingsMenuState {
	Main,
	Display,
	Sound,
	#[default]
	Disabled
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum SettingsMenuButtonAction {
	Display,
	Sound,
	Back
}

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

fn settings_menu_setup(mut commands: Commands) {
	let button_style = Style {
		width: Val::Px(200.0),
		height: Val::Px(65.0),
		margin: UiRect::all(Val::Px(20.0)),
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		..default()
	};

	let button_text_style = TextStyle {
		font_size: 40.0,
		color: TEXT_COLOR,
		..default()
	};

	commands
		.spawn((
			NodeBundle {
				style: Style {
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				..default()
			},
			OnSettingsMenuScreen
		))
		.with_children(|parent| {
			parent
				.spawn(NodeBundle {
					style: Style {
						flex_direction: FlexDirection::Column,
						align_items: AlignItems::Center,
						..default()
					},
					background_color: Color::CRIMSON.into(),
					..default()
				})
				.with_children(|parent| {
					use SettingsMenuButtonAction::*;
					for (action, text) in
						[(Display, "Display"), (Sound, "Sound"), (Back, "Back")]
					{
						parent
							.spawn((
								ButtonBundle {
									style: button_style.clone(),
									background_color: NORMAL_BUTTON.into(),
									..default()
								},
								action
							))
							.with_children(|parent| {
								parent.spawn(TextBundle::from_section(
									text,
									button_text_style.clone()
								));
							});
					}
				});
		});
}

// This system updates the settings when a new value for a setting is selected,
// and marks the button as the one currently selected
pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
	interaction_query: Query<
		(&Interaction, &T, Entity),
		(Changed<Interaction>, With<Button>)
	>,
	mut selected_query: Query<
		(Entity, &mut BackgroundColor),
		With<SelectedOption>
	>,
	mut commands: Commands,
	mut setting: ResMut<T>
) {
	for (interaction, button_setting, entity) in &interaction_query {
		if *interaction == Interaction::Pressed && *setting != *button_setting {
			let (previous_button, mut previous_color) =
				selected_query.single_mut();
			*previous_color = NORMAL_BUTTON.into();
			commands.entity(previous_button).remove::<SelectedOption>();
			commands.entity(entity).insert(SelectedOption);
			*setting = *button_setting;
		}
	}
}

fn settings_menu_action(
	interaction_query: Query<
		(&Interaction, &SettingsMenuButtonAction),
		(Changed<Interaction>, With<Button>)
	>,
	mut main_menu_state: ResMut<NextState<MenuState>>,
	mut settings_menu_state: ResMut<NextState<SettingsMenuState>>,
	mut display_menu_state: ResMut<NextState<DisplaySettingsMenuState>>,
	mut sound_menu_state: ResMut<NextState<SoundSettingsMenuState>>
) {
	for (interaction, action) in &interaction_query {
		if *interaction != Interaction::Pressed {
			continue;
		}

		use SettingsMenuButtonAction::*;
		match action {
			Display => {
				settings_menu_state.set(SettingsMenuState::Display);
				display_menu_state.set(DisplaySettingsMenuState::Main);
			},
			Sound => {
				settings_menu_state.set(SettingsMenuState::Sound);
				sound_menu_state.set(SoundSettingsMenuState::Main);
			},
			Back => {
				settings_menu_state.set(SettingsMenuState::Disabled);
				main_menu_state.set(MenuState::Main);
			}
		}
	}
}
