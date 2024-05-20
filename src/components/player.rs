use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerAssets(SceneWithCollidersAssets);

impl From<SceneWithCollidersAssets> for PlayerAssets {
    fn from(value: SceneWithCollidersAssets) -> Self {
        PlayerAssets(value)
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    const HEIGHT: f32 = 1.1;
    const BODY_RADIUS: f32 = 0.35;
    const SCALE: Vec3 = Vec3::splat(0.5);
    const CAMERA_HEIGHT: f32 = 0.95;

    pub fn camera_translation(player_transform: &Transform) -> Vec3 {
        // TODO: add player radius
        // Warning, adding player radius will modify the player transform while
        // moving the mouse, so the player might clip a wall
        player_transform.translation + Self::SCALE * (Vec3::new(0.0, Player::CAMERA_HEIGHT, 0.0))
    }

    pub fn fire_origin(player_transform: &Transform) -> Vec3 {
        Self::camera_translation(player_transform)
    }

    pub fn center(player_transform: &Transform) -> Vec3 {
        player_transform.translation + Vec3::new(0.0, Self::HEIGHT / 2.0, 0.0) * Self::SCALE
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: Player,
    name: Name,
    life: Life,
    weapon: Weapon,
    scene: SceneBundle,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    locked_axes: LockedAxes,
    collision_groups: CollisionGroups,
}

impl PlayerBundle {
    pub fn new(weapon: Weapon) -> Self {
        PlayerBundle {
            tag: Player,
            name: Name::new("Player"),
            life: Life::new(100),
            weapon,
            scene: SceneBundle::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::cuboid(
                Player::BODY_RADIUS / 2.0,
                Player::HEIGHT / 2.0,
                Player::BODY_RADIUS / 2.0,
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Y
                | LockedAxes::ROTATION_LOCKED_Z,
            collision_groups: CollisionGroups::new(COLLISION_GROUP_PLAYER, Group::all()),
        }
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.scene.transform = Transform::from_translation(pos.to_world().translation())
            .with_scale(Player::SCALE)
            .looking_to(Vec3::NEG_Z, Vec3::Y);
        self
    }

    pub fn with_assets(mut self, assets: &PlayerAssets) -> Self {
        self.scene.scene = assets.scene();
        self
    }
}

#[derive(Component)]
pub struct PlayerCollider;

#[derive(Bundle)]
pub struct PlayerColliderBundle {
    tag: PlayerCollider,
    name: Name,
    transform: TransformBundle,
    collider: Collider,
    collision_groups: CollisionGroups,
}

impl PlayerColliderBundle {
    pub fn new(collider: Collider, transform: Transform) -> Self {
        PlayerColliderBundle {
            tag: PlayerCollider,
            name: Name::new("PlayerCollider"),
            transform: TransformBundle::from_transform(transform),
            collider: Collider::cuboid(
                Player::BODY_RADIUS / 2.0,
                Player::HEIGHT / 2.0,
                Player::BODY_RADIUS / 2.0,
            ),
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
