use bevy::prelude::*;

#[derive(Default)]
pub enum GameMode {
    #[default]
    Standard,
    Replay
}

#[derive(Default, Debug, PartialEq)]
pub enum GameStage {
    #[default]
    Idle,
    TurnWhite,
    TurnBlack
}

#[derive(Default)]
pub struct Scoreboard {
    pub white_wins: u32,
    pub black_wins: u32
}

#[derive(Resource, Default)]
pub struct GameState {
    pub mode:       GameMode,
    pub scoreboard: Scoreboard,
    pub stage:      GameStage
}

pub fn game_stage_toggle(
    keyboard_inputs: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>
) {
    if !keyboard_inputs.just_pressed(KeyCode::F1) {
        return;
    }

    use GameStage::*;
    game_state.stage = match game_state.stage {
        Idle => TurnWhite,
        TurnWhite => TurnBlack,
        TurnBlack => Idle
    };
    debug!(?game_state.stage);
}
