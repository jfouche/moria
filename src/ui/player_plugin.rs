use bevy::prelude::*;
use moria::maze::Position;

use crate::ui::{Materials, TIME_STEP};

use super::{PositionToScreen, WinSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system());
    }
}

#[derive(Component)]
struct PlayerComponent {
    pos: Position,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        PlayerComponent {
            pos: Position::new(0, 0),
        }
    }
}

#[derive(Component)]
struct PlayerSpeedComponent(f32);

impl Default for PlayerSpeedComponent {
    fn default() -> Self {
        PlayerSpeedComponent(300.)
    }
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    let p2s = PositionToScreen::new(&win_size);
    commands
        .spawn_bundle(SpriteBundle {
            texture: materials.player.clone(),
            transform: Transform {
                translation: p2s.to_screen(&Position::new(0, 0), 20.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerComponent::default())
        .insert(PlayerSpeedComponent::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeedComponent, &mut Transform, With<PlayerComponent>)>,
) {
    if let Ok((speed, mut transform, _)) = query.get_single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            (-1.0, 0.)
        } else if keyboard_input.pressed(KeyCode::Right) {
            (1.0, 0.)
        } else if keyboard_input.pressed(KeyCode::Up) {
            (0., 1.)
        } else if keyboard_input.pressed(KeyCode::Down) {
            (0., -1.)
        } else {
            (0., 0.)
        };
        transform.translation.x += dir.0 * speed.0 * TIME_STEP;
        transform.translation.y += dir.1 * speed.0 * TIME_STEP;
    }
}
