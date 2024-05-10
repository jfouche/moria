mod config;
mod debug;
mod ecs;
mod in_game;
mod menu;
mod settings;
mod splash;
mod ui;

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier3d::prelude::*;

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
        .init_state::<ecs::GameState>()
        .init_state::<ecs::InGameState>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((
            splash::plugin,
            config::plugin,
            menu::plugin,
            settings::plugin,
            in_game::InGamePlugins,
            ui::UiPlugins,
        ))
        .add_plugins(debug::plugin)
        .add_systems(PreStartup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle::default());

    // ground
    // TODO: move to Maze
    commands.spawn((RigidBody::Fixed, Collider::halfspace(Vec3::Y).unwrap()));
}
