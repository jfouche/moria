use crate::ecs::*;
use bevy::prelude::*;

#[derive(Component)]
struct MyMusic;

pub fn plugin(app: &mut App) {
    app.insert_resource(AudioVolume(6))
        .add_systems(OnEnter(GameState::InGame), start_music)
        .add_systems(Update, change_volume.run_if(in_state(GameState::InGame)))
        // .add_systems(Update, _toggle.run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), despawn_all::<MyMusic>);
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    // println!("start_music()");
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/Goblins_Den_Regular.ogg"),
            settings: PlaybackSettings::LOOP,
        },
        MyMusic,
    ));
}

fn change_volume(volume: Res<AudioVolume>, sink: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(sink) = sink.get_single() {
        if volume.on() {
            sink.set_volume(volume.db());
            sink.play();
        } else {
            sink.pause();
        }
    }
}

fn _toggle(keys: Res<ButtonInput<KeyCode>>, sink: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(sink) = sink.get_single() {
        if keys.just_pressed(KeyCode::KeyM) {
            sink.toggle();
        }
    }
}
