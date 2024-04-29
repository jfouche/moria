use std::fs;

use bevy::{prelude::*, render::camera::PhysicalCameraParameters};
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
struct MoriaConfig {
    game: GameConfig,
    maze: MazeConfig,
    camera: CameraConfig,
}

#[derive(Default, Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub debug: bool,
}

#[derive(Debug, Deserialize, Resource)]
pub struct MazeConfig {
    pub rows: u32,
    pub cols: u32,
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self { rows: 5, cols: 5 }
    }
}

#[derive(Debug, Deserialize, Resource)]
pub struct CameraConfig {
    pub aperture_f_stops: f32,
    pub shutter_speed_s: f32,
    pub sensitivity_iso: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        let default = PhysicalCameraParameters::default();
        CameraConfig {
            aperture_f_stops: default.aperture_f_stops,
            shutter_speed_s: default.shutter_speed_s,
            sensitivity_iso: default.sensitivity_iso,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<GameConfig>()
        .init_resource::<MazeConfig>()
        .init_resource::<CameraConfig>()
        .add_systems(PreStartup, load_config);
}

fn load_config(
    mut game: ResMut<GameConfig>,
    mut maze: ResMut<MazeConfig>,
    mut camera: ResMut<CameraConfig>,
) {
    if let Ok(content) = fs::read_to_string("moria.toml") {
        match toml::from_str::<MoriaConfig>(&content) {
            Ok(config) => {
                *game = config.game;
                *maze = config.maze;
                *camera = config.camera;
            }
            Err(e) => error!("Can't load config file : {e:?}"),
        };
    }
}
