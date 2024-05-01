use bevy::prelude::*;

use crate::despawn_all;

use super::{
    button_bundle, button_style, button_text, button_text_style, main_panel_center, menu_vertical,
    setting_button, MenuButtonAction, MenuState, SelectedOption,
};

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

#[derive(Component)]
struct OnSoundSettingsMenuScreen;

pub fn plugin(app: &mut App) {
    app.insert_resource(Volume(7))
        .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        .add_systems(
            Update,
            setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
        )
        .add_systems(
            OnExit(MenuState::SettingsSound),
            despawn_all::<OnSoundSettingsMenuScreen>,
        );
}

fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
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
                                Volume(volume_setting),
                            ));
                            if *volume == Volume(volume_setting) {
                                entity.insert(SelectedOption);
                            }
                        }
                    });
                parent
                    .spawn((button_bundle(), MenuButtonAction::BackToSettings))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Back", button_text_style()));
                    });
            });
        });
}
