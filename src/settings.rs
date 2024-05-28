use crate::components::*;
use bevy::{prelude::*, render::camera::Exposure, window::PrimaryWindow};

pub fn plugin(app: &mut App) {
    app.insert_resource(DisplaySettings::Window)
        .insert_resource(ExposureSettings::Dark)
        .insert_resource(MusicVolume(AudioVolume(4)))
        .insert_resource(SoundVolume(AudioVolume(5)))
        .add_systems(Startup, load_settings)
        .add_systems(OnExit(GameState::Menu), save_settings)
        .add_systems(OnExit(InGameState::Pause), save_settings)
        .add_systems(Update, (change_display, change_exposure));
}

fn change_display(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    settings: Res<DisplaySettings>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        if settings.is_changed() {
            window.mode = (*settings).into();
        }
    }
}

fn change_exposure(settings: Res<ExposureSettings>, mut exposure: Query<&mut Exposure>) {
    *exposure.single_mut() = Exposure::from_physical_camera((*settings).into());
}
