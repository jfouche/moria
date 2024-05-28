use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct EndLevelAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl EndLevelAssets {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mesh = meshes.add(Cylinder::new(EndLevel::RADIUS, EndLevel::HEIGHT));
        let material = materials.add(Color::rgba(0.0, 0.0, 0.8, 0.4));
        EndLevelAssets { mesh, material }
    }
}

#[derive(Component)]
pub struct EndLevel;

impl EndLevel {
    const RADIUS: f32 = Wall::WIDTH / 4.0;
    const HEIGHT: f32 = Wall::HEIGHT / 3.0;
}

///
/// EndLevel Bundle
///
#[derive(Bundle)]
pub struct EndLevelBundle {
    tag: EndLevel,
    name: Name,
    pbr: PbrBundle,
}

impl EndLevelBundle {
    pub fn new() -> Self {
        EndLevelBundle {
            tag: EndLevel,
            name: Name::new("EndLevel"),
            pbr: PbrBundle::default(),
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.pbr.transform =
            Transform::from_translation(pos.to_world().translation_with_y(EndLevel::HEIGHT / 2.0));
        self
    }

    pub fn with_assets(mut self, assets: &EndLevelAssets) -> Self {
        self.pbr.mesh = assets.mesh.clone();
        self.pbr.material = assets.material.clone();
        self
    }
}

#[derive(Component)]
pub struct EndLevelCollider;

///
/// EndLevelColliderBundle
///
#[derive(Bundle)]
pub struct EndLevelColliderBundle {
    tag: EndLevelCollider,
    name: Name,
    transform: TransformBundle,
    collider: Collider,
    sensor: Sensor,
    collider_events: ActiveEvents,
}

impl Default for EndLevelColliderBundle {
    fn default() -> Self {
        EndLevelColliderBundle {
            tag: EndLevelCollider,
            name: Name::new("EndLevelCollider"),
            transform: TransformBundle::default(),
            collider: Collider::cylinder(EndLevel::HEIGHT / 2.0, EndLevel::RADIUS / 8.0),
            sensor: Sensor,
            collider_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
