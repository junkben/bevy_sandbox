use bevy::{app::AppExit, prelude::*};
use bevy_eventlistener::{callbacks::ListenerInput, event_listener::On};
use bevy_mod_picking::events::{Click, Over, Pointer};

use super::*;
use crate::audio::{PlaySoundButtonHovered, PlaySoundButtonPressed};

// This plugin manages the menu, with 4 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<ButtonPress>()
			.add_event::<ButtonHover>()
			// Systems to handle the main menu screen
			.add_systems(OnEnter(MenuState::Main), main_menu_setup)
			.add_systems(
				OnExit(MenuState::Main),
				despawn_screen::<OnMainMenuScreen>
			)
			.add_systems(
				Update,
				(
					handle_event_button_pressed
						.run_if(on_event::<ButtonPress>()),
					handle_event_button_hovered
						.run_if(on_event::<ButtonHover>())
				)
			);
	}
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
	Resume,
	NewGame,
	Settings,
	Quit
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

/// Common style for all buttons on the main menu
fn main_menu_button_style() -> Style {
	Style {
		width: Val::Px(250.0),
		height: Val::Px(65.0),
		margin: UiRect::all(Val::Px(20.0)),
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		..default()
	}
}

fn main_menu_button_icon_style() -> Style {
	Style {
		width: Val::Px(30.0),
		// This takes the icons out of the flexbox flow, to be positioned
		// exactly
		position_type: PositionType::Absolute,
		// The icon will be close to the left border of the button
		left: Val::Px(10.0),
		..default()
	}
}

fn main_menu_button_text_style() -> TextStyle {
	TextStyle {
		font_size: 40.0,
		color: TEXT_COLOR,
		..default()
	}
}

fn spawn_main_menu_button(
	parent: &mut ChildBuilder,
	action: MenuButtonAction,
	texture: Handle<Image>,
	text: impl Into<String>
) {
	let icon = ImageBundle {
		style: main_menu_button_icon_style(),
		image: UiImage::new(texture),
		..default()
	};
	let text = TextBundle::from_section(text, main_menu_button_text_style());
	let button = ButtonBundle {
		style: main_menu_button_style(),
		background_color: NORMAL_BUTTON.into(),
		..default()
	};

	parent
		.spawn((
			button,
			action,
			On::<Pointer<Over>>::send_event::<ButtonHover>(),
			On::<Pointer<Click>>::send_event::<ButtonPress>()
		))
		.with_children(|parent| {
			parent.spawn(icon);
			parent.spawn(text);
		});
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
			OnMainMenuScreen
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
					// Display the game name
					parent.spawn(
						TextBundle::from_section(
							"Bevy Game Menu UI",
							TextStyle {
								font_size: 80.0,
								color: TEXT_COLOR,
								..default()
							}
						)
						.with_style(Style {
							margin: UiRect::all(Val::Px(50.0)),
							..default()
						})
					);

					spawn_main_menu_button(
						parent,
						MenuButtonAction::Resume,
						asset_server.load("textures/menu_icons/right.png"),
						"Resume"
					);

					spawn_main_menu_button(
						parent,
						MenuButtonAction::NewGame,
						asset_server.load("textures/menu_icons/right.png"),
						"New Game"
					);

					spawn_main_menu_button(
						parent,
						MenuButtonAction::Settings,
						asset_server.load("textures/menu_icons/wrench.png"),
						"Settings"
					);

					spawn_main_menu_button(
						parent,
						MenuButtonAction::Quit,
						asset_server.load("textures/menu_icons/exit.png"),
						"Quit"
					);
				});
		});
}

#[derive(Event)]
pub struct ButtonPress {
	entity: Entity
}

impl From<ListenerInput<Pointer<Click>>> for ButtonPress {
	fn from(event: ListenerInput<Pointer<Click>>) -> Self {
		ButtonPress {
			entity: event.target
		}
	}
}

fn handle_event_button_pressed(
	mut er: EventReader<ButtonPress>,
	query_button: Query<(Entity, &MenuButtonAction)>,
	mut ew_app_exit: EventWriter<AppExit>,
	mut ew_play_sound_pressed: EventWriter<PlaySoundButtonPressed>,
	mut menu_state: ResMut<NextState<MenuState>>,
	mut game_state: ResMut<NextState<GameState>>
) {
	let Some(event) = er.read().last() else {
		return;
	};

	ew_play_sound_pressed.send(PlaySoundButtonPressed);

	let Some((_, action)) =
		query_button.iter().find(|&(e, _)| e == event.entity)
	else {
		return;
	};

	use MenuButtonAction::*;
	match action {
		Resume => {},
		NewGame => {
			game_state.set(GameState::Game);
			menu_state.set(MenuState::Disabled);
		},
		Settings => {
			menu_state.set(MenuState::Settings);
		},
		Quit => ew_app_exit.send(AppExit)
	}
}

#[derive(Event)]
pub struct ButtonHover {
	entity: Entity
}

impl From<ListenerInput<Pointer<Over>>> for ButtonHover {
	fn from(event: ListenerInput<Pointer<Over>>) -> Self {
		ButtonHover {
			entity: event.target
		}
	}
}

fn handle_event_button_hovered(
	mut er: EventReader<ButtonHover>,
	// query_button: Query<Entity>,
	mut ew_play_sound: EventWriter<PlaySoundButtonHovered>
) {
	let Some(_event) = er.read().last() else {
		return;
	};

	ew_play_sound.send(PlaySoundButtonHovered);
}
