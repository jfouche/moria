use bevy::{prelude::*, window::PrimaryWindow};

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

pub fn plugin(app: &mut App) {
    app.insert_resource(DisplaySettings::Window)
        .add_systems(Update, change_display);
}

fn change_display(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    settings: Res<DisplaySettings>,
) {
    let mut window = windows.get_single_mut().expect("PrimaryWindow");
    if settings.is_changed() {
        window.mode = match *settings {
            DisplaySettings::Window => bevy::window::WindowMode::Windowed,
            DisplaySettings::FullScreen => bevy::window::WindowMode::Fullscreen,
        };
    }
}
