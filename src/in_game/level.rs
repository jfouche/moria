use crate::components::*;
use crate::schedule::InGameLoadingSet;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(CurrentLevel(0))
        .add_systems(
            OnEnter(InGameState::LoadLevel),
            create_level.in_set(InGameLoadingSet::CreateLevel),
        )
        .add_systems(Update, start_level.run_if(in_state(InGameState::LoadLevel)))
        .add_systems(
            OnEnter(InGameState::PlayerEndedLevel),
            (change_level, create_level)
                .chain()
                .in_set(InGameLoadingSet::CreateLevel),
        );
}

fn create_level(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    levels_config: Res<LevelsConfig>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if let Some(level_config) = levels_config.get(&current_level) {
        info!("create_level {:?}", *current_level);
        let mut level = Level::new(level_config.cols, level_config.rows);
        level.add_enemies(level_config.enemy_density);
        level.add_items(level_config.item_density);
        commands.insert_resource(level);
        in_game_state.set(InGameState::LoadLevel);
    }
}

fn start_level(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Running);
}

fn change_level(
    mut current_level: ResMut<CurrentLevel>,
    levels_config: Res<LevelsConfig>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if levels_config.has_next(&current_level) {
        // Go to next level
        **current_level += 1;
        in_game_state.set(InGameState::LoadLevel)
    } else {
        // It's the end
        in_game_state.set(InGameState::PlayerFinished);
    }
}
