use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use crate::GameState;

#[derive(Event)]
pub struct FireEvent {
    pub origin: Vec3,
    pub direction: Direction3d,
}

#[derive(Component)]
pub struct Weapon {
    damage: u16,
    /// in secs
    reload_time: f32,
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
pub struct Ammo;

const AMMO_RADIUS: f32 = 0.03;
const AMMO_LENGTH: f32 = 0.1;

pub fn plugin(app: &mut App) {
    app.add_event::<FireEvent>()
        .add_systems(Update, spawn_ammo.run_if(in_state(GameState::Game)));
}

fn spawn_ammo(
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
            Ammo,
            Name::new("AMMO"),
            PbrBundle {
                mesh: meshes.add(Cylinder::new(AMMO_RADIUS, AMMO_LENGTH)),
                material: materials.add(Color::ORANGE),
                transform,
                ..default()
            },
            // RigidBody::Dynamic,
            // Collider::cylinder(AMMO_LENGTH / 2.0, AMMO_RADIUS / 2.0),
        ));
    }
}
