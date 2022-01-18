use bevy::prelude::*;

use crate::ui::{Materials, TIME_STEP};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system());
    }
}

#[derive(Component)]
struct PlayerComponent;

#[derive(Component)]
struct PlayerSpeedComponent(f32);

impl Default for PlayerSpeedComponent {
    fn default() -> Self {
        PlayerSpeedComponent(300.)
    }
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: materials.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., - 28., 20.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerComponent)
        .insert(PlayerSpeedComponent::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeedComponent, &mut Transform, With<PlayerComponent>)>,
) {
    if let Ok((speed, mut transform, _)) = query.get_single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        } else {
            0.
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}
