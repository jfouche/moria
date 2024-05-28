use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;

#[derive(Component)]
struct MyMusic;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), start_music)
        .add_systems(Update, change_music_volume.in_set(InGameSet::EntityUpdate))
        .add_systems(OnExit(GameState::InGame), despawn_all::<MyMusic>);
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Music"),
        AudioBundle {
            source: asset_server.load("audio/Goblins_Den_Regular.ogg"),
            settings: PlaybackSettings::LOOP,
        },
        MyMusic,
    ));
}

fn change_music_volume(volume: Res<MusicVolume>, sink: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(sink) = sink.get_single() {
        sink.set_volume(volume.db());
    }
}
