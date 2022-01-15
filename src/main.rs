use bevy::prelude::*;

const PLAYER: &str = "player.png";

const TIME_STEP: f32 = 1.0 / 20.;

// RESSOURCES
struct Materials {
    player: Handle<Image>,
}

// COMPONENTS
#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerSpeed(f32);

impl Default for PlayerSpeed {
    fn default() -> Self {
        PlayerSpeed(500.)
    }
}

// LR, TB, TRBL, .
// LT, RB, RT, RB
// LTR, TRB, RBL, BLT

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.74, 0.74, 0.24)))
        .insert_resource(WindowDescriptor {
            title: "Moria - Rust".to_string(),
            width: 640.,
            height: 480.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Resources
    commands.insert_resource(Materials {
        player: assets.load(PLAYER).into()
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: materials.player.clone(),
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerSpeed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>,
) {
    if let Ok((speed, mut transform, _)) = query.get_single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } 
        else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        }
        else {
            0.
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}
