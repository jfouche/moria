use bevy::prelude::*;

pub mod maze_plugin;
pub mod player_plugin;

const PLAYER: &str = "player.png";
const MAZE: &str = "maze.png";

pub const TIME_STEP: f32 = 1.0 / 20.;

// RESSOURCES
pub struct Materials {
    player: Handle<Image>,
    maze: Handle<TextureAtlas>,
}

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Resources
    let texture_handle = assets.load(MAZE);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    commands.insert_resource(Materials {
        player: assets.load(PLAYER).into(),
        maze: texture_atlases.add(texture_atlas),
    });
}
