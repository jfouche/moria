use bevy::prelude::*;

/// Represent the Game state
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    InGame,
}

// Represent the state while in game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    #[default]
    Disabled,
    Running,
    Pause,
    PlayerDied,
    PlayerFinished,
}

pub fn game_is_running(in_game_state: Res<State<InGameState>>) -> bool {
    *in_game_state == InGameState::Running
}
