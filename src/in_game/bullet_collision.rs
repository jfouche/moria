use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (enemy_hit_by_bullet, player_hit_by_bullet),
            despawn_bullet_after_collision,
        )
            .chain()
            .in_set(InGameSet::CollisionDetection),
    );
}

fn despawn_bullet_after_collision(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    bullets: Query<Entity, With<Bullet>>,
) {
    let mut bullets_hit = HashSet::new();
    collisions
        .read()
        .filter_map(start_event_filter)
        .for_each(|(&e1, &e2)| {
            for bullet_entity in bullets.iter() {
                if (e1 == bullet_entity) || (e2 == bullet_entity) {
                    bullets_hit.insert(bullet_entity);
                }
            }
        });

    for bullet in bullets_hit {
        commands.entity(bullet).despawn_recursive();
    }
}

///
/// Enemy hit by a bullet
///
fn enemy_hit_by_bullet(
    mut collisions: EventReader<CollisionEvent>,
    enemies: Query<(), With<Enemy>>,
    enemy_colliders: Query<&Parent, With<EnemyCollider>>,
    bullets: Query<(&Bullet, &FireEmitter)>,
    mut enemy_hit_events: EventWriter<EnemyHitEvent>,
) {
    let mut enemies_hit = HashMap::new();
    collisions
        .read()
        // filter start event only
        .filter_map(start_event_filter)
        // filter Bullet collision ...
        .filter_map(|(&e1, &e2)| bullet_filter(&bullets, e1, e2))
        // ... with enemy emitter ...
        .filter(|(_bullet, emitter, _other_entity)| emitter == &FireEmitter::Player)
        .map(|(bullet, _emitter, other_entity)| (bullet, other_entity))
        // ... colliding with player
        .filter_map(|(bullet, other_entity)| {
            enemy_colliders
                .get(other_entity)
                .map(|parent| (parent.get(), bullet))
                .ok()
        })
        // Manage enemy hit
        .for_each(|(enemy_entity, bullet)| {
            if enemies.get(enemy_entity).is_ok() {
                info!("enemy_hit_by_bullet {enemy_entity:?}");
                *enemies_hit.entry(enemy_entity).or_insert(0) += bullet.damage;
            }
        });

    for (entity, damage) in enemies_hit.iter() {
        enemy_hit_events.send(EnemyHitEvent {
            entity: *entity,
            damage: *damage,
        });
    }
}

///
/// Player hit by a bullet
///
fn player_hit_by_bullet(
    mut collisions: EventReader<CollisionEvent>,
    players: Query<Entity, With<Player>>,
    bullets: Query<(&Bullet, &FireEmitter)>,
    mut player_hit_events: EventWriter<PlayerHitEvent>,
) {
    let player_entity = players.get_single().expect("Player");
    let mut damage = 0;
    collisions
        .read()
        // filter start event only
        .filter_map(start_event_filter)
        // filter Bullet collision ...
        .filter_map(|(&e1, &e2)| bullet_filter(&bullets, e1, e2))
        // ... with enemy emitter ...
        .filter(|(_bullet, emitter, _other_entity)| emitter == &FireEmitter::Enemy)
        .map(|(bullet, _emitter, other_entity)| (bullet, other_entity))
        // ... colliding with player
        .filter(|(_bullet, other_entity)| *other_entity == player_entity)
        .map(|(bullet, _player_entity)| bullet)
        // Manage Player hits
        .for_each(|bullet| {
            info!("player_hit_by_bullet");
            damage += bullet.damage;
        });

    if damage != 0 {
        player_hit_events.send(PlayerHitEvent { damage });
    }
}

/// Filter CollisionEvent::Started events
fn start_event_filter(event: &CollisionEvent) -> Option<(&Entity, &Entity)> {
    match event {
        CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
        _ => None,
    }
}

/// Filter Bullet collision, returning a `([Bullet], [FireEmitter], [Entity])`
/// where Entity is the other collided Entity
fn bullet_filter(
    bullets: &Query<(&Bullet, &FireEmitter)>,
    e1: Entity,
    e2: Entity,
) -> Option<(Bullet, FireEmitter, Entity)> {
    bullets
        .get(e1)
        .map(|(&bullet, &emitter)| (bullet, emitter, e2))
        .or(bullets
            .get(e2)
            .map(|(&bullet, &emitter)| (bullet, emitter, e1)))
        .ok()
}
