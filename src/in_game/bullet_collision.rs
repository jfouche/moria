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
    enemies: Query<(), With<Enemy>>,
    enemy_colliders: Query<&Parent, With<EnemyCollider>>,
    bullets: Query<(&Bullet, &FireEmitter)>,
    mut enemy_hit_events: EventWriter<EnemyHitEvent>,
) {
    let mut enemies_hit = HashMap::new();
    collisions
        .read()
        // Only accept Starting collision
        .filter_map(start_event_filter)
        // Filter Bullet / EnemyCollider collision, returns (Enemy entity, Bullet)
        .filter_map(|(&e1, &e2)| {
            let check_entity = |e1: Entity, e2: Entity| match bullets.get(e1) {
                Ok((bullet, FireEmitter::Player)) => enemy_colliders
                    .get(e2)
                    .map(|parent| (parent.get(), bullet))
                    .ok(),
                _ => None,
            };
            check_entity(e1, e2).or(check_entity(e2, e1))
        })
        // Manage enemy hit
        .for_each(|(enemy_entity, &bullet)| {
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
    players: Query<(), With<Player>>,
    player_colliders: Query<&Parent, With<PlayerCollider>>,
    bullets: Query<(&Bullet, &FireEmitter)>,
    mut player_hit_events: EventWriter<PlayerHitEvent>,
) {
    let mut damage = 0;
    collisions
        .read()
        // Only accept Starting collision
        .filter_map(start_event_filter)
        // Filter Bullet / PlayerCollider collision, returns (Player entity, Bullet)
        .filter_map(|(&e1, &e2)| {
            let check_entity = |e1: Entity, e2: Entity| match bullets.get(e1) {
                Ok((bullet, FireEmitter::Enemy)) => player_colliders
                    .get(e2)
                    .map(|parent| (parent.get(), bullet))
                    .ok(),
                _ => None,
            };
            check_entity(e1, e2).or(check_entity(e2, e1))
        })
        .for_each(|(player_entity, &bullet)| {
            if players.get(player_entity).is_ok() {
                info!("player_hit_by_bullet");
                damage += bullet.damage;
            }
        });

    if damage != 0 {
        player_hit_events.send(PlayerHitEvent { damage });
    }
}

fn start_event_filter(event: &CollisionEvent) -> Option<(&Entity, &Entity)> {
    match event {
        CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
        _ => None,
    }
}

// struct CollisionChecker<'a, 'w: 'a, 's: 'a, T: Component> {
//     from: FireEmitter,
//     bullets: &'a Query<'w, 's, (&'s Bullet, &'s FireEmitter)>,
//     colliders: &'a Query<'w, 's, &'s Parent, With<T>>,
// }

// impl<'a, 'w: 'a, 's: 'a, T: Component> CollisionChecker<'a, 'w, 's, T> {
//     fn new(
//         from: FireEmitter,
//         bullets: &'a Query<'w, 's, (&'s Bullet, &'s FireEmitter)>,
//         colliders: &'a Query<'w, 's, &'s Parent, With<T>>,
//     ) -> Self {
//         CollisionChecker {
//             from,
//             bullets,
//             colliders,
//         }
//     }

//     fn check_inner(&self, e1: &Entity, e2: &Entity) -> Option<(Entity, &'s Bullet)> {
//         match self.bullets.get(*e1) {
//             Ok((bullet, from)) => self
//                 .colliders
//                 .get(*e2)
//                 .map(|parent| (parent.get(), bullet))
//                 .ok(),
//             _ => None,
//         }
//     }

//     fn check(&self, e1: &Entity, e2: &Entity) -> Option<(Entity, &'s Bullet)> {
//         self.check_inner(e1, e2).or(self.check_inner(e2, e1))
//     }
// }
