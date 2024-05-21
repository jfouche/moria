use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CameraState {
    #[default]
    FollowPlayer,
    Free,
}

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    tag: PlayerCamera,
    name: Name,
    camera: Camera3dBundle,
    body: RigidBody,
    velocity: Velocity,
}

impl Default for PlayerCameraBundle {
    fn default() -> Self {
        PlayerCameraBundle {
            tag: PlayerCamera,
            name: Name::new("PlayerCamera"),
            camera: Camera3dBundle::default(),
            body: RigidBody::Dynamic, // TODO: remove use of rapier
            velocity: Velocity::zero(),
        }
    }
}
