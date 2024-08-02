use crate::{
    assets_loader::assets_loading,
    components::*,
    math::SignedAngle,
    schedule::{InGameLoadingSet, InGameSet},
};
use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rapier3d::prelude::*;

/// Resource to store ray castings between Enemy and Player
#[derive(Resource, Default, Deref, DerefMut)]
struct EnemiesSeingPlayer(Vec<Entity>);

pub fn plugin(app: &mut App) {
    app.add_event::<EnemyHitEvent>()
        .add_event::<EnemyDeathEvent>()
        .init_resource::<EnemiesSeingPlayer>()
        .add_systems(
            Startup,
            (
                load_scene_assets::<EnemyAssets>("slime.glb#Scene0"),
                load_impact_assets,
            ),
        )
        .add_systems(
            Update,
            load_scene_colliders::<EnemyAssets>.run_if(assets_loading),
        )
        .add_systems(
            OnEnter(InGameState::LoadLevel),
            (despawn_all::<Enemy>, spawn_enemies)
                .chain()
                .in_set(InGameLoadingSet::SpawnLevelEntities),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<Enemy>)
        .add_systems(
            Update,
            (
                cast_rays_from_enemies,
                (
                    enemy_turns,
                    enemy_fires,
                    enemy_loose_life_on_hit,
                    spawn_impact_on_hit,
                ),
            )
                .chain()
                .in_set(InGameSet::EntityUpdate),
        )
        .add_systems(Update, on_death.in_set(InGameSet::DespawnEntities));
}

fn load_impact_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.1));
    let material = materials.add(Color::Srgba(RED));
    let assets = ImpactAssets { mesh, material };
    commands.insert_resource(assets);
}

fn spawn_enemies(mut commands: Commands, assets: Res<EnemyAssets>, level: Res<Level>) {
    info!("spawn_enemies()");
    for &pos in level.enemies.iter() {
        commands
            .spawn(
                EnemyBundle::new(WeaponType::EnemyGun)
                    .at(pos)
                    .with_assets(&assets),
            )
            .with_children(|parent| {
                for (collider, transform) in assets.colliders() {
                    parent.spawn(EnemyColliderBundle::new(collider.clone(), *transform));
                }
            });
    }
}

fn enemy_loose_life_on_hit(
    mut hit_events: EventReader<EnemyHitEvent>,
    mut enemies: Query<(&mut Life, &Transform), With<Enemy>>,
    mut death_events: EventWriter<EnemyDeathEvent>,
) {
    for event in hit_events.read() {
        if let Ok((mut life, transform)) = enemies.get_mut(event.entity) {
            info!("enemy_loose_life_on_hit : {:?}", event.entity);
            life.hit(event.damage);
            if life.is_dead() {
                death_events.send(EnemyDeathEvent {
                    entity: event.entity,
                    _pos: transform.translation,
                });
            }
        }
    }
}

fn spawn_impact_on_hit(
    mut commands: Commands,
    mut hit_events: EventReader<EnemyHitEvent>,
    assets: Res<ImpactAssets>,
) {
    for event in hit_events.read() {
        info!("spawn_impact_on_hit");
        commands.spawn((
            PbrBundle {
                transform: Transform::from_translation(event.pos),
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                ..default()
            },
            LifeTime::new(1.0),
        ));
    }
}

///
/// Despawn Enemy on death
///
fn on_death(mut commands: Commands, mut death_events: EventReader<EnemyDeathEvent>) {
    death_events.read().map(|ev| ev.entity).for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

///
/// Ray cast from all enemies to Player
///
/// It stores the enemies seeing player in [EnemiesSeingPlayer]
///
fn cast_rays_from_enemies(
    enemies: Query<(Entity, &Transform, &Children), With<Enemy>>,
    enemy_colliders: Query<&Parent, With<EnemyCollider>>,
    players: Query<&Transform, With<Player>>,
    player_colliders: Query<Entity, With<PlayerCollider>>,
    rapier_context: Res<RapierContext>,
    mut enemies_seeing_player: ResMut<EnemiesSeingPlayer>,
    #[cfg(debug_assertions)] mut gizmos: Gizmos,
) {
    enemies_seeing_player.clear();
    let player_transform = players.get_single().expect("Player");
    let player_collider_entity = player_colliders.get_single().expect("PlayerCollider");
    let player_center = player_transform.translation + Player::center_offset();
    for (enemy_entity, enemy_transform, children) in enemies.iter() {
        match children
            .iter()
            .find(|&child_entity| enemy_colliders.get(*child_entity).is_ok())
        {
            Some(enemy_collider_entity) => {
                let ray_pos = enemy_transform.translation + Enemy::center_offset();
                let ray_dir = player_center - ray_pos;

                let max_toi = ray_dir.length();
                let solid = false;
                let filter = QueryFilter::new()
                    .exclude_sensors()
                    .exclude_collider(*enemy_collider_entity);

                {
                    // DEBUG
                    #[cfg(debug_assertions)]
                    gizmos.ray(ray_pos, ray_dir, Color::WHITE);
                }

                if let Some((entity, _toi)) =
                    rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                {
                    if entity == player_collider_entity {
                        debug!("{enemy_entity:?} sees player");
                        enemies_seeing_player.push(enemy_entity);
                    } else {
                        debug!("{enemy_entity:?} sees {entity:?}");
                    }
                }
            }
            None => warn!("EnemyCollider not found in Enemy children"),
        }
    }
}

fn enemy_fires(
    mut commands: Commands,
    enemies_seeing_player: Res<EnemiesSeingPlayer>,
    enemies: Query<(&Transform, &WeaponType), (With<Enemy>, Without<Reload>)>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    weapons: Res<Weapons>,
    level: Res<Level>,
    mut ev_fire: EventWriter<FireEvent>,
) {
    let player_transform = player.get_single().expect("Player");
    for &enemy_entity in enemies_seeing_player.iter() {
        if let Ok((enemy_transform, weapon_type)) = enemies.get(enemy_entity) {
            let fire_origin = enemy_transform.translation + Enemy::weapon_offset();
            let fire_direction =
                player_transform.translation + Player::center_offset() - fire_origin;

            if let Ok(direction) = Dir3::new(fire_direction) {
                let event = FireEvent {
                    weapon_type: *weapon_type,
                    from: FireEmitter::Enemy,
                    origin: fire_origin,
                    direction,
                    bonus: level.enemy_bonus,
                };
                ev_fire.send(event);

                // Weapon reload
                let weapon = weapons.get(*weapon_type);
                commands
                    .entity(enemy_entity)
                    .insert(Reload::new(weapon, level.enemy_bonus));
            }
        }
    }
}

fn enemy_turns(
    enemies_seeing_player: Res<EnemiesSeingPlayer>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player.get_single().expect("Player");
    for &enemy_entity in enemies_seeing_player.iter() {
        let mut enemy_transform = enemies.get_mut(enemy_entity).expect("Enemy");
        let angle = enemy_transform.signed_angle_with(*player_transform);
        enemy_transform.rotate_y(angle);
    }
}
