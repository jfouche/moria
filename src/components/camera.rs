use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*};

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
}

impl Default for PlayerCameraBundle {
    fn default() -> Self {
        PlayerCameraBundle {
            tag: PlayerCamera,
            name: Name::new("PlayerCamera"),
            camera: Camera3dBundle::default(),
        }
    }
}

/// Keeps track of mouse motion events
#[derive(Resource, Default)]
pub struct InputState {
    pub reader_motion: ManualEventReader<MouseMotion>,
}

/// Current camera view rotation and mouse motion events
#[derive(Resource, Default)]
pub struct CameraView {
    rotation: Quat,
    yaw: f32,
    pitch: f32,
}

impl CameraView {
    pub fn init_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.update_yaw_and_pitch();
    }

    fn update_yaw_and_pitch(&mut self) {
        (self.yaw, self.pitch, _) = self.rotation.to_euler(EulerRot::YXZ);
    }

    /// `yaw`: Left / Right rotation
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    /// `pitch`: Up / Down rotation
    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        let pitch = pitch.clamp(-1.54, 1.54);
        self.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        self.update_yaw_and_pitch();
    }
}
