use super::*;
use crate::{despawn_all, in_game::AudioVolume};
use bevy::prelude::*;

#[derive(Component)]
struct OnSoundSettingsMenuScreen;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(MainMenuState::SettingsSound),
        sound_settings_menu_setup,
    )
    .add_systems(
        OnEnter(PauseMenuState::SettingsSound),
        sound_settings_menu_setup,
    )
    .add_systems(
        Update,
        setting_button::<AudioVolume>.run_if(in_state(MainMenuState::SettingsSound)),
    )
    .add_systems(
        OnExit(MainMenuState::SettingsSound),
        despawn_all::<OnSoundSettingsMenuScreen>,
    )
    .add_systems(
        OnExit(PauseMenuState::SettingsSound),
        despawn_all::<OnSoundSettingsMenuScreen>,
    );
}

fn sound_settings_menu_setup(mut commands: Commands, volume: Res<AudioVolume>) {
    commands
        .spawn((main_panel_center(), OnSoundSettingsMenuScreen))
        .with_children(|parent| {
            parent.spawn(menu_vertical()).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(button_text("Volume"));
                        for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                            let mut entity = parent.spawn((
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
                parent
                    .spawn((button_bundle(), MenuButtonAction::BackToSettings))
                    .with_children(|parent| {
                        parent.spawn(button_text("Back"));
                    });
            });
        });
}
