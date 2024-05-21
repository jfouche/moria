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
    app.add_systems(OnEnter(GameState::InGame), (init_game, set_background))
        .add_systems(OnExit(GameState::InGame), (end_game, ungrab_cursor))
        .add_systems(OnEnter(InGameState::Running), (grab_cursor, start_physics))
        .add_systems(OnExit(InGameState::Running), (ungrab_cursor, stop_physics))
        .add_systems(Update, switch_to_pause.in_set(InGameSet::UserInput))
        .add_systems(
            Update,
            despawn_if_too_old.in_set(InGameSet::DespawnEntities),
        );
}

fn init_game(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Running);
}

fn end_game(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Disabled);
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

/// Filter a iterator with either `e1` or `e2`, returning a `([QueryData], [Entity from query], [other Entity])`
fn filter_either<'w, D, F>(
    query: &'w Query<'w, '_, D, F>,
    e1: Entity,
    e2: Entity,
) -> Option<(<D as WorldQuery>::Item<'w>, Entity, Entity)>
where
    D: QueryData<ReadOnly = D>,
    F: QueryFilter,
{
    query
        .get(e1)
        .map(|data| (data, e1, e2))
        .or(query.get(e2).map(|data| (data, e2, e1)))
        .ok()
}
