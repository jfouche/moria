use crate::{components::*, cursor::*, math::Angle, schedule::InGameSet};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, apply_config)
        .add_plugins((
            WorldInspectorPlugin::new().run_if(is_debug_enabled),
            RapierDebugRenderPlugin::default().disabled(),
        ))
        .add_systems(Update, toggle_grab.in_set(InGameSet::UserInput))
        .add_systems(
            Update,
            (
                // debug_player_view.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_secs(1))),
                display_collision_events,
            )
            .after(InGameSet::EntityUpdate),
        )
        .add_systems(Update, show_axes)
        // States
        .add_systems(Update, (state_transition::<GameState>, state_transition::<InGameState>))
        // .add_systems(OnEnter(GameState::InGame), display_states)
        // .add_systems(OnExit(GameState::InGame), display_states)
        // .add_systems(OnEnter(InGameState::Running), display_states)
        // .add_systems(OnExit(InGameState::Pause), display_states)
        // END
        ;
}

fn is_debug_enabled(config: Res<GameConfig>) -> bool {
    config.debug
}

fn apply_config(config: Res<GameConfig>, mut rapier: ResMut<DebugRenderContext>) {
    rapier.enabled = config.debug;
}

fn toggle_grab(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(window) = primary_window.get_single_mut() {
        if keys.just_pressed(KeyCode::KeyG) {
            match window.cursor.grab_mode {
                CursorGrabMode::None => {
                    set_grab_cursor(window, true);
                }
                _ => {
                    set_grab_cursor(window, false);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn debug_player_view(transform: Query<&Transform, With<Player>>) {
    if let Ok(transform) = transform.get_single() {
        let translation = transform.translation;
        let pos: WorldPosition = translation.into();
        let mut forward = *transform.forward();
        forward.y = 0.0;
        let angle = forward.angle_between(Vec3::Z).angle().to_degrees();
        let forward = forward.xz();
        debug!(
            "Player translation: {translation}, pos: {pos:?}, forward: {forward:?}, angle: {angle}"
        );
    }
}

#[allow(dead_code)]
fn show_axes(mut gizmos: Gizmos, config: Res<GameConfig>) {
    if config.debug {
        gizmos.ray(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Color::RED);
        gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), Color::GREEN);
        gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0), Color::BLUE);
    }
}

fn display_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    names: Query<DebugName>,
) {
    for event in collision_events.read() {
        let (collision_type, &e1, &e2) = match event {
            CollisionEvent::Started(e1, e2, _) => ("Started", e1, e2),
            CollisionEvent::Stopped(e1, e2, _) => ("Stopped", e1, e2),
        };
        let name1 = names.get(e1);
        let name2 = names.get(e2);
        debug!("Received collision event: {collision_type}, {name1:?}, {name2:?}");
    }
}

#[allow(dead_code)]
fn display_states(game_state: Res<State<GameState>>, in_game_state: Res<State<InGameState>>) {
    info!(
        "GameState::{:?} - InGameState::{:?}",
        **game_state, **in_game_state
    );
}

fn state_transition<S: States>(mut events: EventReader<StateTransitionEvent<S>>) {
    for event in events.read() {
        let name = std::any::type_name::<S>();
        info!("{name} : {:?} => {:?}", event.before, event.after);
    }
}
