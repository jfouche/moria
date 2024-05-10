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
            mesh: meshes.add(Capsule3d::new(Player::WIDTH / 2.0, Player::HEIGHT / 2.0)),
            material: materials.add(Color::BLACK),
        }
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub const HEIGHT: f32 = 0.6;
    pub const WIDTH: f32 = 0.1;
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
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
            player: Player,
            name: Name::new("Player"),
            life: Life::new(100),
            weapon,
            pbr: PbrBundle::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::round_cuboid(
                Player::WIDTH / 2.0,
                Player::WIDTH / 2.0,
                Player::HEIGHT / 2.0,
                0.05,
            ),
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
