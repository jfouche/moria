use super::*;
use crate::{
    components::*,
    schedule::{InGameLoadingSet, InGameSet},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(
            OnEnter(GameState::InGame),
            spawn_end_level.in_set(InGameLoadingSet::SpawnLevelEntities),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<EndLevel>)
        .add_systems(
            Update,
            player_ends_level.in_set(InGameSet::CollisionDetection),
        );
}

fn load_assets(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = EndLevelAssets::load(meshes, materials);
    commands.insert_resource(assets);
}

fn spawn_end_level(mut commands: Commands, level: Res<Level>, assets: Res<EndLevelAssets>) {
    commands.spawn(
        EndLevelBundle::new()
            .at(level.end_position())
            .with_assets(&assets),
    );
}

fn player_ends_level(
    mut events: EventReader<CollisionEvent>,
    end_level: Query<Entity, With<EndLevel>>,
    player: Query<Entity, With<Player>>,
    mut in_game_next_state: ResMut<NextState<InGameState>>,
) {
    let player_entity = player.get_single().expect("Player");
    let end_level_entity = end_level.get_single().expect("EndLevel");
    events
        .read()
        .filter_map(start_event_filter)
        .filter(|(&e1, &e2)| {
            (e1 == player_entity && e2 == end_level_entity)
                || (e2 == player_entity && e1 == end_level_entity)
        })
        .for_each(|(_, _)| {
            in_game_next_state.set(InGameState::PlayerFinished);
        });
}
