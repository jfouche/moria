mod ui;

use bevy::{prelude::*};
use moria::maze::MazeBuilder;
use crate::ui::init_app;

fn main() {
    let maze = MazeBuilder::new(24, 13).create_maze();
    eprintln!("{}", maze.to_string());
    
    let mut app = App::new();
    init_app(&mut app, maze);
    app.run();
}

