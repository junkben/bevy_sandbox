use bevy::prelude::*;

use super::*;
use crate::menu::MenuState;

pub struct DisplaySettingsMenuPlugin;

impl Plugin for DisplaySettingsMenuPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(DisplayQuality::default())
			// Systems to handle the display settings screen
			.add_systems(
				OnEnter(MenuState::SettingsDisplay),
				display_settings_menu_setup
			)
			.add_systems(
				OnExit(MenuState::SettingsDisplay),
				despawn_screen::<OnDisplaySettingsMenuScreen>
			)
			.add_systems(
				Update,
				(setting_button::<DisplayQuality>
					.run_if(in_state(MenuState::SettingsDisplay)),)
			)
			.add_systems(
				Update,
				display_menu_action
					.run_if(in_state(MenuState::SettingsDisplay))
			);
	}
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum DisplaySettingsMenuButtonAction {
	Back
}

// One of the two settings that can be set through the menu. It will be a
// resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Default)]
enum DisplayQuality {
	Low,
	#[default]
	Medium,
	High
}

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

fn display_settings_menu_setup(
	mut commands: Commands,
	display_quality: Res<DisplayQuality>
) {
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
			OnDisplaySettingsMenuScreen
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
					// Create a new `NodeBundle`, this time not setting its
					// `flex_direction`. It will use the default value,
					// `FlexDirection::Row`, from left to right.
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
							// Display a label for the current setting
							parent.spawn(TextBundle::from_section(
								"Display Quality",
								button_text_style.clone()
							));
							// Display a button for each possible value
							for quality_setting in [
								DisplayQuality::Low,
								DisplayQuality::Medium,
								DisplayQuality::High
							] {
								let mut entity = parent.spawn((
									ButtonBundle {
										style: Style {
											width: Val::Px(150.0),
											height: Val::Px(65.0),
											..button_style.clone()
										},
										background_color: NORMAL_BUTTON.into(),
										..default()
									},
									quality_setting
								));
								entity.with_children(|parent| {
									parent.spawn(TextBundle::from_section(
										format!("{quality_setting:?}"),
										button_text_style.clone()
									));
								});
								if *display_quality == quality_setting {
									entity.insert(SelectedOption);
								}
							}
						});
					// Display the back button to return to the settings screen
					parent
						.spawn((
							ButtonBundle {
								style: button_style,
								background_color: NORMAL_BUTTON.into(),
								..default()
							},
							DisplaySettingsMenuButtonAction::Back
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

fn display_menu_action(
	query_button: Query<
		(&Interaction, &DisplaySettingsMenuButtonAction),
		(Changed<Interaction>, With<Button>)
	>,
	mut menu_state: ResMut<NextState<MenuState>>
) {
	for (interaction, action) in &query_button {
		if *interaction == Interaction::Pressed {
			use DisplaySettingsMenuButtonAction::*;
			match action {
				Back => {
					menu_state.set(MenuState::Settings);
				}
			}
		}
	}
}
