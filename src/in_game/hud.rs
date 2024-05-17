use crate::{components::*, math::*, ui::*};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudCompass;

#[derive(Component)]
struct HudFps;

#[derive(Component)]
struct HudFpsText;

#[derive(Component)]
struct HudAim;

#[derive(Component)]
struct HudLife;

#[derive(Resource)]
struct HudAssets {
    font: Handle<Font>,
    aim: Handle<Image>,
    compass: Handle<Image>,
}

pub fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, load_assets)
        .add_systems(
            OnEnter(GameState::InGame),
            (spawn_fps, spawn_compass, spawn_aim, spawn_life),
        )
        .add_systems(OnExit(GameState::InGame), (despawn_all::<Hud>,))
        .add_systems(
            OnEnter(InGameState::Running),
            set_visibility::<HudAim>(Visibility::Inherited),
        )
        .add_systems(
            OnExit(InGameState::Running),
            set_visibility::<HudAim>(Visibility::Hidden),
        )
        .add_systems(
            Update,
            (update_fps, update_compass, update_life).run_if(game_is_running),
        );
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let aim = asset_server.load("aim.png");
    let compass = asset_server.load("compass.png");
    let assets = HudAssets { font, aim, compass };
    commands.insert_resource(assets);
}

fn spawn_fps(mut commands: Commands, assets: Res<HudAssets>) {
    commands
        .spawn((
            Name::new("HudFps"),
            Hud,
            HudFps,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    right: Val::Px(0.0),
                    width: Val::Px(90.0),
                    height: Val::Px(30.),
                    ..centered_style()
                },
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.3).into(),
                ..Default::default()
            },
        ))
        .with_children(|cmds| {
            const FONT_SIZE: f32 = 15.0;
            let text_style = TextStyle {
                font: assets.font.clone(),
                font_size: FONT_SIZE,
                ..default()
            };
            cmds.spawn((
                HudFpsText,
                TextBundle::from_sections([
                    TextSection::new("FPS: ", text_style.clone()),
                    TextSection::from_style(text_style),
                ]),
            ));
        });
}

fn spawn_compass(mut commands: Commands, assets: Res<HudAssets>) {
    commands.spawn((
        Name::new("HudCompass"),
        HudCompass,
        Hud,
        ImageBundle {
            image: UiImage::new(assets.compass.clone()),
            background_color: Color::rgba(0.4, 0.4, 0.4, 0.7).into(),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                ..default()
            },
            transform: Transform::default().with_scale(Vec3::splat(0.6)),
            ..default()
        },
    ));
}

fn spawn_aim(mut commands: Commands, assets: Res<HudAssets>) {
    commands
        .spawn((Name::new("HudAim"), HudAim, Hud, centered()))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(assets.aim.clone()),
                ..default()
            });
        });
}

fn spawn_life(mut commands: Commands) {
    commands.spawn((
        HudLife,
        Hud,
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
    ));
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
    mut compass: Query<&mut Transform, (With<HudCompass>, Without<Player>)>,
) {
    let forward = transform.get_single().expect("Can't get Player").forward();
    if let Ok(mut transform) = compass.get_single_mut() {
        let angle = forward.horizontal().signed_angle_with(Vec3::NEG_Z.into());
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn update_life(
    life: Query<&Life, With<Player>>,
    mut progressbars: Query<&mut ProgressBar, With<HudLife>>,
) {
    let &life = life.get_single().expect("Player");
    for mut progressbar in progressbars.iter_mut() {
        progressbar.set_value(life.get() as f32);
    }
}
