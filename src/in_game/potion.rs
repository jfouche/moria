use crate::{assets_loader::assets_loading, components::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::InGame), spawn_potions)
        .add_systems(OnExit(GameState::InGame), despawn_all::<Potion>)
        .add_systems(Update, load_colliders.run_if(assets_loading))
        .add_systems(Update, player_take_potion.run_if(game_is_running));
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets_register: ResMut<AssetsLoaderRegister>,
) {
    assets_register.register::<PotionAssets>();
    let assets = PotionAssets::load(&asset_server);
    commands.insert_resource(assets);
}

fn load_colliders(
    scenes: ResMut<Assets<Scene>>,
    meshes: ResMut<Assets<Mesh>>,
    mut assets: ResMut<PotionAssets>,
    mut event_writer: EventWriter<AssetsLoadedEvent>,
) {
    if assets.just_loaded(scenes, meshes) {
        info!("load_colliders() send event");
        event_writer.send(AssetsLoadedEvent::from::<PotionAssets>());
    }
}

fn spawn_potions(mut commands: Commands, assets: Res<PotionAssets>) {
    let pos = Position(1, 0);
    commands
        .spawn(
            PotionBundle::new(Potion::Life(30))
                .at(pos)
                .with_assets(&assets),
        )
        .with_children(|parent| {
            for (collider, transform) in assets.colliders() {
                parent.spawn(PotionColliderBundle::new(collider.clone(), *transform));
            }
        });
}

fn player_take_potion(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    potions: Query<&Potion>,
    potion_colliders: Query<&Parent, With<PotionCollider>>,
    mut player: Query<(Entity, &mut Life), With<Player>>,
) {
    let (player_entity, mut player_life) = player.get_single_mut().expect("Player");
    events
        .read()
        // Only accept Starting collision
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        // Filter Player / PotionCollider collision, return parent entity, ie. Potion
        .filter_map(|(&e1, &e2)| {
            if e1 == player_entity {
                potion_colliders.get(e2).map(|parent| parent.get()).ok()
            } else if e2 == player_entity {
                potion_colliders.get(e1).map(|parent| parent.get()).ok()
            } else {
                None
            }
        })
        // Retrieve the Potion associated with potion_entity
        .for_each(|potion_entity| {
            if let Ok(potion) = potions.get(potion_entity) {
                match potion {
                    Potion::Life(life) => player_life.add(*life),
                }
                commands.entity(potion_entity).despawn_recursive();
            }
        });
}
