use super::*;
use crate::components::*;
use crate::cursor::*;
use crate::schedule::InGameSet;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct LevelWinMenu;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_systems(Update, on_player_death.in_set(InGameSet::EntityUpdate))
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

fn spawn_level_win_menu(commands: Commands) {
    spawn_popup(
        commands,
        "Display settings",
        (Name::new("LevelWinMenu"), LevelWinMenu),
        |popup| spawn_button(popup, "Quit", MenuButtonAction::QuitGame),
    );
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
