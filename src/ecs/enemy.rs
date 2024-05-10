use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct EnemyAssets {
    scene: Handle<Scene>,
}

impl EnemyAssets {
    pub fn load(assets_server: &AssetServer) -> Self {
        EnemyAssets {
            scene: assets_server.load("SWAT.glb#Scene0"),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enemy;

impl Enemy {
    pub const RADIUS: f32 = 0.3;
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    name: Name,
    life: Life,
    weapon: Weapon,
    scene: SceneBundle,
    body: RigidBody,
    collider: Collider,
}

impl EnemyBundle {
    pub fn new(weapon: Weapon) -> Self {
        EnemyBundle {
            enemy: Enemy,
            name: Name::new("Enemy"),
            life: Life::new(50),
            weapon,
            scene: SceneBundle::default(),
            body: RigidBody::Dynamic,
            collider: Collider::ball(Enemy::RADIUS),
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.scene.transform = Transform::from_translation(pos.to_world().translation());
        self
    }

    pub fn with_assets(mut self, assets: &Res<EnemyAssets>) -> Self {
        self.scene.scene = assets.scene.clone();
        self
    }
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
    pub entity: Entity,
    pub _pos: Vec3,
}
