use bevy::prelude::*;

const PLAYER: &str = "player.png";

// LR, TB, TRBL, .
// LT, RB, RT, RB
// LTR, TRB, RBL, BLT

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.74, 0.74, 0.24)))
        .insert_resource(WindowDescriptor {
            title: "Moria - Rust".to_string(),
            width: 640.,
            height: 480.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let sprite_handle = assets.load(PLAYER);
    commands.spawn_bundle(SpriteBundle {
        texture: sprite_handle.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
