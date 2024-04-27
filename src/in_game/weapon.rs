use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{RigidBody, Velocity},
    geometry::{ActiveEvents, Collider},
};
use std::f32::consts::FRAC_PI_2;

#[derive(Event)]
pub struct FireEvent {
    pub from: FireEmitter,
    pub origin: Vec3,
    pub direction: Direction3d,
    pub damage: u16,
    pub speed: f32,
}

#[derive(Component, Clone, Copy)]
pub enum FireEmitter {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Weapon {
    pub damage: u16,
    /// in secs
    pub reload_time: f32,
}

impl Weapon {
    pub const GUN: Weapon = Weapon {
        damage: 10,
        reload_time: 0.4,
    };
    pub const SHOTGUN: Weapon = Weapon {
        damage: 35,
        reload_time: 0.9,
    };
}

#[derive(Component)]
pub struct Bullet {
    pub damage: u16,
}

const BULLET_RADIUS: f32 = 0.03;
const BULLET_LENGTH: f32 = 0.1;

pub fn plugin(app: &mut App) {
    app.add_event::<FireEvent>()
        .add_systems(Update, spawn_bullet.run_if(in_state(GameState::Game)));
}

fn spawn_bullet(
    mut commands: Commands,
    mut events: EventReader<FireEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for fire_ev in events.read() {
        let mut transform = Transform::from_translation(fire_ev.origin);
        transform.look_to(*fire_ev.direction, Vec3::Y);
        transform.rotate_local_x(FRAC_PI_2);
        commands.spawn((
            Bullet {
                damage: fire_ev.damage,
            },
            Name::new("BULLET"),
            fire_ev.from,
            PbrBundle {
                mesh: meshes.add(Cylinder::new(BULLET_RADIUS, BULLET_LENGTH)),
                material: materials.add(Color::ORANGE),
                transform,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cylinder(BULLET_LENGTH / 2.0, BULLET_RADIUS / 2.0),
            Velocity::linear(*fire_ev.direction * fire_ev.speed),
            ActiveEvents::COLLISION_EVENTS,
            // ColliderMassProperties::Density(0.0005), // TODO: don't har code
        ));
    }
}
