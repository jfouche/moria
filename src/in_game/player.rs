use crate::{
    components::*,
    math::IntoHorizontalVec,
    schedule::{InGameLoadingSet, InGameSet},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const PLAYER_SPEED: f32 = 200.0;

///
/// Plugin
///
pub fn plugin(app: &mut App) {
    app.add_event::<PlayerHitEvent>()
        .add_event::<PlayerDeathEvent>()
        .add_systems(Startup, load_assets)
        // .add_systems(
        //     Startup,
        //     load_scene_assets::<PlayerAssets>("player.glb#Scene0"),
        // )
        // .add_systems(
        //     Update,
        //     load_scene_colliders::<PlayerAssets>.run_if(assets_loading),
        // )
        .add_systems(
            OnEnter(GameState::InGame),
            spawn_player.in_set(InGameLoadingSet::SpawnLevelEntities),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<Player>)
        .add_systems(
            Update,
            (player_fires, on_hit).in_set(InGameSet::EntityUpdate),
        )
        .add_systems(
            Update,
            player_move
                .run_if(in_state(CameraState::FollowPlayer))
                .in_set(InGameSet::UserInput),
        );
}

fn load_assets(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = PlayerAssets::new(meshes, materials);
    commands.insert_resource(assets);
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    weapons: Res<Weapons>,
    level: Res<Level>,
) {
    info!("spawn_player()");
    let weapon = weapons.get(WeaponType::Shotgun);
    commands
        .spawn(
            PlayerBundle::new(weapon)
                .at(level.start_position())
                .with_assets(&assets),
        )
        .with_children(|parent| {
            parent.spawn(PlayerColliderBundle::default());
        });
}

// https://github.com/sburris0/bevy_flycam/blob/master/src/lib.rs
fn player_move(
    mut players: Query<(&Transform, &mut Velocity), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (transform, mut velocity) = players.get_single_mut().expect("Player");
    let forward: Vec3 = transform.forward().horizontal().into();
    let right: Vec3 = transform.right().horizontal().into();
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
    if keys.pressed(KeyCode::Space) {
        // The query doesn't return if the weapon is reloading (eg. if it contains the [Reload] component)
        if let Ok((player_entity, player_transform, weapon)) = player.get_single() {
            // Use the camera to manage vertical view
            let cam_transform = cameras.get_single().expect("PlayerCamera");
            let direction = cam_transform.forward();
            let origin = player_transform.translation + Player::fire_origin_offset();
            let event = weapon
                .fire()
                .from(origin, FireEmitter::Player)
                .to(direction)
                .event();
            ev_fire.send(event);

            // Reload
            commands.entity(player_entity).insert(Reload::from(weapon));
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
