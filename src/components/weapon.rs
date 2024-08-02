use super::*;
use bevy::{audio::Volume, color::palettes::css::ORANGE, prelude::*};
use bevy_rapier3d::prelude::*;
use std::{collections::HashMap, f32::consts::FRAC_PI_2};

/// BulletAssets
struct BulletAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl BulletAssets {
    pub fn mesh(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }

    pub fn material(&self) -> Handle<StandardMaterial> {
        self.material.clone()
    }
}

/// WeaponAssets
#[derive(Resource)]
pub struct WeaponAssets {
    bullet_sound: Handle<AudioSource>,
    bullet_assets: HashMap<WeaponType, BulletAssets>,
}

impl WeaponAssets {
    pub fn load(
        asset_server: &AssetServer,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        weapons: &Weapons,
    ) -> Self {
        let bullet_sound = asset_server.load("audio/556-Single-Isolated.ogg");
        let mut bullet_assets = HashMap::new();
        for (weapon_type, weapon) in weapons.0.iter() {
            let mesh = meshes.add(Cylinder::new(weapon.bullet.radius, weapon.bullet.length));
            let material = materials.add(Color::Srgba(ORANGE));
            let assets = BulletAssets { mesh, material };
            bullet_assets.insert(*weapon_type, assets);
        }

        WeaponAssets {
            bullet_sound,
            bullet_assets,
        }
    }

    fn bullet_assets(&self, weapon_type: &WeaponType) -> &BulletAssets {
        self.bullet_assets
            .get(weapon_type)
            .expect("Unconfigured weapon type")
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
    pub weapon_type: WeaponType,
    pub from: FireEmitter,
    pub origin: Vec3,
    pub direction: Dir3,
    pub bonus: f32,
}

// impl From<&FireEvent> for Velocity {
//     fn from(event: &FireEvent) -> Self {
//         Velocity::linear(event.direction * event.weapon.bullet.speed)
//     }
// }

impl FireEvent {
    fn collision_groups(&self) -> CollisionGroups {
        let collision_filters = match self.from {
            FireEmitter::Player => Group::ALL & !COLLISION_GROUP_PLAYER,
            FireEmitter::Enemy => Group::ALL & !COLLISION_GROUP_ENEMY,
        };
        CollisionGroups::new(COLLISION_GROUP_BULLET, collision_filters)
    }
}

// impl From<&FireEvent> for Bullet {
//     fn from(event: &FireEvent) -> Self {
//         Bullet {
//             damage: event.weapon.damage,
//         }
//     }
// }

/// Fire emitter
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum FireEmitter {
    Player,
    Enemy,
}

#[derive(Clone, Debug)]
pub struct Weapon {
    pub damage: u16,
    /// in secs
    pub reload_delay: f32,
    pub bullet: BulletConfig,
}

impl From<&BulletConfig> for LifeTime {
    fn from(bullet: &BulletConfig) -> Self {
        LifeTime::new(bullet.distance / bullet.speed)
    }
}

impl From<&WeaponConfig> for Weapon {
    fn from(config: &WeaponConfig) -> Self {
        Weapon {
            damage: config.damage,
            reload_delay: config.reload_delay,
            bullet: config.bullet_config.clone(),
        }
    }
}

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq)]
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

    pub fn get(&self, weapon_type: WeaponType) -> &Weapon {
        self.0.get(&weapon_type).expect("Existing weapon")
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
    velocity: Velocity,
    collider: Collider,
    ccd: Ccd,
    collider_events: ActiveEvents,
    collision_tpes: ActiveCollisionTypes,
    collision_groups: CollisionGroups,
}

impl BulletBundle {
    pub fn new(fire_ev: &FireEvent, weapons: &Weapons, assets: &WeaponAssets) -> Self {
        let mut transform = Transform::from_translation(fire_ev.origin);
        transform.look_to(*fire_ev.direction, Vec3::Y);
        transform.rotate_local_x(FRAC_PI_2);

        let weapon_type = fire_ev.weapon_type;
        let weapon = weapons.get(weapon_type);
        let bullet_assets = assets.bullet_assets(&weapon_type);
        let bullet_velocity = fire_ev.direction * weapon.bullet.speed;

        BulletBundle {
            bullet: Bullet {
                damage: weapon.damage,
            },
            name: Name::new("BULLET"),
            lifetime: (&weapon.bullet).into(),
            emiter: fire_ev.from,
            pbr: PbrBundle {
                transform,
                mesh: bullet_assets.mesh(),
                material: bullet_assets.material(),
                ..default()
            },
            body: RigidBody::KinematicVelocityBased,
            velocity: Velocity::linear(bullet_velocity),
            ccd: Ccd::enabled(),
            collider: Collider::cylinder(Bullet::LENGTH / 2.0, Bullet::RADIUS / 2.0),
            collider_events: ActiveEvents::COLLISION_EVENTS,
            collision_tpes: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            collision_groups: fire_ev.collision_groups(),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
#[component(storage = "SparseSet")]
pub struct Reload(Timer);

impl Reload {
    pub fn new(weapon: &Weapon, bonus: f32) -> Self {
        let delay = weapon.reload_delay / bonus;
        Reload(Timer::from_seconds(delay, TimerMode::Once))
    }

    pub fn finished(&mut self, time: &Time) -> bool {
        self.0.tick(time.delta()).finished()
    }
}
