use bevy::prelude::*;

use crate::{despawn_all, GameState};

pub fn plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app.add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, key_pressed.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_all::<OnSplashScreen>);
}

// Tag component used to tag entities added on the splash screen
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

fn key_pressed(mut game_state: ResMut<NextState<GameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.get_pressed().len() != 0 {
        game_state.set(GameState::Menu);
    }
}
