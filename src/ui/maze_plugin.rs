use bevy::prelude::*;
use moria::maze::{CellBorders, Maze, Position};

use crate::ui::{Materials};

use super::{WinSize, PositionConverter};

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_maze", SystemStage::single(maze_spawn.system()))
        ;
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

#[derive(Component, Default)]
struct MazeComponent {}

#[derive(Component)]
struct RoomComponent {
}

fn maze_spawn(mut commands: Commands, materials: Res<Materials>, maze: Res<Maze>, win_size: Res<WinSize>) {
    info!("maze_spawn(...)");
    let pos_converter = PositionConverter::new(&win_size);
    for x in 0..maze.width() {
        for y in 00..maze.height() {
            let pos = Position::new(x, y);
            if let Some(room) = maze.get_room(&pos) {
                let screen_pos = pos_converter.to_screen(&pos, 10.);
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: materials.maze.clone(),
                        sprite: TextureAtlasSprite::new(img_index(room.borders())),
                        transform: Transform {
                            translation: screen_pos,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(RoomComponent { });
            }
        }
    }
}
