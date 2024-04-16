use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

mod hud;
mod maze;
mod minimap;
mod player;

pub use player::Player;

use crate::GameState;

pub struct InGamePlugins;

impl PluginGroup for InGamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(maze::plugin)
            .add(player::plugin)
            .add(minimap::plugin)
            .add(hud::plugin)
            .add(in_game_plugin)
    }
}

fn in_game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), (grab_cursor, set_background))
        .add_systems(OnExit(GameState::Game), ungrab_cursor)
        .add_systems(Update, show_menu.run_if(in_state(GameState::Game)));
}

const BACKGROUND_COLOR: Color = Color::BLACK;

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
}

fn grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = primary_window.get_single_mut().expect("Can't get window");
    window.cursor.grab_mode = CursorGrabMode::Confined;
    window.cursor.visible = false;
}

fn ungrab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = primary_window.get_single_mut().expect("Can't get window");
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

fn show_menu(mut state: ResMut<NextState<GameState>>, keys: Res<ButtonInput<KeyCode>>) {
    for key in keys.get_pressed() {
        if *key == KeyCode::Escape {
            state.set(GameState::Menu);
        }
    }
}
