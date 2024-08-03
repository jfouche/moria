use super::*;
use crate::components::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct SettingsMenu;

pub struct SettingsPlugin<S>(pub S);

impl<S: States + Copy> Plugin for SettingsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), spawn_settings_menu)
            .add_systems(OnExit(self.0), despawn_all::<SettingsMenu>);
    }
}

fn spawn_settings_menu(commands: Commands) {
    spawn_popup(
        commands,
        "Settings",
        (Name::new("SettingsMenu"), SettingsMenu),
        |popup| {
            for (action, text) in [
                (MenuButtonAction::SettingsSound, "Sound"),
                (MenuButtonAction::SettingsDisplay, "Display"),
                (MenuButtonAction::BackToMainMenu, "Back"),
            ] {
                spawn_button(popup, text, action);
            }
        },
    );
}
