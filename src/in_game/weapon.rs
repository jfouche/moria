use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<FireEvent>()
        .add_systems(Startup, load_assets)
        .add_systems(
            Update,
            (spawn_bullet, weapon_reloaded).in_set(InGameSet::EntityUpdate),
        );
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = WeaponAssets::load(&asset_server, meshes, materials);
    commands.insert_resource(assets);
}

fn spawn_bullet(
    mut commands: Commands,
    mut events: EventReader<FireEvent>,
    assets: Res<WeaponAssets>,
    sound_volume: Res<SoundVolume>,
) {
    for fire_ev in events.read() {
        info!("spawn_bullet from {:?}", fire_ev.from);
        commands.spawn(BulletBundle::new(fire_ev).with_assets(&assets));

        // spawn the audio in a different entity to be sure it doesn't stop
        // when the bullet is despawn to early
        commands.spawn(BulletSoundBundle::new(&assets, &sound_volume));
    }
}

fn weapon_reloaded(
    mut commands: Commands,
    time: Res<Time>,
    mut reloads: Query<(Entity, &mut Reload)>,
) {
    for (entity, mut reload) in reloads.iter_mut() {
        if reload.finished(&time) {
            commands.entity(entity).remove::<Reload>();
        }
    }
}
