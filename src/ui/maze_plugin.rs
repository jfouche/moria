use bevy::prelude::*;
use moria::maze::{CellBorders, Maze, Position, Room};

use crate::ui::{Materials, TIME_STEP};

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_maze", SystemStage::single(maze_spawn.system()))
        .add_system(maze_spawn.system());
    }
}

/// Get the index in the maze file
///  0:   ,  1: T,    2: R,    3: TR
///  4:  B,  5: TB,   6: RB,   7: TRB
///  8:  L,  9: TL,  10: RL,  11: TRL
/// 12: BL, 13: TBL, 14: RBL, 15: TRBL
fn img_index(borders: &CellBorders) -> usize {
    let mut index = 0;
    if !borders.top {
        index += 1;
    }
    if !borders.right {
        index += 2;
    }
    if !borders.bottom {
        index += 4;
    }
    if !borders.left {
        index += 8;
    }
    index
}

#[derive(Component)]
struct MazeComponent {}

impl Default for MazeComponent {
    fn default() -> Self {
        MazeComponent {}
    }
}

#[derive(Component)]
struct RoomComponent {
}

fn maze_spawn(mut commands: Commands, materials: Res<Materials>, maze: Res<Maze>) {
    for x in 0..maze.width() {
        for y in 00..maze.height() {
            if let Some(room) = maze.get_room(&Position::new(x, y)) {
                let x_pos = -200.0 + x as f32 * 64.; 
                let y_pos = -100.0 + y as f32 * 64.; 
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: materials.maze.clone(),
                        sprite: TextureAtlasSprite::new(img_index(room.borders())),
                        transform: Transform {
                            translation: Vec3::new(x_pos, y_pos, 10.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(RoomComponent { });
            }
        }
    }
}
