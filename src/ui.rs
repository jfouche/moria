use bevy::prelude::*;
use moria::maze::Position;

pub mod maze_plugin;
pub mod player_plugin;

const PLAYER: &str = "player.png";
const MAZE: &str = "maze.png";

pub const TIME_STEP: f32 = 1.0 / 20.;

// region:    Resources
pub struct Materials {
    player: Handle<Image>,
    maze: Handle<TextureAtlas>,
}

struct WinSize {
	w: f32,
	h: f32,
}
// endregion: Resources

struct PositionToScreen<'a> {
    win_size: &'a WinSize
}

impl<'a> PositionToScreen<'a> {
    fn new(win_size: &'a WinSize) -> Self {
        PositionToScreen { win_size }
    }

    fn to_screen(&self, pos: &Position, z: f32) -> Vec3 {
        let x_offset = - self.win_size.w / 2. + 30.;
        let y_offset = - self.win_size.h / 2. + 30.;
        Vec3::new(64.0 * pos.x as f32 + x_offset, 64.0 * pos.y as f32 + y_offset, z)
    }
}

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();

    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Resources
    let texture_handle = assets.load(MAZE);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    commands.insert_resource(Materials {
        player: assets.load(PLAYER),
        maze: texture_atlases.add(texture_atlas),
    });
    commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height(),
	});
}
