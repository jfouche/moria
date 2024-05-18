use crate::{components::*, config::*};
use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.00012;
const CAMERA_SPEED: f32 = 500.0;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

pub fn plugin(app: &mut App) {
    app.init_state::<CameraState>()
        .init_resource::<InputState>()
        .add_systems(PreStartup, spawn_camera)
        .add_systems(Startup, load_config)
        .add_systems(
            Update,
            (
                (change_camera, player_look).run_if(game_is_running),
                move_camera.run_if(in_free_state),
            ),
        )
        .add_systems(PostUpdate, follow_player.run_if(in_follow_player_state));
}

fn in_follow_player_state(
    in_game_state: Res<State<InGameState>>,
    camera_state: Res<State<CameraState>>,
) -> bool {
    *in_game_state == InGameState::Running && *camera_state == CameraState::FollowPlayer
}

fn in_free_state(
    in_game_state: Res<State<InGameState>>,
    camera_state: Res<State<CameraState>>,
) -> bool {
    *in_game_state == InGameState::Running && *camera_state == CameraState::Free
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3dBundle::default(),
        PlayerCamera,
        RigidBody::Dynamic,
        Velocity::zero(),
    ));
}

fn load_config(config: Res<CameraConfig>, mut exposure: Query<&mut Exposure>) {
    let params = PhysicalCameraParameters {
        aperture_f_stops: config.aperture_f_stops,
        shutter_speed_s: config.shutter_speed_s,
        sensitivity_iso: config.sensitivity_iso,
    };
    *exposure.single_mut() = Exposure::from_physical_camera(params);
}

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

fn follow_player(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    players: Query<&Transform, With<Player>>,
) {
    let player_transform = players.get_single().expect("Player");
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    cam_transform.translation = Player::camera_translation(player_transform);
}

fn player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut players: Query<&mut Transform, With<Player>>,
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let window = primary_window.get_single().expect("PrimaryWindow");
    let mut player_transform = players.get_single_mut().expect("Player");
    let mut camera_transform = cameras.get_single_mut().expect("PlayerCamera");
    for ev in state.reader_motion.read(&motion) {
        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
        match window.cursor.grab_mode {
            CursorGrabMode::None => (),
            _ => {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                pitch -= (MOUSE_SENSITIVITY * ev.delta.y * window_scale).to_radians();
                yaw -= (MOUSE_SENSITIVITY * ev.delta.x * window_scale).to_radians();
            }
        }

        pitch = pitch.clamp(-1.54, 1.54);

        // Rotate Camera
        // Order is important to prevent unintended roll
        camera_transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);

        // Rotate player
        let mut direction = *camera_transform.forward();
        direction.y = 0.0;
        player_transform.look_to(direction, Vec3::Y);
    }
}

fn move_camera(
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
