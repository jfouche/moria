use super::Player;
use crate::{
    config::MazeConfig,
    core::{IntoWorldPosition, Maze, MazeBuilder, Position, WorldPosition},
    despawn_all, GameState,
};
use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Game),
        (init_maze, (spawn_maze, spawn_ceiling).after(init_maze)),
    )
    .add_systems(Update, add_light.run_if(in_state(GameState::Game)))
    .add_systems(
        OnExit(GameState::Game),
        (
            despawn_all::<MazeComponent>,
            despawn_all::<RoomLight>,
            despawn_all::<Ceiling>,
        ),
    );
}

#[derive(Component)]
struct MazeComponent;

#[derive(Component)]
struct RoomLight;

#[derive(Component)]
struct Ceiling;

#[derive(Copy, Clone, Debug)]
enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

impl Wall {
    const HEIGHT: f32 = 1.0;

    const COLLIDER_WIDTH: f32 = 0.03;

    fn mesh(&self) -> impl Into<Mesh> {
        let normal = match self {
            Wall::Top => Vec3::Z,
            Wall::Bottom => Vec3::NEG_Z,
            Wall::Left => Vec3::X,
            Wall::Right => Vec3::NEG_X,
        };
        let (w, h) = match self {
            Wall::Top | Wall::Bottom => (WorldPosition::ROOM_WIDTH, Wall::HEIGHT),
            Wall::Left | Wall::Right => (Wall::HEIGHT, WorldPosition::ROOM_WIDTH),
        };
        Plane3d::new(normal).mesh().size(w, h)
    }

    fn transform(&self, pos: &Position) -> Transform {
        let translation = pos.to_world().translation();
        const HH: f32 = Wall::HEIGHT / 2.;
        const HW: f32 = WorldPosition::ROOM_WIDTH / 2.;

        let translation = match self {
            Wall::Top => translation + Vec3::new(0., HH, -HW),
            Wall::Bottom => translation + Vec3::new(0., HH, HW),
            Wall::Left => translation + Vec3::new(-HW, HH, 0.),
            Wall::Right => translation + Vec3::new(HW, HH, 0.),
        };
        Transform::from_translation(translation)
    }

    fn collider(&self) -> Collider {
        const HRW: f32 = WorldPosition::ROOM_WIDTH / 2.0;
        let (hx, hz) = match self {
            Wall::Top | Wall::Bottom => (HRW, Wall::COLLIDER_WIDTH),
            Wall::Left | Wall::Right => (Wall::COLLIDER_WIDTH, HRW),
        };
        Collider::cuboid(hx, Self::HEIGHT / 2., hz)
    }

    fn collider_transform(&self) -> Transform {
        let (x, z) = match self {
            Wall::Top => (0.0, -Wall::COLLIDER_WIDTH),
            Wall::Bottom => (0.0, Wall::COLLIDER_WIDTH),
            Wall::Left => (Wall::COLLIDER_WIDTH, 0.0),
            Wall::Right => (-Wall::COLLIDER_WIDTH, 0.0),
        };
        Transform::from_xyz(x, 0.0, z)
    }
}

struct WallSpawner<'w> {
    meshes: ResMut<'w, Assets<Mesh>>,
    material_handle: Handle<StandardMaterial>,
}

impl<'w> WallSpawner<'w> {
    fn new(meshes: ResMut<'w, Assets<Mesh>>, material_handle: Handle<StandardMaterial>) -> Self {
        Self {
            meshes,
            material_handle,
        }
    }

    fn spawn(&mut self, commands: &mut ChildBuilder, wall: Wall, pos: &Position) {
        let name = format!("Wall::{wall:?}-{pos}");
        commands
            .spawn((
                Name::new(name),
                PbrBundle {
                    mesh: self.meshes.add(wall.mesh()),
                    material: self.material_handle.clone(),
                    transform: wall.transform(pos),
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

fn init_maze(mut commands: Commands, config: Res<MazeConfig>) {
    let maze = MazeBuilder::new(config.cols, config.rows).create_maze();
    commands.insert_resource(maze);
}

fn spawn_maze(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze: Res<Maze>,
) {
    let texture_handle = asset_server.load("textures/Asset 1.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.8,
        ..default()
    });
    let mut wall_spawner = WallSpawner::new(meshes, material_handle);

    commands
        .spawn((Name::new("MAZE"), MazeComponent, SpatialBundle::default()))
        .with_children(|maze_cmd| {
            // TODO : use Iterator
            for x in 0..maze.width() {
                for y in 0..maze.height() {
                    let pos = Position(x, y);
                    if let Some(room) = maze.get_room(&pos) {
                        if room.borders().top {
                            wall_spawner.spawn(maze_cmd, Wall::Top, &pos);
                        }

                        if room.borders().left {
                            wall_spawner.spawn(maze_cmd, Wall::Left, &pos);
                        }

                        if room.borders().bottom {
                            wall_spawner.spawn(maze_cmd, Wall::Bottom, &pos);
                        }

                        if room.borders().right {
                            wall_spawner.spawn(maze_cmd, Wall::Right, &pos);
                        }
                    }
                }
            }
        });
}

fn spawn_ceiling(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("CEILING"),
        Ceiling,
        PbrBundle {
            // Use NEG_Y to show the ceiling face to the player
            mesh: meshes.add(Plane3d::new(Vec3::NEG_Y).mesh().size(50.0, 50.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                perceptual_roughness: 0.9,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, Wall::HEIGHT, 0.0),
            ..default()
        },
    ));
}

fn add_light(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut maze: ResMut<Maze>,
) {
    let player_transform = player.get_single().expect("Can't get Player");
    let player_pos: WorldPosition = player_transform.translation.into();
    if let Some(room) = maze.get_room(&player_pos) {
        if !room.visited() {
            maze.visit(&player_pos);
            commands.spawn((
                Name::new("ROOM_LIGHT"),
                RoomLight,
                SpotLightBundle {
                    spot_light: SpotLight {
                        intensity: 200_000.0,
                        outer_angle: 1.4,
                        ..default()
                    },
                    transform: Transform::from_translation(
                        player_pos.translation_with_y(Wall::HEIGHT),
                    )
                    .looking_at(player_pos.translation(), Vec3::Y),
                    ..default()
                },
            ));
        }
    }
}
