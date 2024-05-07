use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Enemy;

impl Enemy {
    pub const RADIUS: f32 = 0.3;
}

/// Event to notify an enemy was hit
#[derive(Event)]
pub struct EnemyHitEvent {
    pub entity: Entity,
    pub damage: u16,
}

/// Event to notify an enemy is dead
#[derive(Event)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub _pos: Vec3,
}
