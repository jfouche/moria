use bevy::prelude::*;

const PLAYER: &str = "player.png";

// ressources
struct Materials {
    player: Handle<Image>
}


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
        .add_startup_stage("game_setup_actors", SystemStage::single(player_spawn.system()))
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Resources
    commands.insert_resource(Materials {
        player: assets.load(PLAYER).into()
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>){
    commands.spawn_bundle(SpriteBundle {
        texture: materials.player.clone(),
        ..Default::default()
    });
}
