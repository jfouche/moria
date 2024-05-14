use crate::{assets_loader::assets_loading, components::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Resource to store ray castings between Enemy and Player
#[derive(Resource, Default, Deref, DerefMut)]
struct EnemiesSeingPlayer(Vec<Entity>);

pub fn plugin(app: &mut App) {
    app.add_event::<EnemyHitEvent>()
        .add_event::<EnemyDeathEvent>()
        .init_resource::<EnemiesSeingPlayer>()
        .add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::InGame), spawn_enemy)
        .add_systems(OnExit(GameState::InGame), despawn_all::<Enemy>)
        .add_systems(PreUpdate, cast_rays_from_enemies.run_if(game_is_running))
        .add_systems(Update, load_colliders.run_if(assets_loading))
        .add_systems(
            Update,
            (on_hit, on_death, enemy_fires).run_if(game_is_running),
        );
}

fn load_assets(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut assets_register: ResMut<AssetsLoaderRegister>,
) {
    assets_register.register::<EnemyAssets>();
    let assets = EnemyAssets::load(&assets_server);
    commands.insert_resource(assets);
}

fn load_colliders(
    scenes: ResMut<Assets<Scene>>,
    meshes: ResMut<Assets<Mesh>>,
    mut assets: ResMut<EnemyAssets>,
    mut event_writer: EventWriter<AssetsLoadedEvent>,
) {
    if assets.just_loaded(scenes, meshes) {
        event_writer.send(AssetsLoadedEvent::from::<EnemyAssets>());
    }
}

fn spawn_enemy(mut commands: Commands, assets: Res<EnemyAssets>, weapons: Res<Weapons>) {
    info!("spawn_enemy()");
    let pos = Position(2, 2);
    let weapon = weapons.get(WeaponType::Gun);
    commands
        .spawn(EnemyBundle::new(weapon).at(pos).with_assets(&assets))
        .with_children(|parent| {
            for (collider, transform) in assets.colliders() {
                parent.spawn(EnemyColliderBundle::new(collider.clone(), *transform));
            }
        });
}

///
/// Enemy is hit
///
fn on_hit(
    mut hit_events: EventReader<EnemyHitEvent>,
    mut enemies: Query<(Entity, &mut Life, &Transform), With<Enemy>>,
    mut death_events: EventWriter<EnemyDeathEvent>,
) {
    for event in hit_events.read() {
        info!("on_enemy_hit");
        for (entity, mut life, transform) in enemies.iter_mut() {
            if entity == event.entity {
                life.hit(event.damage);
                if life.is_dead() {
                    death_events.send(EnemyDeathEvent {
                        entity,
                        _pos: transform.translation,
                    });
                }
            }
        }
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

fn cast_rays_from_enemies(
    enemies: Query<(Entity, &Transform, &Children), (With<Enemy>, Without<Reload>)>,
    enemy_colliders: Query<&Parent, With<EnemyCollider>>,
    player: Query<(Entity, &Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut enemies_seeing_player: ResMut<EnemiesSeingPlayer>,
) {
    enemies_seeing_player.clear();
    let (player_entity, player_transform) = player.get_single().expect("Player");
    let player_center = Player::center(player_transform);
    for (enemy_entity, enemy_transform, children) in enemies.iter() {
        match children
            .iter()
            .find(|&child_entity| enemy_colliders.get(*child_entity).is_ok())
        {
            Some(enemy_collider_entity) => {
                let enemy_center = Enemy::center(enemy_transform);
                let ray_pos = enemy_center; // TODO:  + ray_dir.normalize() * (Enemy::weapon_offset());
                let ray_dir = player_center - enemy_center;

                let max_toi = ray_dir.length();
                let solid = false;
                let filter = QueryFilter::new()
                    .exclude_sensors()
                    .exclude_collider(*enemy_collider_entity);

                info!("enemy_fires() ray_pos: {ray_pos}, ray_dir: {ray_dir}, max_toi: {max_toi}");

                if let Some((entity, _toi)) =
                    rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                {
                    if entity == player_entity {
                        enemies_seeing_player.push(enemy_entity);
                    }
                }
            }
            None => warn!("EnemyCollider not found in Enemy children"),
        }
    }
}

///
fn enemy_fires(
    mut commands: Commands,
    enemies_seeing_player: Res<EnemiesSeingPlayer>,
    enemies: Query<(&Transform, &Weapon), (With<Enemy>, Without<Reload>)>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut ev_fire: EventWriter<FireEvent>,
) {
    let player_transform = player.get_single().expect("Player");
    for enemy_entity in enemies_seeing_player.iter() {
        if let Ok((enemy_transform, weapon)) = enemies.get(*enemy_entity) {
            let fire_origin = Enemy::center(enemy_transform);
            let fire_direction = Player::center(player_transform) - fire_origin;

            let event = weapon
                .fire()
                .from(FireEmitter::Enemy)
                .origin(fire_origin)
                .direction(Direction3d::new(fire_direction).unwrap())
                .event();
            ev_fire.send(event);

            // Weapon reload
            commands.entity(*enemy_entity).insert(Reload::new(weapon));
        }
    }
}
