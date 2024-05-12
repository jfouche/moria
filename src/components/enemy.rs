use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct EnemyAssets(SceneWithCollidersAssets);

impl EnemyAssets {
    pub fn load(assets_server: &AssetServer) -> Self {
        let scene_handle = assets_server.load("SWAT.glb#Scene0");
        EnemyAssets(SceneWithCollidersAssets::load(scene_handle))
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enemy;

impl Enemy {
    pub const RADIUS: f32 = 0.3;
    pub const SCALE: Vec3 = Vec3::splat(0.8);
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    name: Name,
    life: Life,
    weapon: Weapon,
    scene: SceneBundle,
}

impl EnemyBundle {
    pub fn new(weapon: Weapon) -> Self {
        EnemyBundle {
            enemy: Enemy,
            name: Name::new("Enemy"),
            life: Life::new(50),
            weapon,
            scene: SceneBundle::default(),
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.scene.transform =
            Transform::from_translation(pos.to_world().translation()).with_scale(Enemy::SCALE);
        self
    }

    pub fn with_assets(mut self, assets: &EnemyAssets) -> Self {
        self.scene.scene = assets.scene();
        self
    }
}

#[derive(Component)]
pub struct EnemyCollider;

#[derive(Bundle)]
pub struct EnemyColliderBundle {
    tag: EnemyCollider,
    name: Name,
    collider: Collider,
    transform: TransformBundle,
    sensor: Sensor,
    collider_events: ActiveEvents,
}

impl EnemyColliderBundle {
    pub fn new(collider: Collider, transform: Transform) -> Self {
        EnemyColliderBundle {
            tag: EnemyCollider,
            name: Name::new("EnemyColliderBundle"),
            collider,
            transform: TransformBundle::from_transform(transform),
            sensor: Sensor,
            collider_events: ActiveEvents::COLLISION_EVENTS,
        }
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
