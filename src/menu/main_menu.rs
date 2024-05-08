use super::*;
use crate::ecs::*;
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
struct OnMainMenuScreen;

#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

#[derive(Component)]
struct OnSoundSettingsMenuScreen;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.init_state::<MainMenuState>()
        .add_systems(OnEnter(GameState::Menu), (menu_setup, ungrab_cursor))
        .add_systems(OnEnter(MainMenuState::Main), spawn_main_menu)
        .add_systems(OnExit(MainMenuState::Main), despawn_all::<OnMainMenuScreen>)
        .add_systems(Update, (menu_action).run_if(in_state(GameState::Menu)));
}

fn menu_setup(mut commands: Commands, mut menu_state: ResMut<NextState<MainMenuState>>) {
    commands.insert_resource(ClearColor(Color::BLACK));
    menu_state.set(MainMenuState::Main);
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((centered(), OnMainMenuScreen))
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
                MenuButtonAction::Settings => {
                    menu_state.set(MainMenuState::Settings);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MainMenuState::SettingsSound);
                }
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MainMenuState::SettingsDisplay);
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MainMenuState::Main);
                }
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MainMenuState::Settings);
                }
                _ => {}
            }
        }
    }
}
