use bevy::prelude::*;

use crate::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdate,
    CollisionDetection,
    DespawnEntities,
}

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            InGameSet::UserInput,
            InGameSet::EntityUpdate,
            InGameSet::CollisionDetection,
        )
            .chain()
            .run_if(in_state(GameState::InGame)),
    )
    .add_systems(
        Update,
        apply_deferred
            .after(InGameSet::DespawnEntities)
            .before(InGameSet::UserInput),
    );
}
