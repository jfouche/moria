use crate::{
    components::*,
    schedule::{InGameLoadingSet, InGameSet},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets)
        .add_systems(
            OnEnter(GameState::InGame),
            spawn_maze.in_set(InGameLoadingSet::SpawnLevelEntities),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<MazeComponent>)
        .add_systems(Update, add_light.in_set(InGameSet::EntityUpdate));
}

#[derive(Component)]
struct MazeComponent;

#[derive(Component)]
struct RoomLight;

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

impl MazeAssets {
    fn mesh(&self, wall: Wall) -> Handle<Mesh> {
        match wall {
            Wall::Top => self.top_wall_mesh.clone(),
            Wall::Bottom => self.bottom_wall_mesh.clone(),
            Wall::Left => self.left_wall_mesh.clone(),
            Wall::Right => self.right_wall_mesh.clone(),
        }
    }
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

    let floor_mesh_handle = meshes.add(Plane3d::new(Vec3::Y).mesh().size(Wall::WIDTH, Wall::WIDTH));

    // load ceiling textures and materials
    let ceiling_texture_handle = asset_server.load("textures/Asset 17.png");
    let ceiling_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(ceiling_texture_handle.clone()),
        ..default()
    });

    let ceiling_mesh_handle = meshes.add(
        Plane3d::new(Vec3::NEG_Y)
            .mesh()
            .size(Wall::WIDTH, Wall::WIDTH),
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

fn spawn_wall(commands: &mut Commands, wall: Wall, pos: Position, assets: &MazeAssets) -> Entity {
    let mesh = assets.mesh(wall);
    let material = assets.wall_material.clone();
    let wall_pos = WallPosition { wall, pos };
    commands
        .spawn(WallBundle::new(wall_pos).with_pbr(mesh, material))
        .with_children(|c| {
            c.spawn(WallColliderBundle::new(wall_pos));
        })
        .id()
}

fn spawn_maze(mut commands: Commands, assets: Res<MazeAssets>, level: Res<Level>) {
    let maze = level.maze();

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

        // spawn floor collider
        let floor_collider_id = commands
            .spawn((
                Name::new("Floor collider"),
                RigidBody::Fixed,
                Collider::halfspace(Vec3::Y).unwrap(),
            ))
            .id();
        children.push(floor_collider_id);

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
            let wall_id = spawn_wall(&mut commands, Wall::Top, pos, &assets);
            children.push(wall_id);
        }

        if room.borders().left {
            let wall_id = spawn_wall(&mut commands, Wall::Left, pos, &assets);
            children.push(wall_id);
        }

        if room.borders().bottom {
            let wall_id = spawn_wall(&mut commands, Wall::Bottom, pos, &assets);
            children.push(wall_id);
        }

        if room.borders().right {
            let wall_id = spawn_wall(&mut commands, Wall::Right, pos, &assets);
            children.push(wall_id);
        }
    });

    // Organize children
    commands.entity(maze_id).push_children(&children);
}

fn add_light(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut level: ResMut<Level>,
    maze_components: Query<Entity, With<MazeComponent>>,
) {
    let player_transform = player.get_single().expect("Player");
    let maze_entity = maze_components.get_single().expect("MazeComponent");
    let player_pos: WorldPosition = player_transform.translation.into();
    let maze = level.maze_mut();
    if let Some(room) = maze.get_room(&player_pos) {
        if !room.visited() {
            maze.visit(&player_pos);
            // TODO: RoomLightBundle
            let light_entity = commands
                .spawn((
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
                ))
                .id();
            commands.entity(maze_entity).add_child(light_entity);
        }
    }
}
