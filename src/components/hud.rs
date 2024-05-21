use bevy::prelude::*;

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct HudCompass;

#[derive(Component)]
pub struct HudFps;

#[derive(Component)]
pub struct HudFpsText;

#[derive(Component)]
pub struct HudAim;

#[derive(Component)]
pub struct HudLife;

#[derive(Resource)]
pub struct HudAssets {
    pub font: Handle<Font>,
    pub aim: Handle<Image>,
    pub compass: Handle<Image>,
}

impl HudAssets {
    pub fn from_asset_server(asset_server: &AssetServer) -> Self {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let aim = asset_server.load("aim.png");
        let compass = asset_server.load("compass.png");
        HudAssets { font, aim, compass }
    }
}
