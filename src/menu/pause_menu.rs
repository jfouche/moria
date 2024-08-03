use super::*;
use crate::components::*;
use crate::cursor::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct PauseMenu;

// State used for the pause menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseMenuState {
    Main,
    Settings,
    SettingsSound,
    SettingsDisplay,
    #[default]
    Disabled,
}

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.init_state::<PauseMenuState>()
        .add_systems(OnEnter(InGameState::Pause), (ungrab_cursor, menu_setup))
        .add_systems(OnEnter(PauseMenuState::Main), spawn_pause_menu)
        .add_systems(OnExit(PauseMenuState::Main), despawn_all::<PauseMenu>)
        .add_systems(Update, menu_action.run_if(in_state(InGameState::Pause)));
}

fn menu_setup(mut menu_state: ResMut<NextState<PauseMenuState>>) {
    menu_state.set(PauseMenuState::Main);
}

fn spawn_pause_menu(commands: Commands) {
    spawn_popup(
        commands,
        "Moria - Pause",
        (Name::new("PauseMenu"), PauseMenu),
        |popup| {
            popup
                .spawn((button_bundle(), MenuButtonAction::PlayGame))
                .with_children(|parent| {
                    parent.spawn(button_text("Resume"));
                });
            popup
                .spawn((button_bundle(), MenuButtonAction::Settings))
                .with_children(|parent| {
                    parent.spawn(button_text("Settings"));
                });
            popup
                .spawn((button_bundle(), MenuButtonAction::QuitGame))
                .with_children(|parent| {
                    parent.spawn(button_text("Quit"));
                });
        },
    );
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    menu_state: Res<State<PauseMenuState>>,
    mut next_menu_state: ResMut<NextState<PauseMenuState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::QuitGame => {
                    next_menu_state.set(PauseMenuState::Disabled);
                    next_in_game_state.set(InGameState::Disabled);
                    next_game_state.set(GameState::Menu);
                }
                MenuButtonAction::PlayGame => {
                    next_menu_state.set(PauseMenuState::Disabled);
                    next_in_game_state.set(InGameState::Running);
                }
                MenuButtonAction::Settings => {
                    next_menu_state.set(PauseMenuState::Settings);
                }
                MenuButtonAction::SettingsSound => {
                    next_menu_state.set(PauseMenuState::SettingsSound);
                }
                MenuButtonAction::SettingsDisplay => {
                    next_menu_state.set(PauseMenuState::SettingsDisplay);
                }
                MenuButtonAction::BackToMainMenu => {
                    next_menu_state.set(PauseMenuState::Main);
                }
                MenuButtonAction::BackToSettings => {
                    next_menu_state.set(PauseMenuState::Settings);
                }
                _ => {}
            }
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        match *menu_state.get() {
            PauseMenuState::Main => {
                next_menu_state.set(PauseMenuState::Disabled);
                next_in_game_state.set(InGameState::Running);
            }
            PauseMenuState::Settings => {
                next_menu_state.set(PauseMenuState::Main);
            }
            PauseMenuState::SettingsDisplay => {
                next_menu_state.set(PauseMenuState::Settings);
            }
            PauseMenuState::SettingsSound => {
                next_menu_state.set(PauseMenuState::Settings);
            }
            _ => {}
        }
    }
}
