use crate::{
    components::*,
    math::SignedAngle,
    schedule::{InGameLoadingSet, InGameSet},
};
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

/// Resource to store ray castings between Enemy and Player
#[derive(Resource, Default, Deref, DerefMut)]
struct EnemiesSeingPlayer(Vec<Entity>);

pub fn plugin(app: &mut App) {
    app.add_event::<EnemyHitEvent>()
        .add_event::<EnemyDeathEvent>()
        .init_resource::<EnemiesSeingPlayer>()
        .add_systems(Startup, (load_enemy_assets, load_impact_assets))
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

fn load_enemy_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_assets = EnemyAssets::from(asset_server.as_ref());
    commands.insert_resource(enemy_assets)
}

fn load_impact_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.1));
    let material = materials.add(Color::RED);
    let assets = ImpactAssets { mesh, material };
    commands.insert_resource(assets);
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<EnemyAssets>,
    weapons: Res<Weapons>,
    level: Res<Level>,
) {
    info!("spawn_enemies()");
    let weapon = weapons.get(WeaponType::EnemyGun);
    for &pos in level.enemies_start_pos() {
        commands.spawn(
            EnemyBundle::new(weapon.clone())
                .at(pos)
                .with_assets(&assets),
        );
        // .with_children(|parent| {
        //     for (collider, transform) in assets.colliders() {
        //         parent.spawn(EnemyColliderBundle::new(collider.clone(), *transform));
        //     }
        // });
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
    spatial_query: SpatialQuery,
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
                if let Ok(ray_dir) = Direction3d::new(player_center - ray_pos) {
                    let max_toi = ray_dir.length();
                    let solid = false;
                    let filter = SpatialQueryFilter::default(); // TODO: exclude items and enemies ?

                    {
                        // DEBUG
                        #[cfg(debug_assertions)]
                        gizmos.ray(ray_pos, *ray_dir, Color::WHITE);
                    }

                    if let Some(ray_hit_data) =
                        spatial_query.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                    {
                        if ray_hit_data.entity == player_collider_entity {
                            debug!("{enemy_entity:?} sees player");
                            enemies_seeing_player.push(enemy_entity);
                        }
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
    enemies: Query<(&Transform, &Weapon), (With<Enemy>, Without<Reload>)>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut ev_fire: EventWriter<FireEvent>,
) {
    let player_transform = player.get_single().expect("Player");
    for &enemy_entity in enemies_seeing_player.iter() {
        if let Ok((enemy_transform, weapon)) = enemies.get(enemy_entity) {
            let fire_origin = enemy_transform.translation + Enemy::weapon_offset();
            let fire_direction =
                player_transform.translation + Player::center_offset() - fire_origin;

            let event = weapon
                .fire()
                .from(fire_origin, FireEmitter::Enemy)
                .to(Direction3d::new(fire_direction).unwrap())
                .event();
            ev_fire.send(event);

            // Weapon reload
            commands.entity(enemy_entity).insert(Reload::from(weapon));
        }
    }
}

fn enemy_turns(
    enemies_seeing_player: Res<EnemiesSeingPlayer>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    #[cfg(debug_assertions)] mut gizmos: Gizmos,
) {
    let player_transform = player.get_single().expect("Player");
    for &enemy_entity in enemies_seeing_player.iter() {
        let mut enemy_transform = enemies.get_mut(enemy_entity).expect("Enemy");
        let angle = enemy_transform.signed_angle_with(*player_transform);

        // DEBUG
        #[cfg(debug_assertions)]
        {
            if angle > f32::EPSILON {
                info!("enemy_turns() {enemy_entity:?}: {angle}");
            }

            gizmos.ray(
                enemy_transform.translation,
                *enemy_transform.forward(),
                Color::WHITE,
            );

            // let mut test = *enemy_transform;
            // test.rotate_y(angle);
            // gizmos.ray(enemy_transform.translation, *test.forward(), Color::YELLOW);

            let mut test = *enemy_transform;
            test.rotate_y(-angle);
            gizmos.ray(enemy_transform.translation, *test.forward(), Color::ORANGE);

            // let mut test = *enemy_transform;
            // test.rotate_y(angle);
            // gizmos.ray(enemy_transform.translation, -*test.forward(), Color::OLIVE);

            // let mut test = *enemy_transform;
            // test.rotate_y(-angle);
            // gizmos.ray(enemy_transform.translation, -*test.forward(), Color::PINK);
        }

        enemy_transform.rotate_y(angle);
    }
}
