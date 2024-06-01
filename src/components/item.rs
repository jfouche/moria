use super::*;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Resource)]
pub struct PotionAssets {
    scene: Handle<Scene>,
}

impl From<&AssetServer> for PotionAssets {
    fn from(asset_server: &AssetServer) -> Self {
        PotionAssets {
            scene: asset_server.load("potion.glb#Scene0"),
        }
    }
}

pub enum Item {
    Potion(Potion),
}

#[derive(Component, Clone)]
pub enum Potion {
    Life(u16),
}

impl Potion {
    const SCALE: Vec3 = Vec3::splat(4.0);
}

#[derive(Bundle)]
pub struct PotionBundle {
    potion: Potion,
    name: Name,
    scene: SceneBundle,
    collider: AsyncSceneCollider,
    collision_layers: CollisionLayers,
}

impl PotionBundle {
    pub fn new(potion: Potion) -> Self {
        PotionBundle {
            potion,
            name: Name::new("Potion"),
            scene: SceneBundle::default(),
            collider: AsyncSceneCollider::new(None)
                .with_shape_for_name("collider_potion", ComputedCollider::ConvexHull),
            collision_layers: CollisionLayers::new(InGameLayers::Item, [InGameLayers::Player]),
        }
    }

    pub fn at(mut self, pos: RoomPosition) -> Self {
        self.scene.transform =
            Transform::from_translation(pos.to_world().translation()).with_scale(Potion::SCALE);
        self
    }

    pub fn with_assets(mut self, assets: &PotionAssets) -> Self {
        self.scene.scene = assets.scene.clone();
        self
    }
}

#[derive(Component)]
pub struct PotionCollider;

#[derive(Bundle)]
pub struct PotionColliderBundle {
    tag: PotionCollider,
    collider: Collider,
    transform: TransformBundle,
    sensor: Sensor,
    // collider_events: ActiveEvents,
}

impl PotionColliderBundle {
    pub fn new(collider: Collider, transform: Transform) -> Self {
        PotionColliderBundle {
            tag: PotionCollider,
            collider,
            transform: transform.into(),
            sensor: Sensor,
            // collider_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
