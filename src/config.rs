use std::fs;

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
struct MoriaConfig {
    pub game: GameConfig,
    pub maze: MazeConfig,
}

#[derive(Default, Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub debug: bool,
}

#[derive(Debug, Deserialize, Resource)]
pub struct MazeConfig {
    pub rows: u32,
    pub cols: u32,
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self { rows: 5, cols: 5 }
    }
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameConfig>()
            .init_resource::<MazeConfig>()
            .add_systems(PreStartup, load_config);
    }
}

fn load_config(mut game: ResMut<GameConfig>, mut maze: ResMut<MazeConfig>) {
    if let Ok(content) = fs::read_to_string("moria.toml") {
        match toml::from_str::<MoriaConfig>(&content) {
            Ok(config) => {
                *game = config.game;
                *maze = config.maze;
            }
            Err(e) => error!("Can't load config file : {e:?}"),
        };
    }
}
