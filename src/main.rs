mod assets_loader;
mod bevy_gltf_collider;
mod components;
mod config;
mod cursor;
mod debug;
mod in_game;
mod menu;
mod settings;
mod splash;
mod ui;

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier3d::prelude::*;
use components::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Moria".into(),
                        name: Some("maria.app".into()),
                        position: WindowPosition::At(IVec2::new(100, 0)),
                        resolution: WindowResolution::new(1000.0, 650.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .init_state::<GameState>()
        .init_state::<InGameState>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((
            assets_loader::plugin,
            config::plugin,
            splash::plugin,
            menu::plugin,
            settings::plugin,
            in_game::InGamePlugins,
            ui::UiPlugins,
        ))
        .add_plugins(debug::plugin)
        .add_systems(PreStartup, (spawn_camera, spawn_ground))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn spawn_ground(mut commands: Commands) {
    // TODO: move to Maze
    commands.spawn((RigidBody::Fixed, Collider::halfspace(Vec3::Y).unwrap()));
}
