mod character;
mod cursor;
mod enemy;
mod maze;
mod player;
mod position;
mod settings;
mod states;
mod weapon;

pub use character::*;
pub use cursor::*;
pub use enemy::*;
pub use maze::*;
pub use player::*;
pub use position::*;
pub use settings::*;
pub use states::*;
pub use weapon::*;

use bevy::prelude::*;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
