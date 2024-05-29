use super::*;
use crate::{
    components::*,
    schedule::{InGameLoadingSet, InGameSet},
    ui::Fader,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(
            OnEnter(InGameState::LoadLevel),
            (despawn_all::<EndLevel>, spawn_end_level)
                .chain()
                .in_set(InGameLoadingSet::SpawnLevelEntities),
        )
        .add_systems(OnEnter(InGameState::PlayerFinished), spawn_fader)
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
    let assets = EndLevelAssets::new(meshes, materials);
    commands.insert_resource(assets);
}

fn spawn_end_level(mut commands: Commands, level: Res<Level>, assets: Res<EndLevelAssets>) {
    commands
        .spawn(
            EndLevelBundle::new()
                .at(level.end_position())
                .with_assets(&assets),
        )
        .with_children(|parent| {
            parent.spawn(EndLevelColliderBundle::default());
        });
}

fn player_ends_level(
    mut events: EventReader<CollisionEvent>,
    end_level_colliders: Query<Entity, With<EndLevelCollider>>,
    player_colliders: Query<Entity, With<PlayerCollider>>,
    mut in_game_next_state: ResMut<NextState<InGameState>>,
) {
    let player_collider_entity = player_colliders.get_single().expect("PlayerCollider");
    let end_level_collider_entity = end_level_colliders.get_single().expect("EndLevelCollider");
    events
        .read()
        .filter_map(start_event_filter)
        .filter(|(&e1, &e2)| {
            player_collider_entity.eq_either(e1, e2) && end_level_collider_entity.eq_either(e1, e2)
        })
        .for_each(|_| {
            warn!("END LEVEL");
            in_game_next_state.set(InGameState::PlayerEndedLevel);
        });
}

fn spawn_fader(mut commands: Commands) {
    let from = Color::NONE;
    let to = Color::BLACK;
    commands.spawn(Fader::new(from, to, 3.0));
}
