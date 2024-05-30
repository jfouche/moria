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

// WeaponConfig
#[derive(Debug, Deserialize, Default)]
pub struct WeaponConfig {
    pub name: String,
    pub damage: u16,
    pub reload_delay: f32,
    pub bullet_speed: f32,
    pub bullet_distance: f32,
}

// LevelConfig
#[derive(Debug, Deserialize, Resource)]
pub struct LevelConfig {
    pub rows: u32,
    pub cols: u32,
    pub enemy_density: f32,
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
