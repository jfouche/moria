use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct PotionAssets {
    scene: Handle<Scene>,
}

impl PotionAssets {
    pub fn load(asset_server: &AssetServer) -> Self {
        PotionAssets {
            scene: asset_server.load("potion.glb#Scene0"),
        }
    }
}

#[derive(Component)]
pub enum Potion {
    Life(u16),
}

impl Potion {
    const HEIGHT: f32 = 0.17;
    const RADIUS: f32 = 0.13;
    const SCALE: f32 = 4.0;
}

#[derive(Bundle)]
pub struct PotionBundle {
    potion: Potion,
    name: Name,
    scene: SceneBundle,
    collider: Collider,
    sensor: Sensor,
    collider_events: ActiveEvents,
    // collision_tpes: ActiveCollisionTypes,
}

impl PotionBundle {
    pub fn new(potion: Potion) -> Self {
        PotionBundle {
            potion,
            name: Name::new("Potion"),
            scene: SceneBundle::default(),
            collider: Collider::cylinder(Potion::HEIGHT / 2.0, Potion::RADIUS / 2.0),
            sensor: Sensor,
            collider_events: ActiveEvents::COLLISION_EVENTS,
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.scene.transform = Transform::from_translation(pos.to_world().translation())
            .with_scale(Vec3::splat(Potion::SCALE));
        self
    }

    pub fn with_assets(mut self, assets: &PotionAssets) -> Self {
        self.scene.scene = assets.scene.clone();
        self
    }
}
