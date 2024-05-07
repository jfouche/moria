use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct AudioVolume(pub u8);

impl AudioVolume {
    pub fn db(&self) -> f32 {
        // TODO: modify algo
        // An increase of 10 decibels (dB) roughly corresponds to the perceived volume doubling in intensity.
        // As this function scales not the volume but the amplitude, a conversion might be necessary.
        // For example, to halve the perceived volume you need to decrease the volume by 10 dB.
        // This corresponds to 20log(x) = -10dB, solving x = 10^(-10/20) = 0.316.
        // Multiply the current volume by 0.316 to halve the perceived volume.

        self.0 as f32 / 9.0
    }
}

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
