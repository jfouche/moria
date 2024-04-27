use crate::{
    core::{IntoWorldPosition, Position},
    GameState,
};
use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use super::character::Life;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enemy;

impl Enemy {
    const RADIUS: f32 = 0.3;
}

#[derive(Resource)]
struct EnemyAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

/// Event to notify an enemy was hit
#[derive(Event)]
pub struct EnemyHitEvent {
    pub entity: Entity,
    pub damage: u16,
}

/// Event to notify an enemy is dead
#[derive(Event)]
pub struct EnemyDeathEvent {
    entity: Entity,
    pos: Vec3,
}

pub fn plugin(app: &mut App) {
    app.add_event::<EnemyHitEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::Game), spawn_enemy)
        .add_systems(Update, (on_hit, on_death));
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

fn spawn_enemy(mut commands: Commands, assets: Res<EnemyAssets>) {
    let pos = Position(2, 2);
    commands.spawn((
        Enemy,
        Name::new("Enemy"),
        Life::new(50),
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
                        pos: transform.translation,
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
