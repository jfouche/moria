use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
    window::{close_on_esc, Cursor, CursorGrabMode, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use player::Player;

use crate::maze::Position;

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
    // let window = windows.get_single().unwrap();

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

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
        mesh: meshes.add(Cuboid::new(20.0, 1.0, 20.0)),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..default()
    });
}

// fn log_events(wnds: Res<Windows>) {
//     let wnd = wnds.get_primary().unwrap();
//     if let Some(pos) = wnd.cursor_position() {
//         info!("mouse pos : {}", pos);
//     }
// }

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

fn debug_player_view(
    mut transform: Query<&mut Transform, With<Player>>,
    // keys: Res<ButtonInput<KeyCode>>,
    // time: Res<Time>,
) {
    let transform = transform.get_single_mut().expect("Can't get player camera");
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
