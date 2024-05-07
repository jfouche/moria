use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct AudioVolume(pub u8);

impl AudioVolume {
    pub fn db(&self) -> f32 {
        const DECIBELS: [f32; 10] = [0.0, 0.02, 0.05, 0.1, 0.02, 0.5, 1.0, 2.0, 5.0, 10.0];
        *DECIBELS.get(self.0 as usize).unwrap_or(&1.0)
    }

    pub fn on(&self) -> bool {
        self.0 != 0
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
