use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct PotionAssets(SceneWithCollidersAssets);

impl From<SceneWithCollidersAssets> for PotionAssets {
    fn from(value: SceneWithCollidersAssets) -> Self {
        PotionAssets(value)
    }
}

#[derive(Component)]
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
}

impl PotionBundle {
    pub fn new(potion: Potion) -> Self {
        PotionBundle {
            potion,
            name: Name::new("Potion"),
            scene: SceneBundle::default(),
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.scene.transform =
            Transform::from_translation(pos.to_world().translation()).with_scale(Potion::SCALE);
        self
    }

    pub fn with_assets(mut self, assets: &PotionAssets) -> Self {
        self.scene.scene = assets.scene();
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
    collider_events: ActiveEvents,
}

impl PotionColliderBundle {
    pub fn new(collider: Collider, transform: Transform) -> Self {
        PotionColliderBundle {
            tag: PotionCollider,
            collider,
            transform: TransformBundle::from_transform(transform),
            sensor: Sensor,
            collider_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
