use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Potion;

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
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    collider_events: ActiveEvents,
    // collision_tpes: ActiveCollisionTypes,
}

impl PotionBundle {
    pub fn new() -> Self {
        PotionBundle {
            potion: Potion,
            name: Name::new("Potion"),
            scene: SceneBundle::default(),
            body: RigidBody::Fixed,
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

    pub fn with_scene(mut self, scene: Handle<Scene>) -> Self {
        self.scene.scene = scene;
        self
    }
}
