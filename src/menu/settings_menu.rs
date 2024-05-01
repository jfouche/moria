use bevy::prelude::*;

use crate::despawn_all;

use super::{
    button_bundle, button_text, main_panel_center, menu_vertical, MenuButtonAction, MenuState,
};

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Settings), spawn_settings_menu)
        .add_systems(
            OnExit(MenuState::Settings),
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
