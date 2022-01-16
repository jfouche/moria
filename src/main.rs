use bevy::prelude::*;
use ui::player_plugin::PlayerPlugin;

mod ui;

const PLAYER: &str = "player.png";

pub const TIME_STEP: f32 = 1.0 / 20.;

// RESSOURCES
struct Materials {
    player: Handle<Image>,
}

// COMPONENTS

#[derive(Component)]
struct MazeComponent {
}

impl Default for MazeComponent {
    fn default() -> Self {
        MazeComponent {  }
    }
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
        .add_plugin(PlayerPlugin)
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
