use crate::cursor::*;
use crate::components::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), (splash_setup, ungrab_cursor))
        .add_systems(Update, key_pressed.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_all::<OnSplashScreen>);
}

#[derive(Component)]
struct OnSplashScreen;

const BACKGROUND_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    let icon = asset_server.load("splash.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
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
        });
}

fn key_pressed(
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
