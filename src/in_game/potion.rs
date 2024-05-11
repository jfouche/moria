use crate::components::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::InGame), spawn_potions)
        .add_systems(OnExit(GameState::InGame), despawn_all::<Potion>)
        .add_systems(Update, player_take_potion.run_if(game_is_running));
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = PotionAssets::load(&asset_server);
    commands.insert_resource(assets);
}

fn spawn_potions(mut commands: Commands, assets: Res<PotionAssets>) {
    let pos = Position(1, 0);
    commands.spawn(PotionBundle::new().at(pos).with_assets(&assets));
}

fn player_take_potion(
    mut events: EventReader<CollisionEvent>,
    potions: Query<Entity, With<Potion>>,
    mut player: Query<(Entity, &mut Life), With<Player>>,
) {
    let (player_entity, mut player_life) = player.get_single_mut().expect("Player");
    events
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            let take_potion = if e1 == player_entity {
                potions.get(e2).is_ok()
            } else if e2 == player_entity {
                potions.get(e1).is_ok()
            } else {
                false
            };
            if take_potion {
                player_life.add(30);
            }
        });
}
