use bevy::prelude::*;

use crate::{GameState, InGameState};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum InGameLoadingSet {
    CreateLevel,
    SpawnLevelEntities,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdate,
    CollisionDetection,
    DespawnEntities,
}

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(GameState::InGame),
        (
            InGameLoadingSet::CreateLevel,
            // apply_deffer will be added here
            InGameLoadingSet::SpawnLevelEntities,
        )
            .chain(),
    )
    .configure_sets(
        Update,
        (
            InGameSet::DespawnEntities,
            // apply_deffer will be added here
            InGameSet::UserInput,
            InGameSet::EntityUpdate,
            InGameSet::CollisionDetection,
        )
            .chain()
            .run_if(game_is_running),
    )
    .add_systems(
        OnEnter(GameState::InGame),
        apply_deferred
            .after(InGameLoadingSet::CreateLevel)
            .before(InGameLoadingSet::SpawnLevelEntities),
    )
    .add_systems(
        Update,
        apply_deferred
            .after(InGameSet::DespawnEntities)
            .before(InGameSet::UserInput),
    );
}

fn game_is_running(
    game_state: Res<State<GameState>>,
    in_game_state: Res<State<InGameState>>,
) -> bool {
    *game_state == GameState::InGame && *in_game_state == InGameState::Running
}
