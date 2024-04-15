mod camera;
mod config;
mod core;
mod debug;
mod hud;
mod maze;
mod minimap;
mod player;

use bevy::{
    prelude::*,
    window::{Cursor, CursorGrabMode, WindowResolution},
};
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

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
                        cursor: Cursor {
                            grab_mode: CursorGrabMode::Confined,
                            visible: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.)))
        .add_plugins((
            config::plugin,
            minimap::plugin,
            maze::plugin,
            player::plugin,
            hud::plugin,
            camera::plugin,
        ))
        .add_plugins(debug::plugin)
        .add_systems(PreStartup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle::default());

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(50.0, 50.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::MAROON,
                perceptual_roughness: 0.9,
                ..default()
            }),
            ..default()
        },
        RigidBody::Fixed,
        Collider::halfspace(Vec3::Y).unwrap(),
    ));
}
