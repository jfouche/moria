use crate::Wall;
use bevy::prelude::*;
use std::{fmt, ops::Deref};

/// .0 : x
///
/// .1 : y
#[derive(Clone, Copy, PartialEq, Eq, Debug, Reflect, Component)]
pub struct Position(pub u32, pub u32);

impl Position {
    /// Get the square of the distance
    pub fn sqr_distance(&self, other: &Position) -> u32 {
        let dx = self.0 as i32 - other.0 as i32;
        let dy = self.1 as i32 - other.1 as i32;
        (dx * dx + dy * dy) as u32
    }

    // pub fn x(&self) -> u32 {
    //     self.0
    // }

    // pub fn y(&self) -> u32 {
    //     self.1
    // }

    pub fn fx(&self) -> f32 {
        self.0 as f32
    }

    pub fn fy(&self) -> f32 {
        self.1 as f32
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub struct WorldPosition(Position);

impl Deref for WorldPosition {
    type Target = Position;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WorldPosition {
    pub fn new(x: u32, y: u32) -> Self {
        WorldPosition(Position(x, y))
    }

    pub fn rect(&self) -> Rect {
        let x = self.fx() * Wall::WIDTH;
        let z = (self.fy() + 1.0) * -Wall::WIDTH;
        let min = Vec2::new(x, z);
        let max = min + Vec2::new(Wall::WIDTH, Wall::WIDTH);
        Rect { min, max }
    }

    pub fn translation_with_y(&self, y: f32) -> Vec3 {
        let center = self.rect().center();
        Vec3 {
            x: center.x,
            y,
            z: center.y,
        }
    }
    pub fn translation(&self) -> Vec3 {
        self.translation_with_y(0.0)
    }
}

impl fmt::Debug for WorldPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorldPosition{}", self.0)
    }
}

impl From<Vec3> for WorldPosition {
    fn from(world_pos: Vec3) -> Self {
        let x = (world_pos.x / Wall::WIDTH) as u32;
        let y = (-world_pos.z / Wall::WIDTH) as u32;
        WorldPosition::new(x, y)
    }
}

pub trait IntoWorldPosition {
    fn to_world(&self) -> WorldPosition;
}

impl IntoWorldPosition for Position {
    fn to_world(&self) -> WorldPosition {
        WorldPosition(*self)
    }
}

#[allow(clippy::all)]
#[cfg(test)]
mod test {
    use super::*;

    const RW: f32 = Wall::WIDTH;
    const HRW: f32 = Wall::WIDTH / 2.0;

    #[test]
    fn position_to_world() {
        let pos = Position(0, 0);
        let translation = pos.to_world().translation();
        assert_eq!(Vec3::new(HRW, 0., -HRW), translation, "Position{pos}");

        let pos = Position(0, 1);
        let translation = pos.to_world().translation();
        assert_eq!(Vec3::new(HRW, 0., -RW - HRW), translation, "Position{pos}");

        let pos = Position(1, 0);
        let translation = pos.to_world().translation();
        assert_eq!(Vec3::new(RW + HRW, 0., -HRW), translation, "Position{pos}");
    }

    #[test]
    fn world_to_position() {
        let world_translation = Vec3::new(0.0, 0.0, 0.0);
        let world_pos: WorldPosition = world_translation.into();
        assert_eq!(
            Position(0, 0),
            *world_pos,
            "Translation: {world_translation}"
        );

        let world_translation = Vec3::new(RW - 0.1, 0.0, 0.0);
        let world_pos: WorldPosition = world_translation.into();
        assert_eq!(
            Position(0, 0),
            *world_pos,
            "Translation: {world_translation}"
        );

        let world_translation = Vec3::new(RW, 0.0, 0.0);
        let world_pos: WorldPosition = world_translation.into();
        assert_eq!(
            Position(1, 0),
            *world_pos,
            "Translation: {world_translation}"
        );

        let world_translation = Vec3::new(0.0, 0.0, -RW);
        let world_pos: WorldPosition = world_translation.into();
        assert_eq!(
            Position(0, 1),
            *world_pos,
            "Translation: {world_translation}"
        );
    }
}
