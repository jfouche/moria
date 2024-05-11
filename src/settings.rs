use crate::components::{DisplaySettings, ExposureSettings};
use bevy::{prelude::*, render::camera::Exposure, window::PrimaryWindow};

pub fn plugin(app: &mut App) {
    app.insert_resource(DisplaySettings::Window)
        .insert_resource(ExposureSettings::Dark)
        .add_systems(Update, (change_display, change_exposure));
}

fn change_display(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    settings: Res<DisplaySettings>,
) {
    let mut window = windows.get_single_mut().expect("PrimaryWindow");
    if settings.is_changed() {
        window.mode = (*settings).into();
    }
}

fn change_exposure(settings: Res<ExposureSettings>, mut exposure: Query<&mut Exposure>) {
    *exposure.single_mut() = Exposure::from_physical_camera((*settings).into());
}
