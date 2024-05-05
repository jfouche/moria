use super::*;
use crate::despawn_all;
use bevy::prelude::*;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MainMenuState::Settings), spawn_settings_menu)
        .add_systems(OnEnter(PauseMenuState::Settings), spawn_settings_menu)
        .add_systems(
            OnExit(MainMenuState::Settings),
            despawn_all::<OnSettingsMenuScreen>,
        )
        .add_systems(
            OnExit(PauseMenuState::Settings),
            despawn_all::<OnSettingsMenuScreen>,
        );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands
        .spawn((main_panel_center(), OnSettingsMenuScreen))
        .with_children(|parent| {
            parent.spawn(menu_vertical()).with_children(|parent| {
                for (action, text) in [
                    (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::SettingsDisplay, "Display"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ] {
                    parent
                        .spawn((button_bundle(), action))
                        .with_children(|parent| {
                            parent.spawn(button_text(text));
                        });
                }
            });
        });
}
