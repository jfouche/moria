use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{RigidBody, Velocity},
    geometry::{ActiveEvents, Collider},
};
use std::f32::consts::FRAC_PI_2;

#[derive(Event)]
pub struct FireEvent {
    weapon: Weapon,
    from: FireEmitter,
    origin: Vec3,
    direction: Direction3d,
}

pub struct FireEventBuilder<F, O, D> {
    weapon: Weapon,
    from: F,
    origin: O,
    direction: D,
}

pub struct NoFrom;
pub struct WithFrom(FireEmitter);

pub struct NoOrigin;
pub struct WithOrigin(Vec3);

pub struct NoDirection;
pub struct WithDirection(Direction3d);

impl FireEventBuilder<NoFrom, NoOrigin, NoDirection> {
    fn new(weapon: &Weapon) -> Self {
        FireEventBuilder {
            weapon: weapon.clone(),
            from: NoFrom,
            origin: NoOrigin,
            direction: NoDirection,
        }
    }
}

impl<O, D> FireEventBuilder<NoFrom, O, D> {
    pub fn from(self, from: FireEmitter) -> FireEventBuilder<WithFrom, O, D> {
        FireEventBuilder {
            weapon: self.weapon,
            from: WithFrom(from),
            origin: self.origin,
            direction: self.direction,
        }
    }
}

impl<F, D> FireEventBuilder<F, NoOrigin, D> {
    pub fn origin(self, origin: Vec3) -> FireEventBuilder<F, WithOrigin, D> {
        FireEventBuilder {
            weapon: self.weapon,
            from: self.from,
            origin: WithOrigin(origin),
            direction: self.direction,
        }
    }
}

impl<F, O> FireEventBuilder<F, O, NoDirection> {
    pub fn direction(self, direction: Direction3d) -> FireEventBuilder<F, O, WithDirection> {
        FireEventBuilder {
            weapon: self.weapon,
            from: self.from,
            origin: self.origin,
            direction: WithDirection(direction),
        }
    }
}

impl FireEventBuilder<WithFrom, WithOrigin, WithDirection> {
    pub fn event(self) -> FireEvent {
        FireEvent {
            weapon: self.weapon,
            from: self.from.0,
            origin: self.origin.0,
            direction: self.direction.0,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum FireEmitter {
    Player,
    Enemy,
}

#[derive(Component, Clone)]
pub struct Weapon {
    damage: u16,
    bullet_speed: f32,
    /// in secs
    reload_time: f32,
    reloading: bool,
}

impl Weapon {
    pub const GUN: Weapon = Weapon {
        damage: 10,
        bullet_speed: 20.0,
        reload_time: 0.4,
        reloading: false,
    };

    pub const SHOTGUN: Weapon = Weapon {
        damage: 35,
        bullet_speed: 30.0,
        reload_time: 0.9,
        reloading: false,
    };

    pub fn try_fire(&self) -> Option<FireEventBuilder<NoFrom, NoOrigin, NoDirection>> {
        if self.reloading {
            None
        } else {
            Some(FireEventBuilder::<NoFrom, NoOrigin, NoDirection>::new(self))
        }
    }
}

#[derive(Resource)]
struct WeaponAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    sound: Handle<AudioSource>,
}

#[derive(Component)]
pub struct Bullet {
    pub damage: u16,
}

#[derive(Component, Deref, DerefMut)]
pub struct Reload(Timer);

impl Reload {
    pub fn new(weapon: &Weapon) -> Self {
        Reload(Timer::from_seconds(weapon.reload_time, TimerMode::Once))
    }
}

const BULLET_RADIUS: f32 = 0.03;
const BULLET_LENGTH: f32 = 0.1;

pub fn plugin(app: &mut App) {
    app.add_event::<FireEvent>()
        .add_systems(Startup, load_assets)
        .add_systems(
            Update,
            (spawn_bullet, weapon_reloaded).run_if(in_state(GameState::Game)),
        );
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cylinder::new(BULLET_RADIUS, BULLET_LENGTH));
    let material = materials.add(Color::ORANGE);
    let sound = asset_server.load("audio/556-Single-Isolated.ogg");
    let assets = WeaponAssets {
        mesh,
        material,
        sound,
    };
    commands.insert_resource(assets);
}

fn spawn_bullet(
    mut commands: Commands,
    mut events: EventReader<FireEvent>,
    assets: Res<WeaponAssets>,
) {
    for fire_ev in events.read() {
        let mut transform = Transform::from_translation(fire_ev.origin);
        transform.look_to(*fire_ev.direction, Vec3::Y);
        transform.rotate_local_x(FRAC_PI_2);
        commands.spawn((
            Bullet {
                damage: fire_ev.weapon.damage,
            },
            Name::new("BULLET"),
            fire_ev.from,
            PbrBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cylinder(BULLET_LENGTH / 2.0, BULLET_RADIUS / 2.0),
            Velocity::linear(*fire_ev.direction * fire_ev.weapon.bullet_speed),
            ActiveEvents::COLLISION_EVENTS,
            // ColliderMassProperties::Density(0.0005), // TODO: don't hard code
        ));

        // spawn the audio in a different entity to be sure it doesn't stop
        // when the bullet is despawn to early
        commands.spawn(AudioBundle {
            source: assets.sound.clone(),
            settings: PlaybackSettings::ONCE,
        });
    }
}

fn weapon_reloaded(
    mut commands: Commands,
    time: Res<Time>,
    mut reloads: Query<(Entity, &mut Reload)>,
) {
    for (entity, mut reload) in reloads.iter_mut() {
        if reload.tick(time.delta()).finished() {
            commands.entity(entity).remove::<Reload>();
        }
    }
}
