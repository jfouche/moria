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

fn spawn_settings_menu(mut commands: Commands) {
    commands
        .spawn((centered(), Name::new("SettingsMenu"), SettingsMenu))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                for (action, text) in [
                    (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::SettingsDisplay, "Display"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ] {
                    menu.spawn((button_bundle(), action))
                        .with_children(|parent| {
                            parent.spawn(button_text(text));
                        });
                }
            });
        });
}
