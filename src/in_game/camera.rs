use crate::{
    components::*,
    math::{HorizontalVec, IntoHorizontalVec, SignedAngle},
    schedule::InGameSet,
};
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts::PI;

const MOUSE_SENSITIVITY: f32 = 0.00012;
const CAMERA_SPEED: f32 = 8.0;

pub fn plugin(app: &mut App) {
    app.init_state::<CameraState>()
        .init_resource::<InputState>()
        .init_resource::<CameraView>()
        .add_systems(Startup, (spawn_camera, apply_deferred, load_config).chain())
        .add_systems(
            Update,
            camera_follows_player.run_if(in_state(InGameState::LoadLevel)),
        )
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
        sensor_height: 0.01866,
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

fn rotate_player(mut players: Query<&mut Transform, With<Player>>, camera_view: Res<CameraView>) {
    let mut player_transform = players.get_single_mut().expect("Player");
    let yaw = camera_view.yaw();
    let target_dir = HorizontalVec::from_angle(yaw);
    let delta_angle = PI - player_transform.signed_angle_with(target_dir);
    player_transform.rotate_y(-delta_angle);
}

fn camera_follows_player(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    players: Query<&Transform, With<Player>>,
) {
    let player_transform = players.get_single().expect("Player");
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    let player_dir = player_transform.forward().horizontal();
    cam_transform.translation = player_transform.translation + Player::camera_offset();
    cam_transform.look_to(player_dir, Vec3::Y);
}

fn update_camera_yaw(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    camera_view: Res<CameraView>,
) {
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    let yaw = camera_view.yaw();
    cam_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
}

fn update_camera_pitch(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    camera_view: Res<CameraView>,
) {
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    let pitch = camera_view.pitch();
    cam_transform.rotation *= Quat::from_axis_angle(Vec3::X, pitch);
}

/// handle the player look using the mouse movements
fn handle_player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    mut camera_view: ResMut<CameraView>,
    motion: Res<Events<MouseMotion>>,
    cameras: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    if let Ok(window) = primary_window.get_single() {
        if window.cursor.grab_mode == CursorGrabMode::None {
            return;
        }
        let window_scale = window.height().min(window.width());

        let camera_transform = cameras.get_single().expect("PlayerCamera");
        camera_view.init_rotation(camera_transform.rotation);

        for ev in state.reader_motion.read(&motion) {
            let (mut yaw, mut pitch) = (camera_view.yaw(), camera_view.pitch());
            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
            pitch -= (MOUSE_SENSITIVITY * ev.delta.y * window_scale).to_radians();
            yaw -= (MOUSE_SENSITIVITY * ev.delta.x * window_scale).to_radians();
            camera_view.rotate(yaw, pitch);
        }
    }
}

/// Handle camera movements, using WASD (or ZQSD in FR)
fn handle_camera_moves(
    mut cameras: Query<&mut Transform, With<PlayerCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = cameras.get_single_mut().expect("PlayerCamera");
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
    transform.translation += delta * time.delta_seconds() * CAMERA_SPEED;
}
