use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct EnemyAssets(SceneWithCollidersAssets);

impl From<SceneWithCollidersAssets> for EnemyAssets {
    fn from(value: SceneWithCollidersAssets) -> Self {
        EnemyAssets(value)
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enemy;

impl Enemy {
    const SIZE: Vec3 = Vec3::new(2.0, 2.0, 1.1);
    const SCALE: Vec3 = Vec3::splat(0.3);

    pub fn center(transform: &Transform) -> Vec3 {
        transform.translation + Vec3::new(0.0, Self::SIZE.y / 2.0, 0.0) * Self::SCALE
    }

    pub fn weapon_offset(transform: &Transform) -> Vec3 {
        Self::center(transform)
    }
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
    collision_groups: CollisionGroups,
}

impl EnemyColliderBundle {
    pub fn new(collider: Collider, transform: Transform) -> Self {
        EnemyColliderBundle {
            tag: EnemyCollider,
            name: Name::new("EnemyCollider"),
            transform: TransformBundle::from_transform(transform),
            collider,
            sensor: Sensor,
            collider_events: ActiveEvents::COLLISION_EVENTS,
            collision_groups: CollisionGroups::new(COLLISION_GROUP_ENEMY, Group::all()),
        }
    }
}

/// Event to notify an enemy was hit
#[derive(Event)]
pub struct EnemyHitEvent {
    pub entity: Entity,
    pub pos: Vec3,
    pub damage: u16,
}

/// Event to notify an enemy is dead
#[derive(Event)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub _pos: Vec3,
}

#[derive(Resource)]
pub struct ImpactAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}
