use super::*;
use crate::components::*;
use crate::cursor::*;
use crate::schedule::InGameSet;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct DeathMenu;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_systems(Update, on_player_death.in_set(InGameSet::EntityUpdate))
        .add_systems(
            OnEnter(InGameState::PlayerDied),
            (ungrab_cursor, spawn_death_menu),
        )
        .add_systems(OnExit(InGameState::PlayerDied), despawn_all::<DeathMenu>)
        .add_systems(
            Update,
            menu_action.run_if(in_state(InGameState::PlayerDied)),
        );
}

fn spawn_death_menu(commands: Commands) {
    spawn_popup(
        commands,
        "You died !",
        (Name::new("DeathMenu"), DeathMenu),
        |popup| {
            popup.spawn(popup_text_content("This is the end of your journey..."));
            spawn_button(popup, "Quit", MenuButtonAction::QuitGame);
        },
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
