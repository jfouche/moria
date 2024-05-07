use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

impl Player {
    pub const HEIGHT: f32 = 0.6;
    pub const WIDTH: f32 = 0.1;
}

/// Event to notify the player was hit
#[derive(Event)]
pub struct PlayerHitEvent {
    pub damage: u16,
}

/// Event to notify the player is dead
#[derive(Event)]
pub struct PlayerDeathEvent;
