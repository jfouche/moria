use crate::ecs::*;
use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 200.0,
        }
    }
}

#[derive(Resource)]
struct PlayerAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_event::<PlayerHitEvent>()
        .add_event::<PlayerDeathEvent>()
        .init_resource::<InputState>()
        .init_resource::<MovementSettings>()
        .add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::InGame), spawn_player)
        .add_systems(
            Update,
            (player_move, player_look, player_fires, on_hit).run_if(game_is_running),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<Player>);
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Capsule3d::new(Player::WIDTH / 2.0, Player::HEIGHT / 2.0));
    let material = materials.add(Color::BLACK);
    let player_assets = PlayerAssets { mesh, material };
    commands.insert_resource(player_assets);
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>, weapons: Res<Weapons>) {
    let pos = Position(0, 0);
    let weapon = weapons.get(WeaponType::Shotgun);
    let mesh = assets.mesh.clone();
    let material = assets.material.clone();
    commands.spawn(PlayerBundle::new(weapon).at(pos).with_pbr(mesh, material));
}

// https://github.com/sburris0/bevy_flycam/blob/master/src/lib.rs
fn player_move(
    mut player: Query<(&Transform, &mut Velocity), With<Player>>,
    settings: Res<MovementSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (transform, mut velocity) = player.get_single_mut().expect("Can't retrieve Player");
    let mut forward = *transform.forward();
    forward.y = 0.0;
    let mut right = *transform.right();
    right.y = 0.0;
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
    velocity.linvel = delta * time.delta_seconds() * settings.speed;
}

fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query_player: Query<&mut Transform, With<Player>>,
) {
    let window = primary_window
        .get_single()
        .expect("Can't retrieve primary window");
    let mut transform = query_player
        .get_single_mut()
        .expect("Player should be present");
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

fn player_fires(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player: Query<(Entity, &Transform, &Weapon), (With<Player>, Without<Reload>)>,
    mut ev_fire: EventWriter<FireEvent>,
) {
    // The query doesn't return if the weapon is reloading (eg. if it contains the [Reload] component)
    if let Ok((entity, transform, weapon)) = player.get_single() {
        if keys.pressed(KeyCode::Space) {
            let direction = transform.forward();
            let origin = transform.translation
                + Vec3::new(0.0, Player::HEIGHT * 0.8, 0.0)
                + *direction * Player::WIDTH;
            let event = weapon
                .fire()
                .from(FireEmitter::Player)
                .origin(origin)
                .direction(direction)
                .event();
            ev_fire.send(event);

            // Reload
            commands.entity(entity).insert(Reload::new(weapon));
        }
    }
}

///
/// Player is hit
///
fn on_hit(
    mut hit_events: EventReader<PlayerHitEvent>,
    mut player: Query<(&mut Life, &Transform), With<Player>>,
    mut death_events: EventWriter<PlayerDeathEvent>,
) {
    let (mut life, _transform) = player.get_single_mut().expect("Player");
    for event in hit_events.read() {
        info!("on_hit");
        life.hit(event.damage);
        if life.is_dead() {
            death_events.send(PlayerDeathEvent);
        }
    }
}
