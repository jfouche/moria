use crate::{
    config::MazeConfig,
    core::{Maze, MazeBuilder, Position},
    despawn_all, GameState,
};
use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

const WALL_HEIGHT: f32 = 1.0;

const WALL_COLLIDER_WIDTH: f32 = 0.03;

/// Width on X and Z
const ROOM_WIDTH: f32 = 2.0;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Game),
        (init_maze, spawn_maze.after(init_maze)),
    )
    .add_systems(OnExit(GameState::Game), despawn_all::<MazeComponent>);
}

#[derive(Component, Default)]
struct MazeComponent;

#[derive(Copy, Clone, Debug)]
enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

impl Wall {
    fn mesh(&self) -> impl Into<Mesh> {
        let normal = match self {
            Wall::Top => Vec3::Z,
            Wall::Bottom => Vec3::NEG_Z,
            Wall::Left => Vec3::X,
            Wall::Right => Vec3::NEG_X,
        };
        let (w, h) = match self {
            Wall::Top | Wall::Bottom => (ROOM_WIDTH, WALL_HEIGHT),
            Wall::Left | Wall::Right => (WALL_HEIGHT, ROOM_WIDTH),
        };
        Plane3d::new(normal).mesh().size(w, h)
    }

    fn transform(&self, pos: &Position) -> Transform {
        let world_pos = pos.world_pos();
        let hh = WALL_HEIGHT / 2.;
        let hw = ROOM_WIDTH / 2.;

        let translation = match self {
            Wall::Top => world_pos + Vec3::new(0., hh, -hw),
            Wall::Bottom => world_pos + Vec3::new(0., hh, hw),
            Wall::Left => world_pos + Vec3::new(-hw, hh, 0.),
            Wall::Right => world_pos + Vec3::new(hw, hh, 0.),
        };
        Transform::from_translation(translation)
    }

    fn collider(&self) -> Collider {
        const HRW: f32 = ROOM_WIDTH / 2.0;
        let (hx, hz) = match self {
            Wall::Top | Wall::Bottom => (HRW, WALL_COLLIDER_WIDTH),
            Wall::Left | Wall::Right => (WALL_COLLIDER_WIDTH, HRW),
        };
        Collider::cuboid(hx, WALL_HEIGHT / 2., hz)
    }

    fn collider_transform(&self) -> Transform {
        let (x, z) = match self {
            Wall::Top => (0.0, -WALL_COLLIDER_WIDTH),
            Wall::Bottom => (0.0, WALL_COLLIDER_WIDTH),
            Wall::Left => (WALL_COLLIDER_WIDTH, 0.0),
            Wall::Right => (-WALL_COLLIDER_WIDTH, 0.0),
        };
        Transform::from_xyz(x, 0.0, z)
    }
}

fn init_maze(mut commands: Commands, config: Res<MazeConfig>) {
    let maze = MazeBuilder::new(config.cols, config.rows).create_maze();
    commands.insert_resource(maze);
}

fn spawn_maze(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze: Res<Maze>,
) {
    info!("maze_spawn(...)");
    let texture_handle = asset_server.load("textures/Asset 1.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        perceptual_roughness: 0.8,
        ..default()
    });

    commands
        .spawn((Name::new("MAZE"), MazeComponent, SpatialBundle::default()))
        .with_children(|maze_cmd| {
            // TODO : use Iterator
            for x in 0..maze.width() {
                for y in 00..maze.height() {
                    let pos = Position(x, y);
                    if let Some(room) = maze.get_room(&pos) {
                        if room.borders().top {
                            let wall = Wall::Top;
                            let name = format!("Wall::{wall:?}-{pos}");
                            maze_cmd
                                .spawn((
                                    Name::new(name),
                                    PbrBundle {
                                        mesh: meshes.add(wall.mesh()),
                                        material: material_handle.clone(),
                                        transform: wall.transform(&pos),
                                        ..default()
                                    },
                                ))
                                .with_children(|c| {
                                    c.spawn((
                                        RigidBody::Fixed,
                                        wall.collider(),
                                        SpatialBundle {
                                            transform: wall.collider_transform(),
                                            ..default()
                                        },
                                    ));
                                });
                        }

                        if room.borders().left {
                            let wall = Wall::Left;
                            let name = format!("Wall::{wall:?}-{pos}");
                            maze_cmd
                                .spawn((
                                    Name::new(name),
                                    PbrBundle {
                                        mesh: meshes.add(wall.mesh()),
                                        material: material_handle.clone(),
                                        transform: wall.transform(&pos),
                                        ..default()
                                    },
                                ))
                                .with_children(|c| {
                                    c.spawn((
                                        RigidBody::Fixed,
                                        wall.collider(),
                                        SpatialBundle {
                                            transform: wall.collider_transform(),
                                            ..default()
                                        },
                                    ));
                                });
                        }

                        if room.borders().bottom {
                            let wall = Wall::Bottom;
                            let name = format!("Wall::{wall:?}-{pos}");
                            maze_cmd
                                .spawn((
                                    Name::new(name),
                                    PbrBundle {
                                        mesh: meshes.add(wall.mesh()),
                                        material: material_handle.clone(),
                                        transform: wall.transform(&pos),
                                        ..default()
                                    },
                                ))
                                .with_children(|c| {
                                    c.spawn((
                                        RigidBody::Fixed,
                                        wall.collider(),
                                        SpatialBundle {
                                            transform: wall.collider_transform(),
                                            ..default()
                                        },
                                    ));
                                });
                        }

                        if room.borders().right {
                            let wall = Wall::Right;
                            let name = format!("Wall::{wall:?}-{pos}");
                            maze_cmd
                                .spawn((
                                    Name::new(name),
                                    PbrBundle {
                                        mesh: meshes.add(wall.mesh()),
                                        material: material_handle.clone(),
                                        transform: wall.transform(&pos),
                                        ..default()
                                    },
                                ))
                                .with_children(|c| {
                                    c.spawn((
                                        RigidBody::Fixed,
                                        wall.collider(),
                                        SpatialBundle {
                                            transform: wall.collider_transform(),
                                            ..default()
                                        },
                                    ));
                                });
                        }
                    }
                }
            }
        });
}
