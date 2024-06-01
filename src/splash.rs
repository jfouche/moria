use crate::components::*;
use crate::cursor::*;
use crate::ui::fullscreen_style;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Splash),
        (spawn_splash_screen, ungrab_cursor),
    )
    .add_systems(OnExit(GameState::Splash), despawn_all::<SplashScreen>)
    .add_systems(Update, goto_main_menu.run_if(in_state(GameState::Splash)));
}

#[derive(Component)]
struct SplashScreen;

const BACKGROUND_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands
        .spawn((
            SplashScreen,
            Name::new("SplashScreen"),
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..fullscreen_style()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            let icon = asset_server.load("splash.png");
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn((TextBundle::from_section(
                "Press any key to continue",
                TextStyle {
                    font_size: 16.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),));
        });
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
