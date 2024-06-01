mod camera;
mod character;
mod config;
mod enemy;
mod hud;
mod item;
mod level;
mod maze;
mod player;
mod settings;
mod states;
mod weapon;

pub use camera::*;
pub use character::*;
pub use config::*;
pub use enemy::*;
pub use hud::*;
pub use item::*;
pub use level::*;
pub use maze::*;
pub use player::*;
pub use settings::*;
pub use states::*;
pub use weapon::*;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(PhysicsLayer)]
pub enum InGameLayers {
    Player,
    Enemy,
    Item,
    Bullet,
    Ground,
}

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

/// Generic system to set a the Visibility of a Component
///
/// ex:
/// ```
/// app.add_system(set_visibility::<MyComponent>(Visibility::Hidden));
///
/// ```
pub fn set_visibility<T: Component>(
    visibility: Visibility,
) -> impl FnMut(Query<&mut Visibility, With<T>>) {
    move |mut query| {
        for mut v in query.iter_mut() {
            *v = visibility;
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct LifeTime(Timer);

impl LifeTime {
    pub fn new(secs: f32) -> Self {
        LifeTime(Timer::from_seconds(secs, TimerMode::Once))
    }
}

pub fn despawn_if_too_old(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut query {
        if lifetime.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
