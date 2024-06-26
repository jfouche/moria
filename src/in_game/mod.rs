mod audio;
mod bullet_collision;
mod camera;
mod end_level;
mod enemy;
mod hud;
mod item;
mod level;
mod maze;
mod minimap;
mod player;
mod weapon;

use crate::components::*;
use crate::cursor::*;
use crate::schedule::InGameSet;
use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct InGamePlugins;

impl PluginGroup for InGamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(level::plugin)
            .add(maze::plugin)
            .add(player::plugin)
            .add(minimap::plugin)
            .add(hud::plugin)
            .add(weapon::plugin)
            .add(camera::plugin)
            .add(enemy::plugin)
            .add(bullet_collision::plugin)
            .add(audio::plugin)
            .add(item::plugin)
            .add(end_level::plugin)
            .add(in_game_plugin)
    }
}

fn in_game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), set_background)
        .add_systems(OnExit(GameState::InGame), ungrab_cursor)
        .add_systems(OnEnter(InGameState::Running), (grab_cursor, start_physics))
        .add_systems(OnExit(InGameState::Running), (ungrab_cursor, stop_physics))
        .add_systems(Update, switch_to_pause.in_set(InGameSet::UserInput))
        .add_systems(
            Update,
            despawn_if_too_old.in_set(InGameSet::DespawnEntities),
        );
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::BLACK));
}

fn switch_to_pause(mut state: ResMut<NextState<InGameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        state.set(InGameState::Pause);
    }
}

fn start_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = true;
}

fn stop_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = false;
}

/// Filter CollisionEvent::Started events
fn start_event_filter(event: &CollisionEvent) -> Option<(&Entity, &Entity)> {
    match event {
        CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
        _ => None,
    }
}

/// QueryEither
///
/// Example:
/// ```
/// query1.iter().filter_map(query.get_either(e1, e2)).map(|(data, e1, e2)|{})
/// ```
trait QueryEither<'w, D>
where
    D: QueryData<ReadOnly = D>,
{
    /// get either `e1` or `e2`, returning a `([QueryData], [Entity from query], [other Entity])`
    fn get_either(
        &'w self,
        e1: Entity,
        e2: Entity,
    ) -> Option<(<D as WorldQuery>::Item<'w>, Entity, Entity)>;
}

impl<'w, D, F> QueryEither<'w, D> for Query<'w, '_, D, F>
where
    D: QueryData<ReadOnly = D>,
    F: QueryFilter,
{
    fn get_either(
        &'w self,
        e1: Entity,
        e2: Entity,
    ) -> Option<(<D as WorldQuery>::Item<'w>, Entity, Entity)> {
        self.get(e1)
            .map(|data| (data, e1, e2))
            .or(self.get(e2).map(|data| (data, e2, e1)))
            .ok()
    }
}

/// The [EqEither] trait allow to check if self is equal to either
/// one value or another
pub trait EqEither {
    fn eq_either(&self, v1: Self, v2: Self) -> bool;
}

impl<T> EqEither for T
where
    T: Copy + PartialEq,
{
    fn eq_either(&self, v1: Self, v2: Self) -> bool {
        self == &v1 || self == &v2
    }
}
