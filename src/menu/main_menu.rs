use super::*;
use crate::components::*;
use crate::cursor::*;
use crate::ui::*;

use bevy::{app::AppExit, prelude::*};

// State used for the main menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
pub enum MainMenuState {
    Main,
    Settings,
    SettingsSound,
    SettingsDisplay,
    #[default]
    Disabled,
}

#[derive(Component)]
struct MainMenu;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.init_state::<MainMenuState>()
        .add_systems(OnEnter(GameState::Menu), (menu_setup, ungrab_cursor))
        .add_systems(OnEnter(MainMenuState::Main), spawn_main_menu)
        .add_systems(OnExit(MainMenuState::Main), despawn_all::<MainMenu>)
        .add_systems(Update, (menu_action).run_if(in_state(GameState::Menu)));
}

fn menu_setup(mut commands: Commands, mut menu_state: ResMut<NextState<MainMenuState>>) {
    commands.insert_resource(ClearColor(Color::BLACK));
    menu_state.set(MainMenuState::Main);
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((centered(), Name::new("MainMenu"), MainMenu))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                // Display the game name
                menu.spawn(menu_title("Moria"));

                menu.spawn((button_bundle(), MenuButtonAction::PlayGame))
                    .with_children(|parent| {
                        parent.spawn(button_text("New Game"));
                    });
                menu.spawn((button_bundle(), MenuButtonAction::Settings))
                    .with_children(|parent| {
                        parent.spawn(button_text("Settings"));
                    });
                menu.spawn((button_bundle(), MenuButtonAction::ExitApplication))
                    .with_children(|parent| {
                        parent.spawn(button_text("Exit"));
                    });
            });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    menu_state: Res<State<MainMenuState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::ExitApplication => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::PlayGame => {
                    next_game_state.set(GameState::InGame);
                    next_menu_state.set(MainMenuState::Disabled);
                }
                MenuButtonAction::Settings => {
                    next_menu_state.set(MainMenuState::Settings);
                }
                MenuButtonAction::SettingsSound => {
                    next_menu_state.set(MainMenuState::SettingsSound);
                }
                MenuButtonAction::SettingsDisplay => {
                    next_menu_state.set(MainMenuState::SettingsDisplay);
                }
                MenuButtonAction::BackToMainMenu => {
                    next_menu_state.set(MainMenuState::Main);
                }
                MenuButtonAction::BackToSettings => {
                    next_menu_state.set(MainMenuState::Settings);
                }
                _ => {}
            }
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        match *menu_state.get() {
            MainMenuState::Settings => {
                next_menu_state.set(MainMenuState::Main);
            }
            MainMenuState::SettingsDisplay => {
                next_menu_state.set(MainMenuState::Settings);
            }
            MainMenuState::SettingsSound => {
                next_menu_state.set(MainMenuState::Settings);
            }
            _ => {}
        }
    }
}
