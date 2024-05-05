use crate::{
    config::GameConfig,
    core::WorldPosition,
    cursor::set_grab_cursor,
    in_game::Player,
    menu::{MainMenuState, PauseMenuState},
    GameState, InGameState,
};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::{
    pipeline::CollisionEvent,
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::{DebugRenderContext, RapierDebugRenderPlugin},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, apply_config)
        .add_systems(Update, toggle_camera_controls_system)
        .add_plugins((
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default().disabled(),
            // PanOrbitCameraPlugin
        ))
        .add_systems(Update, (toggle_grab).run_if(in_state(GameState::InGame)))
        .add_systems(
            Update,
            (
                // debug_player_view.run_if(on_timer(Duration::from_secs(1))),
                display_collision_events,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, show_axes)
        // States
        .add_systems(Update, (state_transition::<GameState>, state_transition::<InGameState>, state_transition::<MainMenuState>, state_transition::<InGameState>))
        .add_systems(OnEnter(GameState::InGame), display_states)
        .add_systems(OnExit(GameState::InGame), display_states)
        .add_systems(OnEnter(InGameState::Running), display_states)
        .add_systems(OnExit(InGameState::Pause), display_states)
        // END
        ;
}

fn apply_config(config: Res<GameConfig>, mut rapier: ResMut<DebugRenderContext>) {
    rapier.enabled = config.debug;
}

fn toggle_grab(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let window = primary_window.get_single_mut().expect("PrimaryWindow");
    if keys.just_pressed(KeyCode::KeyG) {
        match window.cursor.grab_mode {
            CursorGrabMode::None => {
                set_grab_cursor(window, true);
            }
            _ => {
                set_grab_cursor(window, true);
            }
        }
    }
}

fn toggle_camera_controls_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    if key_input.just_pressed(KeyCode::KeyT) {
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.enabled = !pan_orbit.enabled;
        }
    }
}

#[allow(dead_code)]
fn debug_player_view(transform: Query<&Transform, With<Player>>) {
    let transform = transform.get_single().expect("Player");
    let translation = transform.translation;
    let pos: WorldPosition = translation.into();
    let mut forward = *transform.forward();
    forward.y = 0.0;
    let angle = forward.angle_between(Vec3::Z).to_degrees();
    let forward = forward.xz();
    info!("Player translation: {translation}, pos: {pos:?}, forward: {forward:?}, angle: {angle}");
}

fn show_axes(mut gizmos: Gizmos, config: Res<GameConfig>) {
    if config.debug {
        gizmos.ray(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Color::RED);
        gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), Color::GREEN);
        gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0), Color::BLUE);
    }
}

fn display_collision_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }
}

fn display_states(
    game_state: Res<State<GameState>>,
    in_game_state: Res<State<InGameState>>,
    main_menu_state: Res<State<MainMenuState>>,
    pause_menu_state: Res<State<PauseMenuState>>,
) {
    info!(
        "GameState::{:?} - InGameState::{:?} - MainMenuState::{:?} - PauseMenuState::{:?}",
        **game_state, **in_game_state, **main_menu_state, **pause_menu_state
    );
}

fn state_transition<S: States>(mut events: EventReader<StateTransitionEvent<S>>) {
    for event in events.read() {
        let name = std::any::type_name::<S>();
        info!("{name} : {:?} => {:?}", event.before, event.after);
    }
}
