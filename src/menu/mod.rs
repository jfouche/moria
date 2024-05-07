mod death_menu;
mod display_settings_menu;
mod main_menu;
mod pause_menu;
mod settings_menu;
mod volume_settings_menu;

use self::{
    display_settings_menu::DisplaySettingsPlugin, main_menu::MainMenuState,
    pause_menu::PauseMenuState, settings_menu::SettingsPlugin,
    volume_settings_menu::SoundSettingsPlugin,
};
use crate::ecs::*;
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
    ))
    .add_systems(Update, (button_system).run_if(in_menu));
}

// run condition
fn in_menu(game_state: Res<State<GameState>>, in_game_state: Res<State<InGameState>>) -> bool {
    *game_state == GameState::Menu || *in_game_state == InGameState::Pause
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            if let Ok((previous_button, mut previous_color)) = selected_query.get_single_mut() {
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
            }
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
