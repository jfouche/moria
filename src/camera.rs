use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct PlayerCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera)
            .add_systems(PostUpdate, follow_player);
    }
}

fn init_camera(mut commands: Commands, mut transform: Query<Entity, With<Camera3d>>) {
    let camera_entity = transform
        .get_single_mut()
        .expect("Can't retrieve camera to init player");
    commands.entity(camera_entity).insert(PlayerCamera);
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
}
