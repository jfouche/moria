use super::*;
use crate::components::*;
use crate::cursor::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct LevelWinMenu;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_systems(Update, on_player_death.run_if(game_is_running))
        .add_systems(
            OnEnter(InGameState::PlayerFinished),
            (ungrab_cursor, spawn_level_win_menu),
        )
        .add_systems(
            OnExit(InGameState::PlayerFinished),
            despawn_all::<LevelWinMenu>,
        )
        .add_systems(
            Update,
            menu_action.run_if(in_state(InGameState::PlayerFinished)),
        );
}

fn spawn_level_win_menu(mut commands: Commands) {
    commands
        .spawn((centered(), Name::new("LevelWinMenu"), LevelWinMenu))
        .with_children(|wnd| {
            wnd.spawn(menu()).with_children(|menu| {
                menu.spawn(menu_title("You Win !"));
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
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed && *menu_button_action == MenuButtonAction::QuitGame
        {
            in_game_state.set(InGameState::Disabled);
            game_state.set(GameState::Menu);
        }
    }
}

fn on_player_death(
    mut death_events: EventReader<PlayerDeathEvent>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for _ in death_events.read() {
        in_game_state.set(InGameState::PlayerDied);
    }
}
