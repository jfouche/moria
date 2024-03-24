use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

const SPEED: f32 = 10.0;

#[derive(Component)]
pub struct Player;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 10.,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_systems(Startup, player_init)
            .add_systems(Update, (player_move, player_look, cursor_grab));
    }
}

fn player_init(
    mut commands: Commands,
    mut transform: Query<(Entity, &mut Transform), With<Camera3d>>,
) {
    let (entity, mut transform) = transform.get_single_mut().expect("Can't player camera");
    commands.entity(entity).insert(Player);
    *transform = Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::Z, Vec3::Y);
}

// https://github.com/sburris0/bevy_flycam/blob/master/src/lib.rs
fn player_move(
    mut transform: Query<&mut Transform, With<Player>>,
    settings: Res<MovementSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = transform.get_single_mut().expect("Can't player camera");
    let forward = *transform.forward();
    let forward = Vec3::new(forward.x, 0.0, forward.z);
    let right = *transform.right();
    let right = Vec3::new(right.x, 0.0, right.z);
    let mut velocity = Vec3::ZERO;
    for key in keys.get_pressed() {
        match *key {
            KeyCode::ArrowUp => velocity += forward,
            KeyCode::ArrowDown => velocity -= forward,
            KeyCode::ArrowLeft => velocity -= right,
            KeyCode::ArrowRight => velocity += right,
            _ => {}
        }
    }
    velocity = velocity.normalize_or_zero();
    transform.translation += velocity * time.delta_seconds() * settings.speed;
}

fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(KeyCode::KeyG) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}
