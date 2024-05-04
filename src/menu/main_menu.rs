use super::*;
use crate::{despawn_all, GameState};
use bevy::{app::AppExit, prelude::*};

// State used for the main menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MainMenuState {
    Main,
    Settings,
    SettingsSound,
    #[default]
    Disabled,
}

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
    app.init_state::<MainMenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MainMenuState::Main), spawn_main_menu)
        .add_systems(OnExit(MainMenuState::Main), despawn_all::<OnMainMenuScreen>)
        .add_systems(Update, (menu_action).run_if(in_state(GameState::Menu)));
}

fn menu_setup(mut commands: Commands, mut menu_state: ResMut<NextState<MainMenuState>>) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    menu_state.set(MainMenuState::Main);
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        // Entire screen
        .spawn((main_panel_center(), OnMainMenuScreen))
        .with_children(|parent| {
            parent
                // Main Menu
                .spawn(menu_vertical())
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(menu_title("Moria"));

                    parent
                        .spawn((button_bundle(), MenuButtonAction::PlayGame))
                        .with_children(|parent| {
                            parent.spawn(button_text("New Game"));
                        });
                    parent
                        .spawn((button_bundle(), MenuButtonAction::Settings))
                        .with_children(|parent| {
                            parent.spawn(button_text("Settings"));
                        });
                    parent
                        .spawn((button_bundle(), MenuButtonAction::ExitApplication))
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
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::ExitApplication => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::PlayGame => {
                    game_state.set(GameState::InGame);
                    menu_state.set(MainMenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MainMenuState::Settings),
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MainMenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MainMenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MainMenuState::Settings);
                }
                _ => {}
            }
        }
    }
}