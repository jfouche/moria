use bevy::prelude::*;

use crate::{despawn_all, GameState, InGameState};

use super::{
    button_bundle, button_text, main_panel_center, menu_title, menu_vertical, MenuButtonAction,
    PauseMenuState, BACKGROUND_COLOR,
};

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.init_state::<PauseMenuState>()
        .add_systems(OnEnter(InGameState::Pause), menu_setup)
        .add_systems(OnEnter(PauseMenuState::Main), spawn_pause_menu)
        .add_systems(
            OnExit(PauseMenuState::Main),
            despawn_all::<OnMainMenuScreen>,
        )
        .add_systems(Update, (menu_action).run_if(in_state(InGameState::Pause)));
}

fn menu_setup(mut commands: Commands, mut menu_state: ResMut<NextState<PauseMenuState>>) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    menu_state.set(PauseMenuState::Main);
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        // Entire screen
        .spawn((main_panel_center(), OnMainMenuScreen))
        .with_children(|parent| {
            parent
                // Main Menu
                .spawn(menu_vertical())
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(menu_title("Moria - Pause"));

                    parent
                        .spawn((button_bundle(), MenuButtonAction::PlayGame))
                        .with_children(|parent| {
                            parent.spawn(button_text("Resume"));
                        });
                    parent
                        .spawn((button_bundle(), MenuButtonAction::Settings))
                        .with_children(|parent| {
                            parent.spawn(button_text("Settings"));
                        });
                    parent
                        .spawn((button_bundle(), MenuButtonAction::QuitGame))
                        .with_children(|parent| {
                            parent.spawn(button_text("Quit"));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<PauseMenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::QuitGame => {
                    game_state.set(GameState::Menu);
                    menu_state.set(PauseMenuState::Disabled);
                }
                MenuButtonAction::PlayGame => {
                    game_state.set(GameState::InGame);
                    menu_state.set(PauseMenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(PauseMenuState::Settings),
                MenuButtonAction::SettingsSound => {
                    menu_state.set(PauseMenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(PauseMenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(PauseMenuState::Settings);
                }
                _ => {}
            }
        }
    }
}
