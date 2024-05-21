use crate::{
    assets_loader::assets_loading,
    components::*,
    schedule::{InGameLoadingSet, InGameSet},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::start_event_filter;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Startup,
        load_scene_assets::<PotionAssets>("potion.glb#Scene0"),
    )
    .add_systems(
        Update,
        load_scene_colliders::<PotionAssets>.run_if(assets_loading),
    )
    .add_systems(
        OnEnter(GameState::InGame),
        spawn_items.in_set(InGameLoadingSet::SpawnLevelEntities),
    )
    .add_systems(OnExit(GameState::InGame), despawn_all::<Potion>)
    .add_systems(
        Update,
        player_take_potion.in_set(InGameSet::CollisionDetection),
    );
}

fn spawn_items(mut commands: Commands, assets: Res<PotionAssets>, level: Res<Level>) {
    info!("spawn_items()");
    for (&pos, item) in level.items() {
        match item {
            Item::Potion(potion) => {
                commands
                    .spawn(
                        PotionBundle::new(potion.clone())
                            .at(pos)
                            .with_assets(&assets),
                    )
                    .with_children(|parent| {
                        for (collider, transform) in assets.colliders() {
                            parent.spawn(PotionColliderBundle::new(collider.clone(), *transform));
                        }
                    });
            }
        }
    }
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
        .filter_map(start_event_filter)
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
