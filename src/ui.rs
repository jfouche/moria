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

struct PositionConverter<'a> {
    win_size: &'a WinSize
}

impl<'a> PositionConverter<'a> {
    fn new(win_size: &'a WinSize) -> Self {
        PositionConverter { win_size }
    }

    /// Convert a Maze position to a screen position.
    /// 
    /// screen = 64 x pos - win_size / 2 + 30
    fn to_screen(&self, pos: &Position, z: f32) -> Vec3 {
        let x_offset = - self.win_size.w / 2. + 30.;
        let y_offset = - self.win_size.h / 2. + 30.;
        Vec3::new(64.0 * pos.x as f32 + x_offset, 64.0 * pos.y as f32 + y_offset, z)
    }

    ///  Convert a scrren position to a Maze position
    ///
    /// pos = (screen - 30 + win_size / 2) / 64
    fn to_position(&self, screen_pos: &Vec3) -> Position {
        let x_offset = (screen_pos.x - 30. + self.win_size.w / 2.) / 64.;
        let y_offset = (screen_pos.y - 30. + self.win_size.h / 2.) / 64.;
        Position { 
            x: x_offset as u32, 
            y: y_offset as u32 
        }
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


#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn it_converts_positions() {
        let win_size = WinSize {
            w: 30. + 64. * 5. + 30., 
            h: 30. + 64. * 4. + 30.
        };
        let pos_converter = PositionConverter::new(&win_size);

        // assert_eq!(pos_converter.to_position(Vec3::new(0., 0., 0.)), None);
        assert_eq!(pos_converter.to_position(&Vec3::new(40., 40., 0.)), Position {x:0, y: 0});

    }
}