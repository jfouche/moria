use crate::{assets_loader::assets_loading, components::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const PLAYER_SPEED: f32 = 200.0;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_event::<PlayerHitEvent>()
        .add_event::<PlayerDeathEvent>()
        .add_systems(
            Startup,
            load_scene_assets::<PlayerAssets>("player.glb#Scene0"),
        )
        .add_systems(
            Update,
            load_scene_colliders::<PlayerAssets>.run_if(assets_loading),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_player)
        .add_systems(Update, (player_fires, on_hit).run_if(game_is_running))
        .add_systems(
            Update,
            player_move.run_if(game_is_running.and_then(in_state(CameraState::FollowPlayer))),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<Player>);
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>, weapons: Res<Weapons>) {
    info!("spawn_player()");
    let pos = Position(0, 0);
    let weapon = weapons.get(WeaponType::Shotgun);
    commands
        .spawn(PlayerBundle::new(weapon).at(pos).with_assets(&assets))
        .with_children(|parent| {
            for (collider, transform) in assets.colliders() {
                parent.spawn(PlayerColliderBundle::new(collider.clone(), *transform));
            }
        });
}

// https://github.com/sburris0/bevy_flycam/blob/master/src/lib.rs
fn player_move(
    mut players: Query<(&Transform, &mut Velocity), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (transform, mut velocity) = players.get_single_mut().expect("Player");
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
    velocity.linvel = delta * time.delta_seconds() * PLAYER_SPEED;
}

fn player_fires(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player: Query<(Entity, &Transform, &Weapon), (With<Player>, Without<Reload>)>,
    cameras: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    mut ev_fire: EventWriter<FireEvent>,
) {
    // The query doesn't return if the weapon is reloading (eg. if it contains the [Reload] component)
    if let Ok((entity, transform, weapon)) = player.get_single() {
        let cam_transform = cameras.get_single().expect("PlayerCamera");
        if keys.pressed(KeyCode::Space) {
            let direction = cam_transform.forward();
            let origin = Player::fire_origin(transform);
            let event = weapon
                .fire()
                .from(origin, FireEmitter::Player)
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
