use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct EndLevelAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl EndLevelAssets {
    pub const RADIUS: f32 = Wall::WIDTH / 4.0;
    pub const HEIGHT: f32 = Wall::HEIGHT / 3.0;

    pub fn load(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        EndLevelAssets {
            mesh: meshes.add(Cylinder::new(Self::RADIUS, Self::HEIGHT)),
            material: materials.add(Color::rgba(0.0, 0.0, 0.8, 0.5)),
        }
    }
}

#[derive(Component)]
pub struct EndLevel;

#[derive(Bundle)]
pub struct EndLevelBundle {
    tag: EndLevel,
    name: Name,
    pbr: PbrBundle,
    collider: Collider,
    sensor: Sensor,
    collider_events: ActiveEvents,
}

impl EndLevelBundle {
    pub fn new() -> Self {
        EndLevelBundle {
            tag: EndLevel,
            name: Name::new("EndLevel"),
            pbr: PbrBundle::default(),
            collider: Collider::cylinder(
                EndLevelAssets::HEIGHT / 2.0,
                EndLevelAssets::RADIUS / 2.0,
            ),
            sensor: Sensor,
            collider_events: ActiveEvents::COLLISION_EVENTS,
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.pbr.transform = Transform::from_translation(pos.to_world().translation());
        self
    }

    pub fn with_assets(mut self, assets: &EndLevelAssets) -> Self {
        self.pbr.mesh = assets.mesh.clone();
        self.pbr.material = assets.material.clone();
        self
    }
}
