use crate::{despawn_all, GameState, InGameStateSet};
use bevy::{audio::Volume, prelude::*};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct AudioVolume(pub u8);

impl AudioVolume {
    fn volume(&self) -> f32 {
        // TODO: modify algo
        // An increase of 10 decibels (dB) roughly corresponds to the perceived volume doubling in intensity.
        // As this function scales not the volume but the amplitude, a conversion might be necessary.
        // For example, to halve the perceived volume you need to decrease the volume by 10 dB.
        // This corresponds to 20log(x) = -10dB, solving x = 10^(-10/20) = 0.316.
        // Multiply the current volume by 0.316 to halve the perceived volume.

        self.0 as f32 / 9.0
    }
}

#[derive(Component)]
struct MyMusic;

pub fn plugin(app: &mut App) {
    app.insert_resource(AudioVolume(9))
        .add_systems(OnEnter(GameState::InGame), start_music)
        .add_systems(Update, change_volume.in_set(InGameStateSet::Running))
        .add_systems(OnExit(GameState::InGame), despawn_all::<MyMusic>);
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>, volume: Res<AudioVolume>) {
    info!("start_music()");
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/Goblins_Den_Regular.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(volume.volume())),
        },
        MyMusic,
    ));
}

fn change_volume(volume: Res<AudioVolume>, audio: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(settings) = audio.get_single() {
        info!("change_volume({})", volume.volume());
        settings.set_volume(volume.volume());
    }
}
