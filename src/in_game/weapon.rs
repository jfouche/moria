use crate::{components::*, config::WeaponsConfig, schedule::InGameSet};
use bevy::prelude::*;

#[derive(Resource)]
struct WeaponAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    sound: Handle<AudioSource>,
}

pub fn plugin(app: &mut App) {
    app.add_event::<FireEvent>()
        .add_systems(Startup, (load_assets, load_weapons))
        .add_systems(
            Update,
            (spawn_bullet, weapon_reloaded).in_set(InGameSet::EntityUpdate),
        );
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cylinder::new(Bullet::RADIUS, Bullet::LENGTH));
    let material = materials.add(Color::ORANGE);
    let sound = asset_server.load("audio/556-Single-Isolated.ogg");
    let assets = WeaponAssets {
        mesh,
        material,
        sound,
    };
    commands.insert_resource(assets);
}

fn load_weapons(mut commands: Commands, config: Res<WeaponsConfig>) {
    let mut weapons = Weapons::new();
    for conf in config.0.iter() {
        if let Ok(weapon_type) = WeaponType::try_from(conf.name.as_str()) {
            weapons.insert(weapon_type, conf.into());
        } else {
            error!("Invalid weapon config");
            panic!();
        }
    }
    commands.insert_resource(weapons);
}

fn spawn_bullet(
    mut commands: Commands,
    mut events: EventReader<FireEvent>,
    assets: Res<WeaponAssets>,
) {
    for fire_ev in events.read() {
        info!("spawn_bullet from {:?}", fire_ev.from);
        let mesh = assets.mesh.clone();
        let material = assets.material.clone();
        commands.spawn(BulletBundle::new(fire_ev).with_pbr(mesh, material));

        // spawn the audio in a different entity to be sure it doesn't stop
        // when the bullet is despawn to early
        commands.spawn((
            Name::new("Bullet sound"),
            AudioBundle {
                source: assets.sound.clone(),
                settings: PlaybackSettings::DESPAWN,
            },
        ));
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
