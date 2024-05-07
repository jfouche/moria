use super::*;
use crate::ecs::*;
use bevy::prelude::*;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

pub struct SettingsPlugin<S>(pub S);

impl<S: States + Copy> Plugin for SettingsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), spawn_settings_menu)
            .add_systems(OnExit(self.0), despawn_all::<OnSettingsMenuScreen>);
    }
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
