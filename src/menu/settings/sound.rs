use bevy::prelude::*;

use super::*;
use crate::menu::MenuState;

pub struct SoundSettingsMenuPlugin;

impl Plugin for SoundSettingsMenuPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Volume::default())
			// Systems to handle the sound settings screen
			.add_systems(
				OnEnter(MenuState::SettingsSound),
				sound_settings_menu_setup
			)
			.add_systems(
				OnExit(MenuState::SettingsSound),
				despawn_screen::<OnSoundSettingsMenuScreen>
			)
			.add_systems(
				Update,
				setting_button::<Volume>
					.run_if(in_state(MenuState::SettingsSound))
			)
			.add_systems(
				Update,
				sound_menu_action.run_if(in_state(MenuState::SettingsSound))
			);
	}
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum SoundSettingsMenuButtonAction {
	Back
}

// One of the two settings that can be set through the menu. It will be a
// resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

impl Default for Volume {
	fn default() -> Self { Self(7) }
}

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;

fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
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
			OnSoundSettingsMenuScreen
		))
		.with_children(|parent| {
			parent
				.spawn(NodeBundle {
					style: Style {
						flex_direction: FlexDirection::Column,
						align_items: AlignItems::Center,
						..default()
					},
					background_color: MENU_PANEL.into(),
					..default()
				})
				.with_children(|parent| {
					parent
						.spawn(NodeBundle {
							style: Style {
								align_items: AlignItems::Center,
								..default()
							},
							background_color: MENU_PANEL.into(),
							..default()
						})
						.with_children(|parent| {
							parent.spawn(TextBundle::from_section(
								"Volume",
								button_text_style.clone()
							));
							for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
							{
								let mut entity = parent.spawn((
									ButtonBundle {
										style: Style {
											width: Val::Px(30.0),
											height: Val::Px(65.0),
											..button_style.clone()
										},
										background_color: NORMAL_BUTTON.into(),
										..default()
									},
									Volume(volume_setting)
								));
								if *volume == Volume(volume_setting) {
									entity.insert(SelectedOption);
								}
							}
						});
					parent
						.spawn((
							ButtonBundle {
								style: button_style,
								background_color: NORMAL_BUTTON.into(),
								..default()
							},
							SoundSettingsMenuButtonAction::Back
						))
						.with_children(|parent| {
							parent.spawn(TextBundle::from_section(
								"Back",
								button_text_style
							));
						});
				});
		});
}

fn sound_menu_action(
	interaction_query: Query<
		(&Interaction, &SoundSettingsMenuButtonAction),
		(Changed<Interaction>, With<Button>)
	>,
	mut menu_state: ResMut<NextState<MenuState>>
) {
	for (interaction, action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			use SoundSettingsMenuButtonAction::*;
			match action {
				Back => {
					menu_state.set(MenuState::Settings);
				}
			}
		}
	}
}
