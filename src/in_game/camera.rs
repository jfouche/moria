use bevy::{
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};

use crate::{config::CameraConfig, in_game::Player, InGameStateSet};

#[derive(Component)]
pub struct PlayerCamera;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (init_camera, load_config))
        .add_systems(PostUpdate, follow_player.in_set(InGameStateSet::Running));
}

fn init_camera(mut commands: Commands, mut transform: Query<Entity, With<Camera3d>>) {
    let camera_entity = transform
        .get_single_mut()
        .expect("Can't retrieve camera to init player");
    commands.entity(camera_entity).insert(PlayerCamera);
}

fn load_config(config: Res<CameraConfig>, mut exposure: Query<&mut Exposure>) {
    let params = PhysicalCameraParameters {
        aperture_f_stops: config.aperture_f_stops,
        shutter_speed_s: config.shutter_speed_s,
        sensitivity_iso: config.sensitivity_iso,
    };
    *exposure.single_mut() = Exposure::from_physical_camera(params);
}

fn follow_player(
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let player_transform = player.get_single().expect("Can't retrieve Player");
    let mut cam_transform = camera
        .get_single_mut()
        .expect("Can't retrieve PlayerCamera");
    *cam_transform = *player_transform;
    cam_transform.translation.y += Player::HEIGHT;
}
