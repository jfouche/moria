use crate::components::*;
use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

// MoriaConfig
#[derive(Default, Debug, Deserialize)]
struct MoriaConfig {
    game: GameConfig,
    camera: CameraConfig,
    weapons: Vec<WeaponConfig>,
    levels: Vec<LevelConfig>,
}

// plugin
pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, load_config);
}

fn load_config(mut commands: Commands) {
    match fs::read_to_string("moria.toml") {
        Ok(content) => match toml::from_str::<MoriaConfig>(&content) {
            Ok(config) => {
                commands.insert_resource(config.game);
                commands.insert_resource(config.camera);
                commands.insert_resource(WeaponsConfig::new(config.weapons));
                commands.insert_resource(LevelsConfig::new(config.levels));
            }
            Err(e) => error!("Can't load config from file : {e:?}"),
        },
        Err(e) => error!("Can't read config file : {e:?}"),
    }
}
