mod components;
mod config;
mod cursor;
mod in_game;
mod math;
mod menu;
mod schedule;
mod settings;
mod splash;
mod ui;

#[cfg(debug_assertions)]
mod debug;

use bevy::{prelude::*, window::WindowResolution};
use bevy_xpbd_3d::prelude::*;
use components::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
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
    .add_plugins(PhysicsPlugins::default())
    .add_plugins((
        schedule::plugin,
        config::plugin,
        splash::plugin,
        menu::plugin,
        settings::plugin,
        in_game::InGamePlugins,
        ui::UiPlugins,
    ))
    .add_systems(Startup, setup_physics);

    #[cfg(debug_assertions)]
    app.add_plugins(debug::plugin);

    app.run();
}

fn setup_physics() {}
