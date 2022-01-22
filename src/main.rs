use bevy::prelude::*;
use moria::maze::MazeBuilder;
use ui::{player_plugin::PlayerPlugin, setup, maze_plugin::MazePlugin};

mod ui;

fn main() {

    let maze = MazeBuilder::new(9, 5).create_maze();
    eprintln!("{}", maze.to_string());

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0. )))
        .insert_resource(WindowDescriptor {
            title: "Moria - Rust".to_string(),
            // width: 640.,
            // height: 480.,
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .insert_resource(maze)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .add_plugin(MazePlugin)
        .run();
}


