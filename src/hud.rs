use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::player::Player;

#[derive(Component)]
struct CompassText;

#[derive(Component)]
struct FpsText;

const BGCOLOR: Color = Color::rgba(0.9, 0.9, 0.9, 0.3);

pub fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, (spawn_fps, spawn_compass))
        .add_systems(Update, (update_fps, update_compass));
}

fn spawn_fps(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("FPS"),
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
                FpsText,
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
        });
}

fn spawn_compass(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Compass"),
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
                CompassText,
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
        });
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
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
    mut query: Query<&mut Text, With<CompassText>>,
) {
    for mut text in &mut query {
        let transform = transform.get_single().expect("Can't get Player");
        let mut forward = *transform.forward();
        forward.y = 0.0;
        let angle = forward.angle_between(Vec3::NEG_Z).to_degrees();
        text.sections[1].value = format!("{angle:.0}");
    }
}
