use super::*;
use crate::ecs::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

#[derive(Component)]
struct OnSoundSettingsMenuScreen;

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
        .add_systems(
            OnExit(PauseMenuState::Main),
            despawn_all::<OnMainMenuScreen>,
        )
        .add_systems(Update, (menu_action).run_if(in_state(InGameState::Pause)));
}

fn menu_setup(mut menu_state: ResMut<NextState<PauseMenuState>>) {
    menu_state.set(PauseMenuState::Main);
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((centered(), OnMainMenuScreen))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                menu.spawn(menu_title("Moria - Pause"));

                menu.spawn((button_bundle(), MenuButtonAction::PlayGame))
                    .with_children(|parent| {
                        parent.spawn(button_text("Resume"));
                    });
                menu.spawn((button_bundle(), MenuButtonAction::Settings))
                    .with_children(|parent| {
                        parent.spawn(button_text("Settings"));
                    });
                menu.spawn((button_bundle(), MenuButtonAction::QuitGame))
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
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::QuitGame => {
                    menu_state.set(PauseMenuState::Disabled);
                    in_game_state.set(InGameState::Disabled);
                    game_state.set(GameState::Menu);
                }
                MenuButtonAction::PlayGame => {
                    menu_state.set(PauseMenuState::Disabled);
                    in_game_state.set(InGameState::Running);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(PauseMenuState::Settings);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(PauseMenuState::SettingsSound);
                }
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(PauseMenuState::SettingsDisplay);
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(PauseMenuState::Main);
                }
                MenuButtonAction::BackToSettings => {
                    menu_state.set(PauseMenuState::Settings);
                }
                _ => {}
            }
        }
    }
}
