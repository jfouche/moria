use super::*;
use crate::{despawn_all, display::DisplaySettings};
use bevy::prelude::*;

#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MainMenuState::SettingsDisplay), spawn_display_menu)
        .add_systems(OnEnter(PauseMenuState::SettingsDisplay), spawn_display_menu)
        .add_systems(
            Update,
            setting_button::<DisplaySettings>.run_if(in_display_settings),
        )
        .add_systems(
            OnExit(MainMenuState::SettingsDisplay),
            despawn_all::<OnDisplaySettingsMenuScreen>,
        )
        .add_systems(
            OnExit(PauseMenuState::SettingsDisplay),
            despawn_all::<OnDisplaySettingsMenuScreen>,
        );
}

/// Condition that returns `true` if a menu is in a DisplaySettings state
fn in_display_settings(
    main_menu_state: Res<State<MainMenuState>>,
    pause_menu_state: Res<State<PauseMenuState>>,
) -> bool {
    *main_menu_state == MainMenuState::SettingsDisplay
        || *pause_menu_state == PauseMenuState::SettingsDisplay
}

fn spawn_display_menu(mut commands: Commands, settings: Res<DisplaySettings>) {
    commands
        .spawn((main_panel_center(), OnDisplaySettingsMenuScreen))
        .with_children(|parent| {
            parent.spawn(menu_vertical()).with_children(|parent| {
                parent.spawn(menu_title("Display settings"));
                for display in [DisplaySettings::Window, DisplaySettings::FullScreen] {
                    let mut btn = parent.spawn((button_bundle(), display));
                    btn.with_children(|parent| {
                        parent.spawn(button_text(&display.to_string()));
                    });
                    if *settings == display {
                        btn.insert(SelectedOption);
                    }
                }
                parent
                    .spawn((button_bundle(), MenuButtonAction::BackToSettings))
                    .with_children(|parent| {
                        parent.spawn(button_text("Back"));
                    });
            });
        });
}
