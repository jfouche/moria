use crate::ecs::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
struct EnemyAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub fn plugin(app: &mut App) {
    app.add_event::<EnemyHitEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::InGame), spawn_enemy)
        .add_systems(
            Update,
            (on_hit, on_death, enemy_fires).run_if(game_is_running),
        );
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = EnemyAssets {
        mesh: meshes.add(Sphere::new(Enemy::RADIUS)),
        material: materials.add(Color::RED),
    };
    commands.insert_resource(assets);
}

fn spawn_enemy(mut commands: Commands, assets: Res<EnemyAssets>, weapons: Res<Weapons>) {
    let pos = Position(2, 2);
    commands.spawn((
        Enemy,
        Name::new("Enemy"),
        Life::new(50),
        weapons.get(WeaponType::Shotgun),
        PbrBundle {
            mesh: assets.mesh.clone(),
            material: assets.material.clone(),
            transform: Transform::from_translation(pos.to_world().translation()),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(Enemy::RADIUS),
    ));
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

///
fn enemy_fires(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform, &Weapon), (With<Enemy>, Without<Reload>)>,
    player: Query<(Entity, &Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut ev_fire: EventWriter<FireEvent>,
) {
    let (player_entity, player_transform) = player.get_single().expect("Player");
    for (enemy_entity, enemy_transform, weapon) in enemies.iter() {
        let dy = Vec3::new(0.0, 0.1, 0.0);
        let enemy_pos = enemy_transform.translation;
        let ray_dir = player_transform.translation - enemy_pos;
        let ray_pos = enemy_pos + ray_dir.normalize() * (Enemy::RADIUS + 0.2) + dy;
        let max_toi = ray_dir.length();
        let solid = true;
        let filter = QueryFilter::new();
        if let Some((entity, toi)) =
            rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        {
            if entity == player_entity {
                info!("Enemy sees Player, toi: {toi}");
                // Enemy fires

                let event = weapon
                    .fire()
                    .from(FireEmitter::Enemy)
                    .origin(ray_pos)
                    .direction(Direction3d::new(ray_dir).unwrap())
                    .event();
                ev_fire.send(event);

                // Weapon reload
                commands.entity(enemy_entity).insert(Reload::new(weapon));
            }
        }
    }
}
