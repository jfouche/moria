use bevy::{prelude::*, render::camera::PhysicalCameraParameters};
use serde::Deserialize;
use std::fs;

// MoriaConfig

#[derive(Default, Debug, Deserialize)]
struct MoriaConfig {
    game: GameConfig,
    maze: MazeConfig,
    camera: CameraConfig,
    weapons: Vec<WeaponConfig>,
}

#[derive(Default, Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub debug: bool,
}

// MazeConfig

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

// CameraConfig

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

// WeaponConfig

#[derive(Debug, Deserialize, Default)]
pub struct WeaponConfig {
    pub name: String,
    pub damage: u16,
    pub bullet_speed: f32,
    pub reload_delay: f32,
}

#[derive(Resource)]
pub struct WeaponsConfig(pub Vec<WeaponConfig>);

// plugin

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, load_config);
}

fn load_config(mut commands: Commands) {
    match fs::read_to_string("moria.toml") {
        Ok(content) => match toml::from_str::<MoriaConfig>(&content) {
            Ok(config) => {
                commands.insert_resource(config.game);
                commands.insert_resource(config.maze);
                commands.insert_resource(config.camera);
                commands.insert_resource(WeaponsConfig(config.weapons));
            }
            Err(e) => error!("Can't load config from file : {e:?}"),
        },
        Err(e) => error!("Can't read config file : {e:?}"),
    }
}
