use crate::{
    components::*,
    config::*,
    math::{HorizontalVec, IntoHorizontalVec, SignedAngle},
    schedule::InGameSet,
};
use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

const MOUSE_SENSITIVITY: f32 = 0.00012;
const CAMERA_SPEED: f32 = 500.0;

/// Keeps track of mouse motion events
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// Current View rotation
#[derive(Resource, Default)]
struct ViewRotation {
    rotation: Quat,
}

impl ViewRotation {
    fn init(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    /// `yaw`: Left / Right
    ///
    /// `pitch`: Up / Down
    fn yaw_and_pitch(&self) -> (f32, f32) {
        let (yaw, pitch, _) = self.rotation.to_euler(EulerRot::YXZ);
        (yaw, pitch)
    }

    fn rotate(&mut self, pitch: f32, yaw: f32) {
        let pitch = pitch.clamp(-1.54, 1.54);
        self.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

pub fn plugin(app: &mut App) {
    app.init_state::<CameraState>()
        .init_resource::<InputState>()
        .init_resource::<ViewRotation>()
        .add_systems(PreStartup, spawn_camera)
        .add_systems(Startup, load_config)
        .add_systems(
            Update,
            (
                change_camera,
                handle_player_look,
                handle_camera_moves.run_if(in_state(CameraState::Free)),
            )
                .in_set(InGameSet::UserInput),
        )
        .add_systems(
            Update,
            (rotate_player, camera_follows_player, update_camera_pitch)
                .chain()
                .run_if(in_state(CameraState::FollowPlayer))
                .in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            (update_camera_yaw, update_camera_pitch)
                .chain()
                .run_if(in_state(CameraState::Free))
                .in_set(InGameSet::EntityUpdate),
        );
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(PlayerCameraBundle::default());
}

fn load_config(config: Res<CameraConfig>, mut exposure: Query<&mut Exposure>) {
    let params = PhysicalCameraParameters {
        aperture_f_stops: config.aperture_f_stops,
        shutter_speed_s: config.shutter_speed_s,
        sensitivity_iso: config.sensitivity_iso,
    };
    *exposure.single_mut() = Exposure::from_physical_camera(params);
}

/// Change the camera view, using digits
fn change_camera(
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera_next_state: ResMut<NextState<CameraState>>,
) {
    if key_input.just_pressed(KeyCode::Digit1) {
        camera_next_state.set(CameraState::FollowPlayer);
    } else if key_input.just_pressed(KeyCode::Digit2) {
        camera_next_state.set(CameraState::Free);
    }
}

fn rotate_player(
    // TODO: use Velocity
    // mut players: Query<(&Transform, &mut Velocity), With<Player>>,
    mut players: Query<&mut Transform, With<Player>>,
    view_rotation: Res<ViewRotation>,
) {
    // let (player_transform, mut player_velocity) = players.get_single_mut().expect("Player");
    let mut player_transform = players.get_single_mut().expect("Player");
    let (yaw, _pitch) = view_rotation.yaw_and_pitch();
    let target_dir = HorizontalVec::from_angle(yaw);
    let delta_angle = PI - player_transform.signed_angle_with(target_dir);
    player_transform.rotate_y(-delta_angle);
    // let angvel = if delta_angle < f32::EPSILON {
    //     Vec3::ZERO
    // } else if delta_angle > 0.0 {
    //     Vec3::NEG_Y
    // } else {
    //     Vec3::Y
    // };
    // player_velocity.angvel = angvel;
}

fn camera_follows_player(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    players: Query<&Transform, With<Player>>,
) {
    let player_transform = players.get_single().expect("Player");
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    let player_dir = player_transform.forward().horizontal();
    cam_transform.translation = Player::camera_translation(player_transform);
    cam_transform.look_to(player_dir.into(), Vec3::Y);
}

fn update_camera_yaw(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    view_rotation: Res<ViewRotation>,
) {
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    let (yaw, _pitch) = view_rotation.yaw_and_pitch();
    cam_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
}

fn update_camera_pitch(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    view_rotation: Res<ViewRotation>,
) {
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    let (_yaw, pitch) = view_rotation.yaw_and_pitch();
    cam_transform.rotation *= Quat::from_axis_angle(Vec3::X, pitch);
}

/// handle the player look using the mouse movements
fn handle_player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    mut view_rotation: ResMut<ViewRotation>,
    motion: Res<Events<MouseMotion>>,
    cameras: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let window = primary_window.get_single().expect("PrimaryWindow");
    if window.cursor.grab_mode == CursorGrabMode::None {
        return;
    }
    let window_scale = window.height().min(window.width());

    let camera_transform = cameras.get_single().expect("PlayerCamera");
    view_rotation.init(camera_transform.rotation);

    for ev in state.reader_motion.read(&motion) {
        let (mut yaw, mut pitch) = view_rotation.yaw_and_pitch();
        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
        pitch -= (MOUSE_SENSITIVITY * ev.delta.y * window_scale).to_radians();
        yaw -= (MOUSE_SENSITIVITY * ev.delta.x * window_scale).to_radians();
        view_rotation.rotate(pitch, yaw);
    }
}

/// Handle camera movements, using WASD (or ZQSD in FR)
fn handle_camera_moves(
    mut cameras: Query<(&Transform, &mut Velocity), With<PlayerCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (transform, mut velocity) = cameras.get_single_mut().expect("PlayerCamera");
    let forward = *transform.forward();
    let right = *transform.right();
    let mut delta = Vec3::ZERO;
    for key in keys.get_pressed() {
        match *key {
            KeyCode::ArrowUp | KeyCode::KeyW => delta += forward,
            KeyCode::ArrowDown | KeyCode::KeyS => delta -= forward,
            KeyCode::ArrowLeft | KeyCode::KeyA => delta -= right,
            KeyCode::ArrowRight | KeyCode::KeyD => delta += right,
            _ => {}
        }
    }
    delta = delta.normalize_or_zero();
    velocity.linvel = delta * time.delta_seconds() * CAMERA_SPEED;
}
