use crate::ecs::*;
use bevy::{audio::Volume, prelude::*};

#[derive(Component)]
struct MyMusic;

pub fn plugin(app: &mut App) {
    app.insert_resource(AudioVolume(9))
        .add_systems(OnEnter(GameState::InGame), start_music)
        .add_systems(Update, change_volume.run_if(game_is_running))
        .add_systems(OnExit(GameState::InGame), despawn_all::<MyMusic>);
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>, volume: Res<AudioVolume>) {
    info!("start_music()");
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/Goblins_Den_Regular.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(volume.db())),
        },
        MyMusic,
    ));
}

fn change_volume(volume: Res<AudioVolume>, audio: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(settings) = audio.get_single() {
        settings.set_volume(volume.db());
    }
}
