use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CameraState {
    #[default]
    FollowPlayer,
    PanOrbitCamera,
}
