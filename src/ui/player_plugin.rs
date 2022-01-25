use bevy::prelude::*;
use moria::maze::{Position, Maze, Direction};

use crate::ui::{Materials, TIME_STEP};

use super::{PositionConverter, WinSize};

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
    info!("player_spawn(...)");
    let pos_converter = PositionConverter::new(&win_size);
    commands
        .spawn_bundle(SpriteBundle {
            texture: materials.player.clone(),
            transform: Transform {
                translation: pos_converter.to_screen(&Position::new(0, 0), 20.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerComponent::default())
        .insert(PlayerSpeedComponent::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    maze: Res<Maze>,
    mut query: Query<(&PlayerSpeedComponent, &mut Transform, With<PlayerComponent>)>,
) {
    let pos_converter = PositionConverter::new(&win_size);
    if let Ok((speed, mut transform, _)) = query.get_single_mut() {
        let screen_pos = Vec3::new(transform.translation.x, transform.translation.y, 0.);
        let pos = pos_converter.to_position(&screen_pos);
        info!("pos: {}", pos);
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            maze.get_next_room(&pos, Direction::RIGHT);
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
