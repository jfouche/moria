use crate::components::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            enemy_hit_by_bullet,
            player_hit_by_bullet,
            despawn_bullet_after_collision,
        )
            .run_if(game_is_running),
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
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            for bullet_entity in bullets.iter() {
                if (e1 == bullet_entity) || (e2 == bullet_entity) {
                    bullets_hit.insert(bullet_entity);
                }
            }
        });

    for bullet in bullets_hit {
        commands.entity(bullet).despawn();
    }
}

///
/// Enemy hit by a bullet
///
fn enemy_hit_by_bullet(
    mut collisions: EventReader<CollisionEvent>,
    enemies: Query<Entity, With<Enemy>>,
    bullets: Query<(Entity, &Bullet)>,
    mut enemy_hit_events: EventWriter<EnemyHitEvent>,
) {
    let mut enemies_hit = HashMap::new();
    collisions
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            for enemy in enemies.iter() {
                for (bullet_entity, bullet) in bullets.iter() {
                    if (e1 == enemy && e2 == bullet_entity) || (e1 == bullet_entity && e2 == enemy)
                    {
                        info!("enemy_hit_by_bullet {enemy:?}");
                        *enemies_hit.entry(enemy).or_insert(0) += bullet.damage;
                    }
                }
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
    player: Query<Entity, With<Player>>,
    bullets: Query<(Entity, &Bullet)>,
    mut player_hit_events: EventWriter<PlayerHitEvent>,
) {
    let player = player.get_single().expect("Player");
    collisions
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            for (bullet_entity, bullet) in bullets.iter() {
                if (e1 == player && e2 == bullet_entity) || (e1 == bullet_entity && e2 == player) {
                    player_hit_events.send(PlayerHitEvent {
                        damage: bullet.damage,
                    });
                }
            }
        });
}
