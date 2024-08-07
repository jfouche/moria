use bevy::{audio::Volume, prelude::*};
use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, ops::RangeInclusive, path::PathBuf};

/// Audio volume
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Deref)]
pub struct AudioVolume(pub u8);

impl AudioVolume {
    const MAX: u8 = 7;

    pub const fn range() -> RangeInclusive<u8> {
        0..=Self::MAX
    }

    pub fn db(&self) -> f32 {
        const DECIBELS: [f32; 8] = [0.0, 0.07, 0.1, 0.25, 0.4, 0.7, 1.0, 2.0];
        *DECIBELS
            .get(self.0 as usize)
            .unwrap_or(&DECIBELS[Self::MAX as usize])
    }
}

/// Music volume
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Deref)]
pub struct MusicVolume(pub AudioVolume);

impl PartialEq<u8> for MusicVolume {
    fn eq(&self, other: &u8) -> bool {
        &***self == other
    }
}

impl From<&MusicVolume> for Volume {
    fn from(volume: &MusicVolume) -> Self {
        Volume::new(volume.db())
    }
}

/// Sound volume (like bullet for ex)
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Deref)]
pub struct SoundVolume(pub AudioVolume);

impl PartialEq<u8> for SoundVolume {
    fn eq(&self, other: &u8) -> bool {
        &***self == other
    }
}
/// Display setting
#[derive(Resource, Debug, Component, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum DisplaySettings {
    FullScreen,
    Window,
}

impl std::fmt::Display for DisplaySettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            DisplaySettings::Window => "Window",
            DisplaySettings::FullScreen => "Full screen",
        };
        write!(f, "{label}")
    }
}

impl From<DisplaySettings> for bevy::window::WindowMode {
    fn from(value: DisplaySettings) -> Self {
        match value {
            DisplaySettings::Window => bevy::window::WindowMode::Windowed,
            DisplaySettings::FullScreen => bevy::window::WindowMode::Fullscreen,
        }
    }
}

/// Gamma exposure
#[derive(Resource, Debug, Component, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ExposureSettings {
    Dark,
    Medium,
    Light,
}

impl std::fmt::Display for ExposureSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            ExposureSettings::Dark => "Dark",
            ExposureSettings::Medium => "Medium",
            ExposureSettings::Light => "Light",
        };
        write!(f, "{label}")
    }
}

impl From<ExposureSettings> for bevy::render::camera::PhysicalCameraParameters {
    fn from(value: ExposureSettings) -> Self {
        let (aperture_f_stops, shutter_speed_s, sensitivity_iso) = match value {
            ExposureSettings::Dark => (8.0, 1.0 / 125.0, 100.0),
            ExposureSettings::Medium => (5.0, 1.0 / 125.0, 200.0),
            ExposureSettings::Light => (3.0, 1.0 / 125.0, 200.0),
        };
        bevy::render::camera::PhysicalCameraParameters {
            aperture_f_stops,
            shutter_speed_s,
            sensitivity_iso,
            sensor_height: 0.01866,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Settings {
    music_volume: AudioVolume,
    sound_volume: AudioVolume,
    display: DisplaySettings,
    exposure: ExposureSettings,
}

const MORIA_SETTINGS_DIR: &str = "Moria";
const MORIA_SETTINGS_FILE: &str = ".moria";

fn settings_path() -> PathBuf {
    let dir = dirs::data_dir().expect("DataDir").join(MORIA_SETTINGS_DIR);
    match dir.try_exists() {
        Err(err) => error!("Can't check if dir {dir:?} exists : {err:?}"),
        Ok(false) => {
            if let Err(err) = fs::create_dir(&dir) {
                error!("Can't create settings dir {dir:?} : {err:?}")
            }
        }
        _ => {}
    }
    dir.join(MORIA_SETTINGS_FILE)
}

pub fn load_settings(
    mut music_volume: ResMut<MusicVolume>,
    mut sound_volume: ResMut<SoundVolume>,
    mut display: ResMut<DisplaySettings>,
    mut exposure: ResMut<ExposureSettings>,
) {
    let path = settings_path();
    match fs::read_to_string(&path) {
        Ok(content) => match toml::from_str::<Settings>(&content) {
            Ok(settings) => {
                *music_volume = MusicVolume(settings.music_volume);
                *sound_volume = SoundVolume(settings.sound_volume);
                *display = settings.display;
                *exposure = settings.exposure;
            }
            Err(e) => error!("Can't load config from file {path:?}: {e:?}"),
        },
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {}
            _ => error!("{}", format!("Can't load settings {path:?}: {err:?}")),
        },
    }
}

pub fn save_settings(
    music_volume: Res<MusicVolume>,
    sound_volume: Res<SoundVolume>,
    display: Res<DisplaySettings>,
    exposure: Res<ExposureSettings>,
) {
    let settings = Settings {
        music_volume: **music_volume,
        sound_volume: **sound_volume,
        display: *display,
        exposure: *exposure,
    };
    match toml::to_string_pretty(&settings) {
        Ok(content) => {
            if let Err(err) = fs::write(settings_path(), content) {
                error!("{}", format!("Can't save settings: {err:?}"));
            }
        }
        Err(err) => error!("{}", format!("Can't save settings: {err:?}")),
    }
}
