use super::*;
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
    bullets: Query<(&Bullet, &FireEmitter, &Transform)>,
    mut enemy_hit_events: EventWriter<EnemyHitEvent>,
    names: Query<DebugName>,
) {
    let mut enemies_hit = HashMap::new();
    collisions
        .read()
        // filter start event only
        .filter_map(start_event_filter)
        // filter Bullet collision ...
        .filter_map(|(&e1, &e2)| filter_either(&bullets, e1, e2))
        // ... with player emitter ...
        .filter(|(bullet_data, _bullet_entity, _other_entity)| {
            bullet_data.1 == &FireEmitter::Player
        })
        // ... colliding with enemy
        .filter_map(|(bullet_data, _bullet_entity, other_entity)| {
            warn!("bullet colling with {:?}", names.get(other_entity));
            enemy_colliders
                .get(other_entity)
                .map(|parent| (bullet_data, parent.get()))
                .ok()
        })
        // Manage enemy hit
        .for_each(|(bullet_data, enemy_entity)| {
            if enemies.get(enemy_entity).is_ok() {
                info!("enemy_hit_by_bullet {enemy_entity:?}");
                enemies_hit
                    .entry(enemy_entity)
                    .or_insert((bullet_data.2.translation, 0))
                    .1 += bullet_data.0.damage;
            }
        });

    for (entity, (pos, damage)) in enemies_hit.iter() {
        enemy_hit_events.send(EnemyHitEvent {
            entity: *entity,
            pos: *pos,
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
        .filter_map(|(&e1, &e2)| filter_either(&bullets, e1, e2))
        // ... From enemy, colliding with player ...
        .filter(|(bullet_data, _bullet_entity, other_entity)| {
            bullet_data.1 == &FireEmitter::Enemy && *other_entity == player_entity
        })
        .map(|(bullet_data, _bullet_entity, _player_entity)| bullet_data.0)
        // Manage Player hits
        .for_each(|bullet| {
            info!("player_hit_by_bullet");
            damage += bullet.damage;
        });

    if damage != 0 {
        player_hit_events.send(PlayerHitEvent { damage });
    }
}
