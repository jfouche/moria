use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct PlayerAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl PlayerAssets {
    pub fn load(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        PlayerAssets {
            mesh: meshes.add(Cuboid::new(
                Player::BODY_RADIUS,
                Player::BODY_RADIUS,
                Player::HEIGHT,
            )),
            material: materials.add(Color::BEIGE),
        }
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    const HEIGHT: f32 = 0.7;
    const BODY_RADIUS: f32 = 0.3;
    pub const CAMERA_HEIGHT: f32 = Self::HEIGHT * 0.9;

    pub fn fire_origin(transform: &Transform) -> Vec3 {
        let direction = transform.forward();
        transform.translation
            + Vec3::new(0.0, Player::HEIGHT * 0.9, 0.0)
            + *direction * Player::BODY_RADIUS
    }

    pub fn center(transform: &Transform) -> Vec3 {
        // TODO: add Head
        transform.translation + Vec3::new(0.0, Self::HEIGHT / 2.0, 0.0)
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: Player,
    name: Name,
    life: Life,
    weapon: Weapon,
    pbr: PbrBundle,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    locked_axes: LockedAxes,
}

impl PlayerBundle {
    pub fn new(weapon: Weapon) -> Self {
        PlayerBundle {
            tag: Player,
            name: Name::new("Player"),
            life: Life::new(100),
            weapon,
            pbr: PbrBundle::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::cylinder(Player::HEIGHT / 2.0, Player::BODY_RADIUS / 2.0),
            locked_axes: LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Y
                | LockedAxes::ROTATION_LOCKED_Z,
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.pbr.transform = Transform::from_translation(pos.to_world().translation())
            .looking_to(Vec3::NEG_Z, Vec3::Y);
        self
    }

    pub fn with_assets(mut self, assets: &PlayerAssets) -> Self {
        self.pbr.mesh = assets.mesh.clone();
        self.pbr.material = assets.material.clone();
        self
    }
}

/// Event to notify the player was hit
#[derive(Event)]
pub struct PlayerHitEvent {
    pub damage: u16,
}

/// Event to notify the player is dead
#[derive(Event)]
pub struct PlayerDeathEvent;
