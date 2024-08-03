use super::*;
use crate::components::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct DisplaySettingsMenu;

/// generic plugin to spawn "Display settings", depending on state `S`
pub struct DisplaySettingsPlugin<S>(pub S);

impl<S: States + Copy> Plugin for DisplaySettingsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), spawn_display_menu)
            .add_systems(OnExit(self.0), despawn_all::<DisplaySettingsMenu>)
            .add_systems(
                Update,
                (
                    setting_button::<DisplaySettings>,
                    setting_button::<ExposureSettings>,
                )
                    .run_if(in_state(self.0)),
            );
    }
}

fn spawn_display_menu(
    commands: Commands,
    current_settings: Res<DisplaySettings>,
    current_exposure: Res<ExposureSettings>,
) {
    spawn_popup(
        commands,
        "Display settings",
        (Name::new("DisplaySettingsMenu"), DisplaySettingsMenu),
        |popup| {
            popup.spawn(button_text("Display"));
            popup.spawn(hsizer()).with_children(|sizer| {
                for display in [DisplaySettings::Window, DisplaySettings::FullScreen] {
                    let mut btn = sizer.spawn((button_bundle(), display));
                    btn.with_children(|parent| {
                        parent.spawn(button_text(&display.to_string()));
                    });
                    if *current_settings == display {
                        btn.insert(SelectedOption);
                    }
                }
            });

            popup.spawn(button_text("Exposure"));
            popup.spawn(hsizer()).with_children(|sizer| {
                for exposure in [
                    ExposureSettings::Dark,
                    ExposureSettings::Medium,
                    ExposureSettings::Light,
                ] {
                    let mut btn = sizer.spawn((button_bundle(), exposure));
                    btn.with_children(|parent| {
                        parent.spawn(button_text(&exposure.to_string()));
                    });
                    if *current_exposure == exposure {
                        btn.insert(SelectedOption);
                    }
                }
            });

            spawn_button(popup, "Back", MenuButtonAction::BackToSettings);
        },
    );
}
