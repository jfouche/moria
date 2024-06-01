use super::*;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Resource)]
pub struct EnemyAssets {
    scene: Handle<Scene>,
}

impl From<&AssetServer> for EnemyAssets {
    fn from(asset_server: &AssetServer) -> Self {
        EnemyAssets {
            scene: asset_server.load("slime.glb#Scene0"),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enemy;

impl Enemy {
    const SIZE: Vec3 = Vec3::new(2.0, 2.0, 1.1);
    const SCALE: Vec3 = Vec3::splat(0.3);

    pub fn center_offset() -> Vec3 {
        Vec3::new(0.0, Self::SIZE.y / 2.0, 0.0) * Self::SCALE
    }

    pub fn weapon_offset() -> Vec3 {
        Self::center_offset()
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    name: Name,
    life: Life,
    weapon: Weapon,
    scene: SceneBundle,
    collider: AsyncSceneCollider,
    collision_layers: CollisionLayers,
}

impl EnemyBundle {
    pub fn new(weapon: Weapon) -> Self {
        EnemyBundle {
            enemy: Enemy,
            name: Name::new("Enemy"),
            life: Life::new(50),
            weapon,
            scene: SceneBundle::default(),
            collider: AsyncSceneCollider::new(None)
                .with_shape_for_name("collider_slime", ComputedCollider::ConvexHull),
            collision_layers: CollisionLayers::new(InGameLayers::Enemy, LayerMask::ALL),
        }
    }

    pub fn at(mut self, pos: RoomPosition) -> Self {
        self.scene.transform =
            Transform::from_translation(pos.to_world().translation()).with_scale(Enemy::SCALE);
        self
    }

    pub fn with_assets(mut self, assets: &EnemyAssets) -> Self {
        self.scene.scene = assets.scene.clone();
        self
    }
}

// TODO: Remove this
#[derive(Component)]
pub struct EnemyCollider;

#[derive(Bundle)]
pub struct EnemyColliderBundle {
    tag: EnemyCollider,
    name: Name,
    collider: AsyncSceneCollider,
    transform: TransformBundle,
    collision_layers: CollisionLayers,
}

impl EnemyColliderBundle {
    pub fn new(collider: Collider, transform: Transform) -> Self {
        EnemyColliderBundle {
            tag: EnemyCollider,
            name: Name::new("EnemyCollider"),
            transform: transform.into(),
            collider: AsyncSceneCollider::default(),
            collision_layers: CollisionLayers::new(InGameLayers::Enemy, LayerMask::ALL),
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
