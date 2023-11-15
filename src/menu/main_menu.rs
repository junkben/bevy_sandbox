use bevy::{app::AppExit, prelude::*};

use super::*;

// This plugin manages the menu, with 4 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app
			// Systems to handle the main menu screen
			.add_systems(OnEnter(MenuState::Main), main_menu_setup)
			.add_systems(
				OnExit(MenuState::Main),
				despawn_screen::<OnMainMenuScreen>
			)
			.add_systems(
				Update,
				main_menu_action.run_if(in_state(MenuState::Main))
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

	parent.spawn((button, action)).with_children(|parent| {
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

fn main_menu_action(
	interaction_query: Query<
		(&Interaction, &MenuButtonAction),
		(Changed<Interaction>, With<Button>)
	>,
	mut app_exit_events: EventWriter<AppExit>,
	mut menu_state: ResMut<NextState<MenuState>>,
	mut game_state: ResMut<NextState<GameState>>
) {
	for (interaction, menu_button_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			use MenuButtonAction::*;
			match menu_button_action {
				Resume => {},
				NewGame => {
					game_state.set(GameState::Game);
					menu_state.set(MenuState::Disabled);
				},
				Settings => {
					menu_state.set(MenuState::Settings);
				},
				Quit => app_exit_events.send(AppExit)
			}
		}
	}
}
