use std::ops::Range;

use bevy::prelude::*;

/// Audio volume
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct AudioVolume(pub u8);

impl AudioVolume {
    const MAX: u8 = 7;

    pub const fn range() -> Range<u8> {
        0..Self::MAX + 1
    }

    pub fn db(&self) -> f32 {
        const DECIBELS: [f32; 8] = [0.0, 0.04, 0.08, 0.1, 0.3, 0.6, 1.0, 2.0];
        *DECIBELS
            .get(self.0 as usize)
            .unwrap_or(&DECIBELS[Self::MAX as usize])
    }
}

/// Display setting
#[derive(Resource, Debug, Component, Clone, Copy, PartialEq)]
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
#[derive(Resource, Debug, Component, Clone, Copy, PartialEq)]
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
        }
    }
}
