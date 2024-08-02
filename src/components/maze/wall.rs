use super::*;
use bevy::{math::vec2, prelude::*};
use bevy_rapier3d::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Copy, Clone, Debug)]
pub enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

impl Wall {
    pub const HEIGHT: f32 = 2.0;
    pub const WIDTH: f32 = 5.0;

    pub const COLLIDER_WIDTH: f32 = 0.02;

    pub fn mesh(&self) -> impl Into<Mesh> {
        let normal = match self {
            Wall::Top => Vec3::Z,
            Wall::Bottom => Vec3::NEG_Z,
            Wall::Left => Vec3::X,
            Wall::Right => Vec3::NEG_X,
        };

        Plane3d::new(normal, vec2(Wall::WIDTH, Wall::HEIGHT)).mesh()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WallPosition {
    pub wall: Wall,
    pub pos: Position,
}

#[derive(Bundle)]
pub struct WallBundle {
    name: Name,
    pbr: PbrBundle,
}

impl WallBundle {
    pub fn new(wall_pos: WallPosition) -> Self {
        WallBundle {
            name: Name::new(format!("Wall {wall_pos:?}")),
            pbr: PbrBundle {
                transform: Self::transform(wall_pos),
                ..default()
            },
        }
    }

    pub fn with_pbr(mut self, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        self.pbr.mesh = mesh;
        self.pbr.material = material;
        self
    }

    pub fn transform(wall_pos: WallPosition) -> Transform {
        let translation = wall_pos.pos.to_world().translation();
        const HH: f32 = Wall::HEIGHT / 2.;
        const HW: f32 = Wall::WIDTH / 2.;

        let translation = match wall_pos.wall {
            Wall::Top => translation + Vec3::new(0., HH, -HW),
            Wall::Bottom => translation + Vec3::new(0., HH, HW),
            Wall::Left => translation + Vec3::new(-HW, HH, 0.),
            Wall::Right => translation + Vec3::new(HW, HH, 0.),
        };
        let rotation = match wall_pos.wall {
            Wall::Top => Quat::IDENTITY,
            Wall::Bottom => Quat::IDENTITY,
            Wall::Left => Quat::from_rotation_x(FRAC_PI_2),
            Wall::Right => Quat::from_rotation_x(FRAC_PI_2),
        };
        Transform {
            translation,
            rotation,
            scale: Vec3::ONE,
        }
    }
}

#[derive(Bundle)]
pub struct WallColliderBundle {
    collider: Collider,
    transform: TransformBundle,
}

impl WallColliderBundle {
    pub fn new(wall_pos: WallPosition) -> Self {
        WallColliderBundle {
            collider: Self::collider(wall_pos),
            transform: Self::transform(wall_pos).into(),
        }
    }

    pub fn collider(wall_pos: WallPosition) -> Collider {
        const HRW: f32 = Wall::WIDTH / 2.0;
        let (hx, hz) = match wall_pos.wall {
            Wall::Top | Wall::Bottom => (HRW, Wall::COLLIDER_WIDTH),
            Wall::Left | Wall::Right => (Wall::COLLIDER_WIDTH, HRW),
        };
        Collider::cuboid(hx, Wall::HEIGHT / 2., hz)
    }

    pub fn transform(wall_pos: WallPosition) -> Transform {
        let (x, z) = match wall_pos.wall {
            Wall::Top => (0.0, -Wall::COLLIDER_WIDTH),
            Wall::Bottom => (0.0, Wall::COLLIDER_WIDTH),
            Wall::Left => (Wall::COLLIDER_WIDTH, 0.0),
            Wall::Right => (-Wall::COLLIDER_WIDTH, 0.0),
        };
        let rotation = match wall_pos.wall {
            Wall::Top => Quat::IDENTITY,
            Wall::Bottom => Quat::IDENTITY,
            Wall::Left => Quat::from_rotation_x(FRAC_PI_2),
            Wall::Right => Quat::from_rotation_x(FRAC_PI_2),
        };

        Transform::from_xyz(x, 0.0, z).with_rotation(rotation)
    }
}
