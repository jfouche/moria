use crate::{
    components::*,
    schedule::{InGameLoadingSet, InGameSet},
};
use bevy::prelude::*;

const MINIMAP_ATLAS_FILENAME: &str = "textures/minimap_atlas.png";
const MINIMAP_PLAYER_FILENAME: &str = "minimap_player.png";

#[derive(Component)]
struct Minimap;

#[derive(Component)]
struct MMPlayer;

#[derive(Component, Reflect)]
struct RoomComponent {
    pos: Position,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum MinimapState {
    Hide,
    Show,
}

const GRID_SIZE: UVec2 = UVec2::new(32, 32);

trait IntoGridPlacement {
    /// .0 : col
    ///
    /// .1 : row
    fn to_grid_pos(&self, pos: &Position) -> (GridPlacement, GridPlacement);
}

impl IntoGridPlacement for Maze {
    fn to_grid_pos(&self, pos: &Position) -> (GridPlacement, GridPlacement) {
        let grid_row = self.height() - pos.1 + 1;
        let grid_col = pos.0 + 1;
        (
            GridPlacement::start(grid_col as i16).set_span(1),
            GridPlacement::start(grid_row as i16).set_span(1),
        )
    }
}

trait ImgIndex {
    /// Get the index in the maze file
    ///  0:   ,  1: T,    2: R,    3: TR
    ///  4:  B,  5: TB,   6: RB,   7: TRB
    ///  8:  L,  9: TL,  10: RL,  11: TRL
    /// 12: BL, 13: TBL, 14: RBL, 15: TRBL
    fn img_index(&self) -> usize;
}

impl ImgIndex for Room {
    fn img_index(&self) -> usize {
        let mut index = 0;
        let borders = self.borders();
        if !borders.top {
            index += 1;
        }
        if !borders.right {
            index += 2;
        }
        if !borders.bottom {
            index += 4;
        }
        if !borders.left {
            index += 8;
        }
        index
    }
}

pub fn plugin(app: &mut App) {
    app.insert_state(MinimapState::Hide)
        .add_systems(Startup, load_minimap_atlas)
        .add_systems(
            OnEnter(InGameState::LoadLevel),
            (despawn_all::<Minimap>, init_minimap)
                .chain()
                .in_set(InGameLoadingSet::SpawnLevelEntities),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<Minimap>)
        .add_systems(OnEnter(MinimapState::Show), spawn_minimap)
        .add_systems(OnExit(MinimapState::Show), despawn_all::<Minimap>)
        .add_systems(
            Update,
            (
                toggle_minimap.in_set(InGameSet::UserInput),
                (show_player, update_visibility)
                    .run_if(minimap_visible)
                    .in_set(InGameSet::EntityUpdate),
            ),
        );
}

fn minimap_visible(
    in_game_state: Res<State<InGameState>>,
    minimap_state: Res<State<MinimapState>>,
) -> bool {
    *in_game_state == InGameState::Running && *minimap_state == MinimapState::Show
}

fn init_minimap(mut minimap_state: ResMut<NextState<MinimapState>>) {
    minimap_state.set(MinimapState::Hide);
}

fn load_minimap_atlas(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let _: Handle<Image> = asset_server.load(MINIMAP_ATLAS_FILENAME);
    let texture_atlas = TextureAtlasLayout::from_grid(GRID_SIZE, 4, 4, None, None);
    let _ = texture_atlases.add(texture_atlas);
}

fn toggle_minimap(
    state: Res<State<MinimapState>>,
    mut next_state: ResMut<NextState<MinimapState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        match state.get() {
            MinimapState::Hide => next_state.set(MinimapState::Show),
            MinimapState::Show => next_state.set(MinimapState::Hide),
        }
    }
}

fn spawn_minimap(
    mut commands: Commands,
    level: Res<Level>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("MINIMAP_PLAYER"),
        MMPlayer,
        ImageBundle {
            image: UiImage::new(asset_server.load(MINIMAP_PLAYER_FILENAME)),
            style: Style {
                margin: UiRect::all(Val::Auto),
                width: Val::Px(12.0),
                height: Val::Px(12.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
    ));

    commands
        .spawn((
            Minimap,
            Name::new("Minimap"),
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    position_type: PositionType::Absolute,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                background_color: Color::srgba(0.9, 0.9, 0.9, 0.3).into(),
                ..Default::default()
            },
        ))
        .with_children(|minimap_cmds| {
            let texture_handle: Handle<Image> = asset_server.load(MINIMAP_ATLAS_FILENAME);
            let texture_atlas = TextureAtlasLayout::from_grid(GRID_SIZE, 4, 4, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            for x in 0..level.maze.width() {
                for y in 00..level.maze.height() {
                    let pos = Position(x, y);
                    if let Some(room) = level.maze.get_room(&pos) {
                        let (grid_column, grid_row) = level.maze.to_grid_pos(&pos);
                        let visibiliy = if room.visited() {
                            Visibility::Visible
                        } else {
                            Visibility::Hidden
                        };
                        minimap_cmds.spawn((
                            ImageBundle {
                                image: UiImage::new(texture_handle.clone()),
                                style: Style {
                                    width: Val::Px(GRID_SIZE.x as f32),
                                    height: Val::Px(GRID_SIZE.y as f32),
                                    grid_row,
                                    grid_column,
                                    ..default()
                                },
                                visibility: visibiliy,
                                ..Default::default()
                            },
                            TextureAtlas {
                                layout: texture_atlas_handle.clone(),
                                index: room.img_index(),
                            },
                            RoomComponent { pos },
                        ));
                    }
                }
            }
        });
}

fn show_player(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    rooms: Query<(Entity, &RoomComponent)>,
    mut mm_player: Query<(Entity, &mut Visibility), With<MMPlayer>>,
) {
    let player_pos: WorldPosition = player
        .get_single()
        .expect("Can't get Player")
        .translation
        .into();
    let (mm_player_entity, mut mm_player_visibility) =
        mm_player.get_single_mut().expect("Can't get MMPlayer");
    *mm_player_visibility = Visibility::Visible;

    rooms
        .iter()
        .filter_map(|(e, r)| (r.pos == *player_pos).then_some(e))
        .for_each(|room_entity| {
            commands.entity(room_entity).add_child(mm_player_entity);
        });
}

fn update_visibility(mut rooms: Query<(&RoomComponent, &mut Visibility)>, level: Res<Level>) {
    for (room_comp, mut visibility) in rooms.iter_mut() {
        if let Some(room) = level.maze.get_room(&room_comp.pos) {
            if room.visited() {
                *visibility = Visibility::Visible;
            }
        }
    }
}
