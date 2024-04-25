mod config;
mod core;
mod debug;
mod in_game;
mod menu;
mod splash;

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

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
        .add_plugins((
            splash::plugin,
            config::plugin,
            menu::plugin,
            in_game::InGamePlugins,
        ))
        .add_plugins(debug::plugin)
        .add_systems(PreStartup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle::default());

    // ground
    commands.spawn((RigidBody::Fixed, Collider::halfspace(Vec3::Y).unwrap()));
}

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
