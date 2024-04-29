use super::Player;
use crate::{
    config::MazeConfig,
    core::{IntoWorldPosition, Maze, MazeBuilder, Position, WorldPosition},
    despawn_all, GameState,
};
use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::Game), (spawn_maze,))
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
    const HEIGHT: f32 = 1.6;

    const COLLIDER_WIDTH: f32 = 0.02;

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

#[derive(Resource)]
struct MazeAssets {
    floor_material: Handle<StandardMaterial>,
    floor_mesh: Handle<Mesh>,
    ceiling_material: Handle<StandardMaterial>,
    ceiling_mesh: Handle<Mesh>,
    wall_material: Handle<StandardMaterial>,
    top_wall_mesh: Handle<Mesh>,
    bottom_wall_mesh: Handle<Mesh>,
    left_wall_mesh: Handle<Mesh>,
    right_wall_mesh: Handle<Mesh>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // load floor textures and materials
    let floor_texture_handle = asset_server.load("textures/Asset 6.png");
    let floor_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture_handle.clone()),
        ..default()
    });

    let floor_mesh_handle = meshes.add(
        Plane3d::new(Vec3::Y)
            .mesh()
            .size(WorldPosition::ROOM_WIDTH, WorldPosition::ROOM_WIDTH),
    );

    // load ceiling textures and materials
    let ceiling_texture_handle = asset_server.load("textures/Asset 17.png");
    let ceiling_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(ceiling_texture_handle.clone()),
        ..default()
    });

    let ceiling_mesh_handle = meshes.add(
        Plane3d::new(Vec3::NEG_Y)
            .mesh()
            .size(WorldPosition::ROOM_WIDTH, WorldPosition::ROOM_WIDTH),
    );

    // load wall textures and materials
    let wall_texture_handle = asset_server.load("textures/Asset 1.png");
    let wall_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(wall_texture_handle.clone()),
        ..default()
    });

    let maze_assets = MazeAssets {
        floor_material: floor_material_handle,
        floor_mesh: floor_mesh_handle,
        ceiling_material: ceiling_material_handle,
        ceiling_mesh: ceiling_mesh_handle,
        wall_material: wall_material_handle,
        top_wall_mesh: meshes.add(Wall::Top.mesh()),
        bottom_wall_mesh: meshes.add(Wall::Bottom.mesh()),
        left_wall_mesh: meshes.add(Wall::Left.mesh()),
        right_wall_mesh: meshes.add(Wall::Right.mesh()),
    };
    commands.insert_resource(maze_assets);
}

fn spawn_wall(commands: &mut Commands, wall: Wall, pos: &Position, assets: &MazeAssets) -> Entity {
    let mesh = match wall {
        Wall::Top => assets.top_wall_mesh.clone(),
        Wall::Bottom => assets.bottom_wall_mesh.clone(),
        Wall::Left => assets.left_wall_mesh.clone(),
        Wall::Right => assets.right_wall_mesh.clone(),
    };
    commands
        .spawn((
            Name::new(format!("Wall::{wall:?}-{pos}")),
            PbrBundle {
                mesh,
                material: assets.wall_material.clone(),
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
        })
        .id()
}

fn spawn_maze(mut commands: Commands, assets: Res<MazeAssets>, config: Res<MazeConfig>) {
    // create the maze
    let maze = MazeBuilder::new(config.cols, config.rows).create_maze();

    // Spawn Maze
    let maze_id = commands
        .spawn((Name::new("MAZE"), MazeComponent, SpatialBundle::default()))
        .id();

    // a vec to store children
    let mut children = vec![];

    maze.iter().for_each(|(room, pos)| {
        // Spawn floor
        let floor_id = commands
            .spawn((
                Name::new(format!("Floor {pos}")),
                PbrBundle {
                    mesh: assets.floor_mesh.clone(),
                    material: assets.floor_material.clone(),
                    transform: Transform::from_translation(pos.to_world().translation()),
                    ..default()
                },
            ))
            .id();
        children.push(floor_id);

        // Spawn ceiling
        let ceiling_id = commands
            .spawn((
                Name::new(format!("Ceiling {pos}")),
                PbrBundle {
                    mesh: assets.ceiling_mesh.clone(),
                    material: assets.ceiling_material.clone(),
                    transform: Transform::from_translation(
                        pos.to_world().translation_with_y(Wall::HEIGHT),
                    ),
                    ..default()
                },
            ))
            .id();
        children.push(ceiling_id);

        // Spawn walls
        if room.borders().top {
            let wall_id = spawn_wall(&mut commands, Wall::Top, &pos, &assets);
            children.push(wall_id);
        }

        if room.borders().left {
            let wall_id = spawn_wall(&mut commands, Wall::Left, &pos, &assets);
            children.push(wall_id);
        }

        if room.borders().bottom {
            let wall_id = spawn_wall(&mut commands, Wall::Bottom, &pos, &assets);
            children.push(wall_id);
        }

        if room.borders().right {
            let wall_id = spawn_wall(&mut commands, Wall::Right, &pos, &assets);
            children.push(wall_id);
        }
    });

    // Organize children
    commands.entity(maze_id).push_children(&children);

    // insert maze as resource
    commands.insert_resource(maze);
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