use super::*;
use crate::config::WeaponConfig;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{collections::HashMap, f32::consts::FRAC_PI_2};

#[derive(Event)]
pub struct FireEvent {
    pub weapon: Weapon,
    pub from: FireEmitter,
    pub origin: Vec3,
    pub direction: Direction3d,
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
    pub damage: u16,
    pub bullet_speed: f32,
    /// in secs
    pub reload_delay: f32,
}

impl Weapon {
    pub fn fire(&self) -> FireEventBuilder<NoFrom, NoOrigin, NoDirection> {
        FireEventBuilder::<NoFrom, NoOrigin, NoDirection>::new(self)
    }
}

impl From<&WeaponConfig> for Weapon {
    fn from(config: &WeaponConfig) -> Self {
        Weapon {
            damage: config.damage,
            bullet_speed: config.bullet_speed,
            reload_delay: config.reload_delay,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum WeaponType {
    Gun,
    Shotgun,
}

impl TryFrom<&str> for WeaponType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Gun" => Ok(Self::Gun),
            "Shotgun" => Ok(Self::Shotgun),
            _ => Err("Unknown weapon type"),
        }
    }
}

#[derive(Resource)]
pub struct Weapons(HashMap<WeaponType, Weapon>);

impl Weapons {
    pub fn new() -> Self {
        Weapons(HashMap::new())
    }

    pub fn insert(&mut self, weapon_type: WeaponType, weapon: Weapon) {
        self.0.insert(weapon_type, weapon);
    }

    pub fn get(&self, weapon_type: WeaponType) -> Weapon {
        self.0.get(&weapon_type).expect("Existing weapon").clone()
    }
}

#[derive(Component)]
pub struct Bullet {
    pub damage: u16,
}

impl Bullet {
    pub const RADIUS: f32 = 0.03;
    pub const LENGTH: f32 = 0.1;
}

#[derive(Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    name: Name,
    lifetime: LifeTime,
    emiter: FireEmitter,
    pbr: PbrBundle,
    body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    collider_events: ActiveEvents,
}

impl BulletBundle {
    pub fn new(fire_ev: &FireEvent) -> Self {
        let mut transform = Transform::from_translation(fire_ev.origin);
        transform.look_to(*fire_ev.direction, Vec3::Y);
        transform.rotate_local_x(FRAC_PI_2);

        BulletBundle {
            bullet: Bullet {
                damage: fire_ev.weapon.damage,
            },
            name: Name::new("BULLET"),
            lifetime: LifeTime::new(1.0),
            emiter: fire_ev.from,
            pbr: PbrBundle {
                transform,
                ..default()
            },
            body: RigidBody::KinematicVelocityBased,
            collider: Collider::cylinder(Bullet::LENGTH / 2.0, Bullet::RADIUS / 2.0),
            velocity: Velocity::linear(*fire_ev.direction * fire_ev.weapon.bullet_speed),
            collider_events: ActiveEvents::COLLISION_EVENTS,
        }
    }

    pub fn with_pbr(mut self, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        self.pbr.mesh = mesh;
        self.pbr.material = material;
        self
    }
}

#[derive(Component, Deref, DerefMut)]
#[component(storage = "SparseSet")]
pub struct Reload(Timer);

impl Reload {
    pub fn new(weapon: &Weapon) -> Self {
        Reload(Timer::from_seconds(weapon.reload_delay, TimerMode::Once))
    }
}
