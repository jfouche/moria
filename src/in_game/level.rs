use crate::components::*;
use crate::schedule::InGameLoadingSet;
use crate::ui::{FaderBundle, FaderFinishEvent};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(CurrentLevel(0))
        // LoadLevel
        .add_systems(
            OnEnter(InGameState::LoadLevel),
            (show_level, create_level).in_set(InGameLoadingSet::CreateLevel),
        )
        .add_systems(Update, start_level.run_if(in_state(InGameState::LoadLevel)))
        // PlayerEndedLevel
        .add_systems(OnEnter(InGameState::PlayerEndedLevel), hide_level)
        .add_systems(
            Update,
            change_level.run_if(in_state(InGameState::PlayerEndedLevel)),
        );
}

const END_LEVEL_FADE_COLOR: Color = Color::rgba(0.0, 0.0, 0.8, 1.0);

fn hide_level(mut commands: Commands) {
    info!("hide_level()");
    commands.spawn(FaderBundle::new(Color::NONE, END_LEVEL_FADE_COLOR, 2.0));
}

fn show_level(mut commands: Commands) {
    info!("show_level()");
    commands.spawn(FaderBundle::new(END_LEVEL_FADE_COLOR, Color::NONE, 2.0));
}

fn create_level(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    levels_config: Res<LevelsConfig>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if let Some(level_config) = levels_config.get(&current_level) {
        info!("create_level {:?}", *current_level);
        let level = Level::new(level_config);
        commands.insert_resource(level);
        in_game_state.set(InGameState::LoadLevel);
    }
}

/// wait for fader to finish and start running
fn start_level(
    mut commands: Commands,
    mut events: EventReader<FaderFinishEvent>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for event in events.read() {
        if let Some(mut cmd) = commands.get_entity(event.entity) {
            info!("start_level() - despawn({:?})", event.entity);
            cmd.despawn();
        }
        in_game_state.set(InGameState::Running);
    }
}

/// Wait for fader to finish and try to change level
fn change_level(
    mut commands: Commands,
    mut events: EventReader<FaderFinishEvent>,
    mut current_level: ResMut<CurrentLevel>,
    levels_config: Res<LevelsConfig>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for event in events.read() {
        if let Some(mut cmd) = commands.get_entity(event.entity) {
            info!("change_level() - despawn({:?})", event.entity);
            cmd.despawn();
        }

        match levels_config.next_level(&current_level) {
            Some(next_level) => {
                // Go to next level
                *current_level = CurrentLevel(next_level);
                in_game_state.set(InGameState::LoadLevel);
            }
            None => {
                // It's the end
                in_game_state.set(InGameState::PlayerFinished);
            }
        }
    }
}
