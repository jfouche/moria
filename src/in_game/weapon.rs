use crate::{config::WeaponsConfig, ecs::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Resource)]
struct WeaponAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    sound: Handle<AudioSource>,
}

const BULLET_RADIUS: f32 = 0.03;
const BULLET_LENGTH: f32 = 0.1;

pub fn plugin(app: &mut App) {
    app.add_event::<FireEvent>()
        .add_systems(Startup, (load_assets, load_weapons))
        .add_systems(
            Update,
            (spawn_bullet, weapon_reloaded).run_if(game_is_running),
        );
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cylinder::new(BULLET_RADIUS, BULLET_LENGTH));
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
        let mut transform = Transform::from_translation(fire_ev.origin);
        transform.look_to(*fire_ev.direction, Vec3::Y);
        transform.rotate_local_x(FRAC_PI_2);
        commands.spawn((
            Bullet {
                damage: fire_ev.weapon.damage,
            },
            Name::new("BULLET"),
            fire_ev.from,
            PbrBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform,
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            Collider::cylinder(BULLET_LENGTH / 2.0, BULLET_RADIUS / 2.0),
            Velocity::linear(*fire_ev.direction * fire_ev.weapon.bullet_speed),
            ActiveEvents::COLLISION_EVENTS,
        ));

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
        if reload.tick(time.delta()).finished() {
            commands.entity(entity).remove::<Reload>();
        }
    }
}
