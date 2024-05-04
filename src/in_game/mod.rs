use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

mod audio;
mod bullet_collision;
mod camera;
mod character;
mod enemy;
mod hud;
mod maze;
mod minimap;
mod player;
mod weapon;

pub use audio::AudioVolume;
pub use player::Player;

use crate::{despawn_all, GameState, InGameState, InGameStateSet};
pub struct InGamePlugins;

impl PluginGroup for InGamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(maze::plugin)
            .add(player::plugin)
            .add(minimap::plugin)
            .add(hud::plugin)
            .add(weapon::plugin)
            .add(camera::plugin)
            .add(enemy::plugin)
            .add(bullet_collision::plugin)
            .add(audio::plugin)
            .add(in_game_plugin)
    }
}

#[derive(Component)]
struct MyMusic;

fn in_game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (grab_cursor, set_background, start_music),
    )
    .add_systems(
        OnExit(GameState::InGame),
        (ungrab_cursor, despawn_all::<MyMusic>),
    )
    .add_systems(Update, show_menu.in_set(InGameStateSet::Running));
}

const BACKGROUND_COLOR: Color = Color::BLACK;

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
}

fn grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = primary_window.get_single_mut().expect("PrimaryWindow");
    window.cursor.grab_mode = CursorGrabMode::Confined;
    window.cursor.visible = false;
}

fn ungrab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = primary_window.get_single_mut().expect("PrimaryWindow");
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

fn show_menu(mut state: ResMut<NextState<InGameState>>, keys: Res<ButtonInput<KeyCode>>) {
    for key in keys.get_pressed() {
        if *key == KeyCode::Escape {
            state.set(InGameState::Pause);
        }
    }
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/Goblins_Den_Regular.ogg"),
            settings: PlaybackSettings::LOOP,
        },
        MyMusic,
    ));
}
