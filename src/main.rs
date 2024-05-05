mod config;
mod core;
mod cursor;
mod debug;
mod in_game;
mod menu;
mod splash;
mod ui;

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    InGame,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    #[default]
    Disabled,
    Running,
    Pause,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InGameStateSet {
    Running,
    Pause,
}

fn game_is_running(
    game_state: Res<State<GameState>>,
    in_game_state: Res<State<InGameState>>,
) -> bool {
    *game_state == GameState::InGame && *in_game_state == InGameState::Running
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
        .init_state::<InGameState>()
        .configure_sets(Update, InGameStateSet::Running.run_if(game_is_running))
        .configure_sets(PostUpdate, InGameStateSet::Running.run_if(game_is_running))
        .add_plugins((
            splash::plugin,
            config::plugin,
            menu::plugin,
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
    commands.spawn((RigidBody::Fixed, Collider::halfspace(Vec3::Y).unwrap()));
}

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
