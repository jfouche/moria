use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{despawn_all, in_game::player::Player, ui::ProgressBar, GameState};

use super::character::Life;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudCompass;

#[derive(Component)]
struct HudCompassText;

#[derive(Component)]
struct HudFps;

#[derive(Component)]
struct HudFpsText;

#[derive(Component)]
struct HudAim;

#[derive(Component)]
struct HudLife;

const BGCOLOR: Color = Color::rgba(0.9, 0.9, 0.9, 0.3);

pub fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(
            OnEnter(GameState::Game),
            (
                spawn_hud,
                (spawn_fps, spawn_compass, spawn_aim, spawn_life).after(spawn_hud),
            ),
        )
        .add_systems(
            Update,
            (update_fps, update_compass, update_life).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), (despawn_all::<Hud>,));
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Hud,
        Name::new("Hud"),
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_fps(
    mut commands: Commands,
    hud: Query<Entity, With<Hud>>,
    asset_server: Res<AssetServer>,
) {
    let fps = commands
        .spawn((
            Name::new("HudFps"),
            HudFps,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    width: Val::Px(90.0),
                    height: Val::Px(30.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: BGCOLOR.into(),
                ..Default::default()
            },
        ))
        .with_children(|cmds| {
            const FONT_SIZE: f32 = 15.0;
            cmds.spawn((
                HudFpsText,
                TextBundle::from_sections([
                    TextSection::new(
                        "FPS: ",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_SIZE,
                            ..default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: FONT_SIZE,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        ..default()
                    }),
                ]),
            ));
        })
        .id();
    let hud = hud.get_single().expect("Hud");
    commands.entity(hud).push_children(&[fps]);
}

fn spawn_compass(
    mut commands: Commands,
    hud: Query<Entity, With<Hud>>,
    asset_server: Res<AssetServer>,
) {
    let compass = commands
        .spawn((
            Name::new("HudCompass"),
            HudCompass,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    width: Val::Px(90.0),
                    height: Val::Px(30.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: BGCOLOR.into(),
                ..Default::default()
            },
        ))
        .with_children(|cmds| {
            const FONT_SIZE: f32 = 15.0;
            cmds.spawn((
                HudCompassText,
                TextBundle::from_sections([
                    TextSection::new(
                        "Compass: ",
                        TextStyle {
                            // This font is loaded and will be used instead of the default font.
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_SIZE,
                            ..default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: FONT_SIZE,
                        color: Color::GOLD,
                        // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                        ..default()
                    }),
                ]),
            ));
        })
        .id();
    let hud = hud.get_single().expect("Hud");
    commands.entity(hud).push_children(&[compass]);
}

fn spawn_aim(
    mut commands: Commands,
    hud: Query<Entity, With<Hud>>,
    asset_server: Res<AssetServer>,
) {
    let aim = commands
        .spawn((
            Name::new("HudAim"),
            HudAim,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("aim.png")),
                ..default()
            });
        })
        .id();
    let hud = hud.get_single().expect("Hud");
    commands.entity(hud).push_children(&[aim]);
}

fn spawn_life(mut commands: Commands, hud: Query<Entity, With<Hud>>) {
    let life = commands
        .spawn((
            HudLife,
            Name::new("HudLife"),
            ProgressBar::new(0.0, 100.0, 60.0).with_colors(Color::BLACK, Color::RED),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(25.0),
                    left: Val::Auto,
                    right: Val::Auto,
                    width: Val::Percent(60.0),
                    height: Val::Px(40.0),
                    margin: UiRect::horizontal(Val::Auto),
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    let hud = hud.get_single().expect("Hud");
    commands.entity(hud).push_children(&[life]);
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<HudFpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.1}");
            }
        }
    }
}

fn update_compass(
    transform: Query<&Transform, With<Player>>,
    mut query: Query<&mut Text, With<HudCompassText>>,
) {
    for mut text in &mut query {
        let transform = transform.get_single().expect("Can't get Player");
        let forward = transform.forward();
        let angle = 180.0 - forward.x.atan2(forward.z).to_degrees();
        text.sections[1].value = format!("{angle:.0}");
    }
}

fn update_life(
    life: Query<&Life, With<Player>>,
    mut progressbars: Query<&mut ProgressBar, With<HudLife>>,
) {
    let &life = life.get_single().expect("Player");
    for mut progressbar in progressbars.iter_mut() {
        progressbar.set_value(*life as f32);
    }
}
