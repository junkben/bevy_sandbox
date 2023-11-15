use bevy::prelude::*;
use menu::MenuState;

mod game;
mod log;
mod menu;
mod splash;
mod window;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
	App::new()
		// Declare the game state, whose starting value is determined by the
		// `Default` trait
		.add_state::<GameState>()
		.add_plugins(
			DefaultPlugins
				.set(window::window_plugin())
				.set(log::log_plugin())
		)
		// Adds the plugins for each state
		.add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
		.add_systems(
			Update,
			(
				handle_user_open_menu.run_if(in_state(GameState::Game)),
				handle_user_close_menu.run_if(in_state(GameState::Menu))
			)
		)
		.run();
}

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
	#[default]
	Splash,
	Menu,
	Game
}

// Generic system that takes a component as a parameter, and will despawn all
// entities with that component
fn despawn_screen<T: Component>(
	query_entities_to_despawn: Query<Entity, With<T>>,
	mut commands: Commands
) {
	for entity in &query_entities_to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}

fn handle_user_open_menu(
	mut game_state: ResMut<NextState<GameState>>,
	mut menu_state: ResMut<NextState<MenuState>>,
	keys: Res<Input<KeyCode>>
) {
	if keys.just_pressed(KeyCode::Escape) {
		debug!("moving to menu from game");
		game_state.set(GameState::Menu);
		menu_state.set(MenuState::Main);
	}
}

fn handle_user_close_menu(
	mut game_state: ResMut<NextState<GameState>>,
	mut menu_state: ResMut<NextState<MenuState>>,
	keys: Res<Input<KeyCode>>
) {
	if keys.just_pressed(KeyCode::Escape) {
		debug!("moving to game from menu");
		game_state.set(GameState::Game);
		menu_state.set(MenuState::Disabled);
	}
}
