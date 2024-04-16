use bevy::prelude::*;

use crate::{
    core::{Maze, Position, Room},
    GameState,
};

pub const MINIMAP_ATLAS_FILENAME: &str = "textures/minimap_atlas.png";

#[derive(Component)]
pub struct Minimap;

#[derive(Component, Reflect)]
struct RoomComponent {
    room: Room,
    pos: Position,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum MinimapState {
    Hide,
    Show,
}

const GRID_SIZE: Vec2 = Vec2::new(32.0, 32.0);

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
        .add_systems(Update, toggle_minimap.run_if(in_state(GameState::Game)))
        .add_systems(OnEnter(MinimapState::Show), spawn_minimap)
        .add_systems(OnExit(MinimapState::Show), despawn_minimap)
        .add_systems(OnExit(GameState::Game), despawn_minimap);
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
    maze: Res<Maze>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn((
            Minimap,
            Name::new("Minimap"),
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    position_type: PositionType::Absolute,
                    margin: UiRect::all(Val::Auto),
                    // top: Val::Px(50.0),
                    // bottom: Val::Px(50.),
                    // left: Val::Px(20.0),
                    // right: Val::Px(20.0),
                    // flex_direction: FlexDirection::Column,
                    // justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: Color::rgba(0.9, 0.9, 0.9, 0.3).into(),
                ..Default::default()
            },
        ))
        .with_children(|minimap_cmds| {
            let texture_handle: Handle<Image> = asset_server.load(MINIMAP_ATLAS_FILENAME);
            let texture_atlas = TextureAtlasLayout::from_grid(GRID_SIZE, 4, 4, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            for x in 0..maze.width() {
                for y in 00..maze.height() {
                    let pos = Position(x, y);
                    if let Some(room) = maze.get_room(&pos) {
                        let (grid_column, grid_row) = maze.to_grid_pos(&pos);
                        minimap_cmds
                            .spawn(AtlasImageBundle {
                                style: Style {
                                    width: Val::Px(GRID_SIZE.x),
                                    height: Val::Px(GRID_SIZE.y),
                                    grid_row,
                                    grid_column,
                                    ..default()
                                },
                                texture_atlas: TextureAtlas {
                                    layout: texture_atlas_handle.clone(),
                                    index: room.img_index(),
                                },
                                image: UiImage::new(texture_handle.clone()),
                                ..Default::default()
                            })
                            .insert(RoomComponent {
                                pos,
                                room: room.clone(),
                            });
                    }
                }
            }
        });
}

fn despawn_minimap(mut commands: Commands, minimap: Query<Entity, With<Minimap>>) {
    for entity in minimap.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
