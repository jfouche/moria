use crate::components::*;
use crate::schedule::InGameLoadingSet;
use crate::GameState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(CurrentLevel(0)).add_systems(
        OnEnter(GameState::InGame),
        create_level.in_set(InGameLoadingSet::CreateLevel),
    );
}

fn create_level(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    levels_config: Res<LevelsConfig>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    info!("create_level {:?}", *current_level);
    let level_config = levels_config
        .get(&current_level)
        .expect("Out of bound level");
    let mut level = Level::new(level_config.cols, level_config.rows);
    level.add_enemies(level_config.enemy_density);
    level.add_items(level_config.item_density);
    commands.insert_resource(level);
    in_game_state.set(InGameState::Running);
}
