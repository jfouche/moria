use crate::components::*;
use crate::cursor::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Splash),
        (spawn_splash_screen, ungrab_cursor),
    )
    .add_systems(OnExit(GameState::Splash), despawn_all::<SplashScreen>)
    .add_systems(OnEnter(AssetsLoadingState::Loaded), display_continue)
    .add_systems(Update, goto_main_menu.run_if(assets_loaded));
}

fn assets_loaded(
    game_state: Res<State<GameState>>,
    load_state: Res<State<AssetsLoadingState>>,
) -> bool {
    *game_state == GameState::Splash && *load_state == AssetsLoadingState::Loaded
}

#[derive(Component)]
struct SplashScreen;

#[derive(Component)]
struct SplashScreenMessage;

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    let icon = asset_server.load("splash.png");
    // Display the logo
    commands
        .spawn((
            SplashScreen,
            Name::new("SplashScreen"),
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn((
                SplashScreenMessage,
                TextBundle::from_section(
                    "Loading",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
            ));
        });
}

fn display_continue(mut messages: Query<&mut Text, With<SplashScreenMessage>>) {
    for mut text in &mut messages {
        text.sections[0].value = "Press any key to continue".into();
    }
}

fn goto_main_menu(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if keys.get_pressed().len() != 0 {
        game_state.set(GameState::Menu);
    }
    if mouse.pressed(MouseButton::Left) {
        game_state.set(GameState::Menu);
    }
}
