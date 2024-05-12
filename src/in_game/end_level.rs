use crate::components::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::InGame), spawn_end_level)
        .add_systems(OnExit(GameState::InGame), despawn_all::<EndLevel>)
        .add_systems(Update, player_ends_level.run_if(game_is_running));
}

fn load_assets(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = EndLevelAssets::load(meshes, materials);
    commands.insert_resource(assets);
}

fn spawn_end_level(mut commands: Commands, assets: Res<EndLevelAssets>) {
    let pos = Position(3, 3);
    commands.spawn(EndLevelBundle::new().at(pos).with_assets(&assets));
}

fn player_ends_level(
    mut events: EventReader<CollisionEvent>,
    end_level: Query<Entity, With<EndLevel>>,
    player: Query<Entity, With<Player>>,
    mut in_game_next_state: ResMut<NextState<InGameState>>,
) {
    let player_entity = player.get_single().expect("Player");
    events
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            let player_finished = if e1 == player_entity {
                end_level.get(e2).is_ok()
            } else if e2 == player_entity {
                end_level.get(e1).is_ok()
            } else {
                false
            };
            if player_finished {
                in_game_next_state.set(InGameState::PlayerFinished);
            }
        });
}
