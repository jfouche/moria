use super::*;
use crate::ecs::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct OnSoundSettingsMenuScreen;

pub struct SoundSettingsPlugin<S>(pub S);

impl<S: States + Copy> Plugin for SoundSettingsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), spawn_sound_settings_menu)
            .add_systems(OnExit(self.0), despawn_all::<OnSoundSettingsMenuScreen>)
            .add_systems(
                Update,
                setting_button::<AudioVolume>.run_if(in_state(self.0)),
            );
    }
}

fn spawn_sound_settings_menu(mut commands: Commands, volume: Res<AudioVolume>) {
    commands
        .spawn((centered(), OnSoundSettingsMenuScreen))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                menu.spawn(menu_title("Sound settings"));

                menu.spawn(button_text("Volume"));
                menu.spawn(hsizer()).with_children(|sizer| {
                    for volume_setting in AudioVolume::range() {
                        let mut entity = sizer.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(65.0),
                                    ..button_style()
                                },
                                ..button_bundle()
                            },
                            AudioVolume(volume_setting),
                        ));
                        if *volume == AudioVolume(volume_setting) {
                            entity.insert(SelectedOption);
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
