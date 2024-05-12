use crate::{components::*, config::*};
use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_panorbit_camera::*;

const MOUSE_SENSITIVITY: f32 = 0.00012;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

pub fn plugin(app: &mut App) {
    app.init_state::<CameraState>()
        .init_resource::<InputState>()
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(PreStartup, spawn_camera)
        .add_systems(Startup, load_config)
        .add_systems(
            Update,
            (
                toggle_camera_controls_system.run_if(game_is_running),
                player_look.run_if(in_follow_player_state),
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

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle::default(),
        PlayerCamera,
        PanOrbitCamera::default(),
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

fn toggle_camera_controls_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
    camera_state: Res<State<CameraState>>,
    mut camera_next_state: ResMut<NextState<CameraState>>,
) {
    if key_input.just_pressed(KeyCode::KeyT) {
        let mut pan_orbit = pan_orbit_query.get_single_mut().expect("PanOrbitCamera");
        match *camera_state.get() {
            CameraState::FollowPlayer => {
                camera_next_state.set(CameraState::PanOrbitCamera);
                pan_orbit.enabled = true;
            }
            CameraState::PanOrbitCamera => {
                camera_next_state.set(CameraState::FollowPlayer);
                pan_orbit.enabled = false;
            }
        }
    }
}

fn follow_player(
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    players: Query<&Transform, With<Player>>,
) {
    let player_transform = players.get_single().expect("Player");
    let mut cam_transform = cameras.get_single_mut().expect("PlayerCamera");
    cam_transform.translation =
        player_transform.translation + Vec3::new(0.0, Player::CAMERA_HEIGHT, 0.0);
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
