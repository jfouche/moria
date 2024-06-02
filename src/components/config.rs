use bevy::{prelude::*, render::camera::PhysicalCameraParameters};
use serde::Deserialize;

use crate::CurrentLevel;

#[derive(Default, Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub debug: bool,
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

/// WeaponConfig
#[derive(Debug, Deserialize, Default)]
pub struct WeaponConfig {
    pub name: String,
    pub damage: u16,
    pub reload_delay: f32,
    pub bullet_config: BulletConfig,
}

/// BulletConfig
#[derive(Debug, Deserialize, Default, Clone)]
pub struct BulletConfig {
    pub speed: f32,
    pub distance: f32,
    pub radius: f32,
    pub length: f32,
}

// LevelConfig
#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    pub rows: u32,
    pub cols: u32,
    /// `enemy_density` is the percent of enemies according to the number of rooms
    pub enemy_density: f32,
    pub enemy_bonus: f32,
    /// `item_density` is the percent of item according to the number of rooms
    pub item_density: f32,
}

#[derive(Resource)]
pub struct LevelsConfig(Vec<LevelConfig>);

impl LevelsConfig {
    pub fn new(levels_config: Vec<LevelConfig>) -> Self {
        LevelsConfig(levels_config)
    }

    pub fn get(&self, level: &CurrentLevel) -> Option<&LevelConfig> {
        self.0.get(**level)
    }

    pub fn next_level(&self, level: &CurrentLevel) -> Option<usize> {
        let level = **level;
        if level < self.0.len() - 1 {
            Some(level + 1)
        } else {
            None
        }
    }
}
