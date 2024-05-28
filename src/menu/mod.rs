mod audio_settings_menu;
mod death_menu;
mod display_settings_menu;
mod level_win_menu;
mod main_menu;
mod pause_menu;
mod settings_menu;

use self::{
    audio_settings_menu::SoundSettingsPlugin, display_settings_menu::DisplaySettingsPlugin,
    main_menu::MainMenuState, pause_menu::PauseMenuState, settings_menu::SettingsPlugin,
};
use crate::components::*;
use crate::ui::*;
use bevy::prelude::*;

// All actions that can be triggered from a button click
#[derive(Component, PartialEq)]
enum MenuButtonAction {
    PlayGame,
    Settings,
    SettingsSound,
    SettingsDisplay,
    BackToMainMenu,
    BackToSettings,
    ExitApplication,
    QuitGame,
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        main_menu::plugin,
        pause_menu::plugin,
        DisplaySettingsPlugin(MainMenuState::SettingsDisplay),
        SoundSettingsPlugin(MainMenuState::SettingsSound),
        SettingsPlugin(MainMenuState::Settings),
        DisplaySettingsPlugin(PauseMenuState::SettingsDisplay),
        SoundSettingsPlugin(PauseMenuState::SettingsSound),
        SettingsPlugin(PauseMenuState::Settings),
        death_menu::plugin,
        level_win_menu::plugin,
    ))
    .add_systems(Update, (button_system).run_if(in_menu));
}

/// run condition
fn in_menu(game_state: Res<State<GameState>>, in_game_state: Res<State<InGameState>>) -> bool {
    *game_state == GameState::Menu || *in_game_state == InGameState::Pause
}
