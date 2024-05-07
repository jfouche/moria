use super::*;
use crate::ecs::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

pub struct DisplaySettingsPlugin<S>(pub S);

impl<S: States + Copy> Plugin for DisplaySettingsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), spawn_display_menu)
            .add_systems(OnExit(self.0), despawn_all::<OnDisplaySettingsMenuScreen>)
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
    mut commands: Commands,
    current_settings: Res<DisplaySettings>,
    current_exposure: Res<ExposureSettings>,
) {
    commands
        .spawn((centered(), OnDisplaySettingsMenuScreen))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                menu.spawn(menu_title("Display settings"));

                menu.spawn(button_text("Display"));
                menu.spawn(hsizer()).with_children(|sizer| {
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

                menu.spawn(button_text("Exposure"));
                menu.spawn(hsizer()).with_children(|sizer| {
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

                menu.spawn((button_bundle(), MenuButtonAction::BackToSettings))
                    .with_children(|parent| {
                        parent.spawn(button_text("Back"));
                    });
            });
        });
}
