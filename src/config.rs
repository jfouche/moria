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
                load_weapons(commands.reborrow(), &config.weapons);
                commands.insert_resource(LevelsConfig::new(config.levels));
            }
            Err(e) => error!("Can't load config from file : {e:?}"),
        },
        Err(e) => error!("Can't read config file : {e:?}"),
    }
}

fn load_weapons(mut commands: Commands, weapons_config: &[WeaponConfig]) {
    let mut weapons = Weapons::new();
    for conf in weapons_config.iter() {
        if let Ok(weapon_type) = WeaponType::try_from(conf.name.as_str()) {
            weapons.insert(weapon_type, conf.into());
        } else {
            error!("Invalid weapon config");
            panic!();
        }
    }
    commands.insert_resource(weapons);
}
