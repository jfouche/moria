use crate::components::*;
use crate::config::MazeConfig;
use crate::schedule::InGameLoadingSet;
use crate::GameState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        create_level.in_set(InGameLoadingSet::CreateLevel),
    );
}

fn create_level(
    mut commands: Commands,
    config: Res<MazeConfig>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    info!("create_level");
    let level = Level::new(config.cols, config.rows);
    commands.insert_resource(level);
    in_game_state.set(InGameState::Running);
    info!("create_level finished");
}
