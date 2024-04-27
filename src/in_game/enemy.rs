use crate::{
    core::{IntoWorldPosition, Position},
    GameState,
};
use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

#[derive(Component)]
struct Enemy;

impl Enemy {
    const RADIUS: f32 = 0.3;
}

#[derive(Resource)]
struct EnemyAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::Game), spawn_enemy);
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
