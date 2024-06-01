use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// #[derive(Resource, Deref, DerefMut)]
// pub struct PlayerAssets(SceneWithCollidersAssets);

// impl From<SceneWithCollidersAssets> for PlayerAssets {
//     fn from(value: SceneWithCollidersAssets) -> Self {
//         PlayerAssets(value)
//     }
// }

///
/// Stores all [Player] assets
///
#[derive(Resource)]
pub struct PlayerAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl PlayerAssets {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mesh = meshes.add(Cuboid::new(Player::SIZE.x, Player::SIZE.y, Player::SIZE.z));
        let material = materials.add(Color::PINK);
        PlayerAssets { mesh, material }
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    const SIZE: Vec3 = Vec3 {
        x: 0.6,
        y: 1.0,
        z: 0.3,
    };
    const SCALE: Vec3 = Vec3::splat(0.6);
    const CAMERA_HEIGHT: f32 = 0.95 * Self::SIZE.y;

    fn offset() -> Vec3 {
        Player::SCALE
            * Vec3 {
                x: 0.0,
                y: -Player::SIZE.y / 2.0,
                z: 0.0,
            }
    }

    pub fn camera_offset() -> Vec3 {
        // TODO: add player radius
        // Warning, adding player radius will modify the player transform while
        // moving the mouse, so the player might clip a wall
        Self::offset() + Player::SCALE * Vec3::new(0.0, Player::CAMERA_HEIGHT, 0.0)
    }

    pub fn fire_origin_offset() -> Vec3 {
        Self::camera_offset()
    }

    pub fn center_offset() -> Vec3 {
        Self::offset()
            + Self::SCALE
                * Vec3 {
                    x: 0.0,
                    y: Self::SIZE.y / 2.0,
                    z: 0.0,
                }
    }
    pub fn tranform(pos: Position) -> Transform {
        Transform::from_translation(pos.to_world().translation_with_y(Player::SIZE.y / 2.0))
            .with_scale(Player::SCALE)
            .looking_to(Vec3::NEG_Z, Vec3::Y)
    }
}

///
/// Player bundle. It should have a [PlayerColliderBundle] child
///
#[derive(Bundle)]
pub struct PlayerBundle {
    tag: Player,
    name: Name,
    life: Life,
    weapon_type: WeaponType,
    // scene: SceneBundle,
    pbr: PbrBundle,
    body: RigidBody,
    velocity: Velocity,
    locked_axes: LockedAxes,
}

impl PlayerBundle {
    pub fn new(weapon_type: WeaponType) -> Self {
        PlayerBundle {
            tag: Player,
            name: Name::new("Player"),
            life: Life::new(100),
            weapon_type,
            // scene: SceneBundle::default(),
            pbr: PbrBundle::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            locked_axes: /*LockedAxes::TRANSLATION_LOCKED |*/ LockedAxes::ROTATION_LOCKED,
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.pbr.transform = Player::tranform(pos);
        self
    }

    pub fn with_assets(mut self, assets: &PlayerAssets) -> Self {
        //     self.scene.scene = assets.scene();
        self.pbr.mesh = assets.mesh.clone();
        self.pbr.material = assets.material.clone();
        self
    }
}

#[derive(Component)]
pub struct PlayerCollider;

///
/// Components for the player collider
///
#[derive(Bundle)]
pub struct PlayerColliderBundle {
    tag: PlayerCollider,
    name: Name,
    transform: TransformBundle,
    collider: Collider,
    locked_axes: LockedAxes,
    collision_groups: CollisionGroups,
}

impl Default for PlayerColliderBundle {
    fn default() -> Self {
        PlayerColliderBundle {
            tag: PlayerCollider,
            name: Name::new("PlayerCollider"),
            transform: TransformBundle::default(),
            collider: Collider::cuboid(
                Player::SIZE.x / 2.0,
                Player::SIZE.y / 2.0,
                Player::SIZE.z / 2.0,
            ),
            locked_axes: /*LockedAxes::TRANSLATION_LOCKED |*/ LockedAxes::ROTATION_LOCKED,
            collision_groups: CollisionGroups::new(COLLISION_GROUP_PLAYER, Group::all()),
        }
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
