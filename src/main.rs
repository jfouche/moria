use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
    window::{close_on_esc, Cursor, CursorGrabMode, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::PanOrbitCamera;
use player::Player;

use crate::maze::Position;

mod camera;
mod hud;
mod maze;
mod minimap;
mod player;

#[cfg(test)]
mod test;

fn main() {
    // eprintln!("{}", maze.to_string());

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Moria".into(),
                        name: Some("maria.app".into()),
                        position: WindowPosition::At(IVec2::new(0, 0)),
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
            minimap::MinimapPlugin,
            maze::MazePlugin,
            player::PlayerPlugin,
            hud::HudPlugin,
            camera::CameraPlugin,
        ))
        .add_systems(PreStartup, setup)
        // DEBUG
        .add_systems(Update, close_on_esc)
        // .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Update, toggle_camera_controls_system)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(
            Update,
            (debug_player_view).run_if(on_timer(Duration::from_secs(1))),
        )
        .add_systems(Update, show_axes)
        // RUN
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((Camera3dBundle::default(), PanOrbitCamera::default()));

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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(50.0, 50.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::MAROON,
            perceptual_roughness: 0.9,
            ..default()
        }),
        ..default()
    });
}

fn toggle_camera_controls_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    if key_input.just_pressed(KeyCode::KeyT) {
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.enabled = !pan_orbit.enabled;
        }
    }
}

fn debug_player_view(transform: Query<&Transform, With<Player>>) {
    let transform = transform.get_single().expect("Can't get Player");
    let translation = transform.translation;
    let pos: Position = translation.into();
    let mut forward = *transform.forward();
    forward.y = 0.0;
    let angle = forward.angle_between(Vec3::Z).to_degrees();
    let forward = forward.xz();
    info!("Player translation: {translation}, pos: {pos}, forward: {forward:?}, angle: {angle}");
}

fn show_axes(mut gizmos: Gizmos) {
    gizmos.ray(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Color::RED);
    gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), Color::GREEN);
    gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0), Color::BLUE);
}
