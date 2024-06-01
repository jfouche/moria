use super::*;
use bevy::{audio::Volume, prelude::*};
use bevy_xpbd_3d::prelude::*;
use std::{collections::HashMap, f32::consts::FRAC_PI_2};

/// WeaponAssets
#[derive(Resource)]
pub struct WeaponAssets {
    bullet_mesh: Handle<Mesh>,
    bullet_material: Handle<StandardMaterial>,
    bullet_sound: Handle<AudioSource>,
}

impl WeaponAssets {
    pub fn load(
        asset_server: &AssetServer,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let bullet_mesh = meshes.add(Cylinder::new(Bullet::RADIUS, Bullet::LENGTH));
        let bullet_material = materials.add(Color::ORANGE);
        let bullet_sound = asset_server.load("audio/556-Single-Isolated.ogg");
        WeaponAssets {
            bullet_mesh,
            bullet_material,
            bullet_sound,
        }
    }

    pub fn bullet_mesh(&self) -> Handle<Mesh> {
        self.bullet_mesh.clone()
    }

    pub fn bullet_material(&self) -> Handle<StandardMaterial> {
        self.bullet_material.clone()
    }

    pub fn bullet_sound(&self) -> Handle<AudioSource> {
        self.bullet_sound.clone()
    }
}

/// BulletSoundBundle
#[derive(Bundle)]
pub struct BulletSoundBundle {
    name: Name,
    audio: AudioBundle,
}

impl BulletSoundBundle {
    pub fn new(assets: &WeaponAssets, sound_volume: &SoundVolume) -> Self {
        let volume = Volume::new(sound_volume.db());
        BulletSoundBundle {
            name: Name::new("Bullet sound"),
            audio: AudioBundle {
                source: assets.bullet_sound(),
                settings: PlaybackSettings::DESPAWN.with_volume(volume),
            },
        }
    }
}

/// FireEvent
#[derive(Event)]
pub struct FireEvent {
    pub weapon: Weapon,
    pub from: FireEmitter,
    pub origin: Vec3,
    pub direction: Direction3d,
}

impl From<&FireEvent> for LinearVelocity {
    fn from(event: &FireEvent) -> Self {
        LinearVelocity(event.direction * event.weapon.bullet_speed)
    }
}

impl From<&FireEvent> for LifeTime {
    fn from(event: &FireEvent) -> Self {
        (&event.weapon).into()
    }
}

impl From<&FireEvent> for CollisionLayers {
    fn from(event: &FireEvent) -> Self {
        let collision_filter = match event.from {
            FireEmitter::Player => InGameLayers::Enemy,

            FireEmitter::Enemy => InGameLayers::Player,
        };
        CollisionLayers::new(
            InGameLayers::Bullet,
            [InGameLayers::Ground, collision_filter],
        )
    }
}

impl From<&FireEvent> for Bullet {
    fn from(event: &FireEvent) -> Self {
        Bullet {
            damage: event.weapon.damage,
        }
    }
}

/// FireEventBuilder
pub struct FireEventBuilder<F, D> {
    weapon: Weapon,
    from: F,
    direction: D,
}

pub struct NoFrom;
pub struct WithFrom(Vec3, FireEmitter);

pub struct NoDirection;
pub struct WithDirection(Direction3d);

impl FireEventBuilder<NoFrom, NoDirection> {
    fn new(weapon: &Weapon) -> Self {
        FireEventBuilder {
            weapon: weapon.clone(),
            from: NoFrom,
            direction: NoDirection,
        }
    }
}

impl<D> FireEventBuilder<NoFrom, D> {
    pub fn from(self, origin: Vec3, from: FireEmitter) -> FireEventBuilder<WithFrom, D> {
        FireEventBuilder {
            weapon: self.weapon,
            from: WithFrom(origin, from),
            direction: self.direction,
        }
    }
}

impl<F> FireEventBuilder<F, NoDirection> {
    pub fn to(self, direction: Direction3d) -> FireEventBuilder<F, WithDirection> {
        FireEventBuilder {
            weapon: self.weapon,
            from: self.from,
            direction: WithDirection(direction),
        }
    }
}

impl FireEventBuilder<WithFrom, WithDirection> {
    pub fn event(self) -> FireEvent {
        FireEvent {
            weapon: self.weapon,
            from: self.from.1,
            origin: self.from.0,
            direction: self.direction.0,
        }
    }
}

/// Fire emiter
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum FireEmitter {
    Player,
    Enemy,
}

#[derive(Component, Clone, Debug)]
pub struct Weapon {
    pub damage: u16,
    /// in secs
    pub reload_delay: f32,
    pub bullet_speed: f32,
    pub bullet_distance: f32,
}

impl Weapon {
    pub fn fire(&self) -> FireEventBuilder<NoFrom, NoDirection> {
        FireEventBuilder::<NoFrom, NoDirection>::new(self)
    }
}

impl From<&Weapon> for LifeTime {
    fn from(weapon: &Weapon) -> Self {
        LifeTime::new(weapon.bullet_distance / weapon.bullet_speed)
    }
}

impl From<&WeaponConfig> for Weapon {
    fn from(config: &WeaponConfig) -> Self {
        Weapon {
            damage: config.damage,
            bullet_speed: config.bullet_speed,
            bullet_distance: config.bullet_distance,
            reload_delay: config.reload_delay,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum WeaponType {
    Gun,
    Shotgun,
    EnemyGun,
}

impl TryFrom<&str> for WeaponType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Gun" => Ok(Self::Gun),
            "Shotgun" => Ok(Self::Shotgun),
            "Enemy" => Ok(Self::EnemyGun),
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

#[derive(Component, Clone, Copy)]
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
    velocity: LinearVelocity,
    collider: Collider,
    // collider_events: ActiveEvents,
    // collision_tpes: ActiveCollisionTypes,
    collision_layers: CollisionLayers,
}

impl BulletBundle {
    pub fn new(fire_ev: &FireEvent) -> Self {
        let mut transform = Transform::from_translation(fire_ev.origin);
        transform.look_to(*fire_ev.direction, Vec3::Y);
        transform.rotate_local_x(FRAC_PI_2);

        BulletBundle {
            bullet: fire_ev.into(),
            name: Name::new("BULLET"),
            lifetime: fire_ev.into(),
            emiter: fire_ev.from,
            pbr: PbrBundle {
                transform,
                ..default()
            },
            body: RigidBody::Kinematic,
            velocity: fire_ev.into(),
            collider: Collider::cylinder(Bullet::LENGTH / 2.0, Bullet::RADIUS / 2.0),
            // collider_events: ActiveEvents::COLLISION_EVENTS,
            // collision_tpes: ActiveCollisionTypes::default()
            //     | ActiveCollisionTypes::KINEMATIC_STATIC,
            collision_layers: fire_ev.into(),
        }
    }

    pub fn with_assets(mut self, assets: &WeaponAssets) -> Self {
        self.pbr.mesh = assets.bullet_mesh();
        self.pbr.material = assets.bullet_material();
        self
    }
}

#[derive(Component, Deref, DerefMut)]
#[component(storage = "SparseSet")]
pub struct Reload(Timer);

impl Reload {
    pub fn from(weapon: &Weapon) -> Self {
        Reload(Timer::from_seconds(weapon.reload_delay, TimerMode::Once))
    }

    pub fn finished(&mut self, time: &Time) -> bool {
        self.0.tick(time.delta()).finished()
    }
}
