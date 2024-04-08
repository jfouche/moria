use bevy::prelude::*;
use rand::{
    prelude::{SliceRandom, ThreadRng},
    Rng,
};
use std::fmt;

pub(crate) const WALL_HEIGHT: f32 = 1.0;

/// Width on X and Z
pub(crate) const ROOM_WIDTH: f32 = 2.0;

/// .0 : x
///
/// .1 : y
#[derive(Clone, Copy, PartialEq, Eq, Debug, Reflect)]
pub struct Position(pub u32, pub u32);

impl Position {
    /// Get the square of the distance
    fn sqr_distance(&self, other: &Position) -> u32 {
        let dx = self.0 as i32 - other.0 as i32;
        let dy = self.1 as i32 - other.1 as i32;
        (dx * dx + dy * dy) as u32
    }

    pub fn world_pos(&self) -> Vec3 {
        Vec3 {
            x: self.0 as f32 * ROOM_WIDTH,
            y: 0.0,
            z: self.1 as f32 * -ROOM_WIDTH,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl From<Vec3> for Position {
    fn from(world_pos: Vec3) -> Self {
        Position(
            (world_pos.x / ROOM_WIDTH) as u32,
            (-world_pos.z / ROOM_WIDTH) as u32,
        )
    }
}

#[derive(Resource)]
pub struct Maze {
    width: u32,
    height: u32,
    rooms: Vec<Room>,
}

impl Maze {
    /// ```text
    /// ^ (0, h)     (w, h)
    /// |  
    /// | (0, 0)     (w, 0)
    /// + ----------------->
    /// ```
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Maze {
            width,
            height,
            rooms: vec![Room::new(); (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn room_index(&self, pos: &Position) -> usize {
        (pos.1 * self.width + pos.0) as usize
    }

    pub fn get_room(&self, pos: &Position) -> Option<&Room> {
        self.rooms.get(self.room_index(pos))
    }

    fn get_room_mut(&mut self, pos: &Position) -> Option<&mut Room> {
        let index = self.room_index(pos);
        self.rooms.get_mut(index)
    }

    fn visit(&mut self, pos: &Position) {
        if let Some(room) = self.get_room_mut(pos) {
            room.visit();
        }
    }

    pub(crate) fn left_position(&self, pos: &Position) -> Option<Position> {
        if pos.0 > 0 {
            Some(Position(pos.0 - 1, pos.1))
        } else {
            None
        }
    }

    pub(crate) fn right_position(&self, pos: &Position) -> Option<Position> {
        if pos.0 < self.width - 1 {
            Some(Position(pos.0 + 1, pos.1))
        } else {
            None
        }
    }

    pub(crate) fn up_position(&self, pos: &Position) -> Option<Position> {
        if pos.1 < self.height - 1 {
            Some(Position(pos.0, pos.1 + 1))
        } else {
            None
        }
    }

    pub(crate) fn down_position(&self, pos: &Position) -> Option<Position> {
        if pos.1 > 0 {
            Some(Position(pos.0, pos.1 - 1))
        } else {
            None
        }
    }

    fn clear(&mut self) {
        for room in &mut self.rooms {
            room.clear();
        }
    }

    // pub fn get_next_room(&self, pos: &Position, dir: Direction) -> Option<&Room> {
    //     let borders = &self.get_room(pos)?.borders;
    //     match dir {
    //         Direction::TOP if !borders.top => self.get_room(&Position(pos.0, pos.1 + 1)),
    //         Direction::RIGHT if !borders.right => self.get_room(&Position(pos.0 + 1, pos.1)),
    //         Direction::BOTTOM if !borders.bottom => self.get_room(&Position(pos.0, pos.1 - 1)),
    //         Direction::LEFT if !borders.left => self.get_room(&Position(pos.0 - 1, pos.1)),
    //         _ => None,
    //     }
    // }
}

/// Get the index in the maze file
/// ```text
///  0:   ,  1: T,    2: R,    3: TR
///  4:  B,  5: TB,   6: RB,   7: TRB
///  8:  L,  9: TL,  10: RL,  11: TRL
/// 12: BL, 13: TBL, 14: RBL, 15: TRBL
/// ```
pub(crate) fn borders_index(borders: &CellBorders) -> usize {
    let mut index = 0;
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

const CELL_DISPLAY: &[&[&str]] = &[
    &[
        // 0 :
        "   ", "   ", "   ",
    ],
    &[
        // 1 : T
        " █ ", " █ ", "   ",
    ],
    &[
        // 2 : R
        "   ", " ██", "   ",
    ],
    &[
        // 3 : TR
        " █ ", " ██", "   ",
    ],
    &[
        // 4 : B
        "   ", " █ ", " █ ",
    ],
    &[
        // 5 : TB
        " █ ", " █ ", " █ ",
    ],
    &[
        // 6 : RB
        "   ", " ██", " █ ",
    ],
    &[
        // 7 : TRB
        " █ ", " ██", " █ ",
    ],
    &[
        // 8 : L
        "   ", "██ ", "   ",
    ],
    &[
        // 9 : TL
        " █ ", "██ ", "   ",
    ],
    &[
        // 10: RL
        "   ",
        "███",
        "   ",
    ],
    &[
        // 11: TRL
        " █ ",
        "███",
        "   ",
    ],
    &[
        // 12: BL
        "   ", "██ ", " █ ",
    ],
    &[
        // 13: TBL
        " █ ", "██ ", " █ ",
    ],
    &[
        // 14: RBL
        "   ",
        "███",
        " █ ",
    ],
    &[
        // 15: TRBL
        " █ ",
        "███",
        " █ ",
    ],
];

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if let Some(room) = self.get_room(&Position(x, y)) {
                    write!(f, " {:0>}", borders_index(room.borders()))?
                };
            }
            writeln!(f)?;
        }

        write!(f, "    ")?;
        for _ in 0..self.width {
            write!(f, "┌─┐")?;
        }
        writeln!(f)?;
        for y in (0..self.height).rev() {
            for i in 0..3 {
                write!(f, "{:0>2} : ", y)?;
                for x in 0..self.width {
                    if let Some(room) = self.get_room(&Position(x, y)) {
                        let s = CELL_DISPLAY[borders_index(room.borders())][i];
                        write!(f, "{}", s)?;
                    } else {
                        write!(f, "???")?;
                    }
                }
                writeln!(f)?;
            }
        }
        write!(f, "    ")?;
        for _ in 0..self.width {
            write!(f, "└─┘")?;
        }
        Ok(())
    }
}

#[derive(Clone, Reflect)]
pub struct CellBorders {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

impl Default for CellBorders {
    fn default() -> Self {
        CellBorders {
            top: true,
            right: true,
            bottom: true,
            left: true,
        }
    }
}

#[derive(Clone, Reflect)]
pub struct Room {
    borders: CellBorders,
    visited: bool,
    //public item: Item = undefined;
}

impl Room {
    fn new() -> Self {
        Room {
            borders: CellBorders::default(),
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn clear(&mut self) {
        self.visited = false;
    }

    pub fn borders(&self) -> &CellBorders {
        &self.borders
    }
}

pub struct MazeBuilder {
    width: u32,
    height: u32,
    rng: ThreadRng,
}

impl MazeBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        MazeBuilder {
            width,
            height,
            rng: rand::thread_rng(),
        }
    }

    pub fn create_maze(&mut self) -> Maze {
        let mut backtracking = vec![];
        let mut maze = Maze::new(self.width, self.height);
        let mut current_position = Position(0, 0);
        let mut finished = false;
        maze.visit(&current_position);
        while !finished {
            if let Some(next_position) = self.get_next_neighboor(&maze, &current_position) {
                // Still some unvisited room next to current room
                self.remove_walls_between(&mut maze, &current_position, &next_position);
                maze.visit(&next_position);
                backtracking.push(next_position);
                current_position = next_position;
            } else if let Some(previous_position) = backtracking.pop() {
                // No more unvisited room next to current room : go back one room
                current_position = previous_position;
            } else {
                // No more unvisited room
                finished = true
            }
        }

        // Clear all rooms as they have been visited
        maze.clear();

        // Add items

        // Remove some more random walls
        let n_walls_to_remove = ((self.width * self.height) as f32 * 0.07) as usize;
        self.remove_random_walls(&mut maze, n_walls_to_remove);

        maze
    }

    /// Return a random position of an unvisited room next to the `pos`
    fn get_next_neighboor(&mut self, maze: &Maze, pos: &Position) -> Option<Position> {
        let mut neighbors = vec![];

        if let Some(left) = maze.left_position(pos) {
            if let Some(room) = maze.get_room(&left) {
                if !room.visited {
                    neighbors.push(left);
                }
            }
        }
        if let Some(right) = maze.right_position(pos) {
            if let Some(room) = maze.get_room(&right) {
                if !room.visited {
                    neighbors.push(right);
                }
            }
        }
        if let Some(up) = maze.up_position(pos) {
            if let Some(room) = maze.get_room(&up) {
                if !room.visited {
                    neighbors.push(up);
                }
            }
        }
        if let Some(down) = maze.down_position(pos) {
            if let Some(room) = maze.get_room(&down) {
                if !room.visited {
                    neighbors.push(down);
                }
            }
        }

        neighbors.choose(&mut self.rng).copied()
    }

    pub(crate) fn remove_walls_between(&self, maze: &mut Maze, p1: &Position, p2: &Position) {
        assert_eq!(p1.sqr_distance(p2), 1);
        // eprintln!(" - remove_walls_between({}, {}", p1, p2);
        if p1.0 > p2.0 {
            if let Some(r1) = maze.get_room_mut(p1) {
                r1.borders.left = false;
            }
            if let Some(r2) = maze.get_room_mut(p2) {
                r2.borders.right = false;
            }
        }
        if p1.0 < p2.0 {
            if let Some(r1) = maze.get_room_mut(p1) {
                r1.borders.right = false;
            }
            if let Some(r2) = maze.get_room_mut(p2) {
                r2.borders.left = false;
            }
        }
        if p1.1 > p2.1 {
            if let Some(r1) = maze.get_room_mut(p1) {
                r1.borders.bottom = false;
            }
            if let Some(r2) = maze.get_room_mut(p2) {
                r2.borders.top = false;
            }
        }
        if p1.1 < p2.1 {
            if let Some(r1) = maze.get_room_mut(p1) {
                r1.borders.top = false;
            }
            if let Some(r2) = maze.get_room_mut(p2) {
                r2.borders.bottom = false;
            }
        }
    }

    fn remove_random_walls(&mut self, maze: &mut Maze, n: usize) {
        let mut modifications = 0;
        while modifications < n {
            let x = self.rng.gen_range(1..self.width - 1);
            let y = self.rng.gen_range(1..self.height - 1);
            let pos = Position(x, y);
            let room = maze.get_room(&pos).unwrap();
            match self.rng.gen_range(0..4) {
                0 if room.borders.top => {
                    if let Some(up) = maze.up_position(&pos) {
                        self.remove_walls_between(maze, &pos, &up);
                        modifications += 1;
                    }
                }

                1 if room.borders.right => {
                    if let Some(right) = maze.right_position(&pos) {
                        self.remove_walls_between(maze, &pos, &right);
                        modifications += 1;
                    }
                }

                2 if room.borders.bottom => {
                    if let Some(bottom) = maze.down_position(&pos) {
                        self.remove_walls_between(maze, &pos, &bottom);
                        modifications += 1;
                    }
                }

                3 if room.borders.left => {
                    if let Some(left) = maze.left_position(&pos) {
                        self.remove_walls_between(maze, &pos, &left);
                        modifications += 1;
                    }
                }

                _ => {}
            }
        }
    }
}

// struct PositionConverter<'a> {
//     win_size: &'a WinSize,
// }

// impl<'a> PositionConverter<'a> {
//     fn new(win_size: &'a WinSize) -> Self {
//         PositionConverter { win_size }
//     }

//     /// Convert a Maze position to a screen position.
//     ///
//     /// screen = 64 x pos - win_size / 2 + 30
//     fn to_screen(&self, pos: &Position, z: f32) -> Vec3 {
//         let x_offset = -self.win_size.w / 2. + 30.;
//         let y_offset = -self.win_size.h / 2. + 30.;
//         Vec3::new(
//             64.0 * pos.x as f32 + x_offset,
//             64.0 * pos.y as f32 + y_offset,
//             z,
//         )
//     }

//     ///  Convert a scrren position to a Maze position
//     ///
//     /// pos = (screen - 30 + win_size / 2) / 64
//     fn to_position(&self, screen_pos: &Vec3) -> Position {
//         let x_offset = (screen_pos.x - 30. + self.win_size.w / 2.) / 64.;
//         let y_offset = (screen_pos.y - 30. + self.win_size.h / 2.) / 64.;
//         Position(x_offset as u32, y_offset as u32)
//     }
// }

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        // TODO Load maze in PreStartup
        //        let maze = MazeBuilder::new(24, 13).create_maze();
        let maze = MazeBuilder::new(5, 5).create_maze();

        app.insert_resource(maze).add_systems(Startup, maze_spawn);
    }
}

#[derive(Component, Default)]
struct MazeComponent {}

#[derive(Component)]
struct RoomComponent {}

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
}

fn maze_spawn(
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
        .spawn((Name::new("MAZE"), SpatialBundle::default()))
        .with_children(|maze_cmd| {
            // TODO : use Iterator
            for x in 0..maze.width() {
                for y in 00..maze.height() {
                    let pos = Position(x, y);
                    if let Some(room) = maze.get_room(&pos) {
                        if room.borders.top {
                            let wall = Wall::Top;
                            maze_cmd.spawn(PbrBundle {
                                mesh: meshes.add(wall.mesh()),
                                material: material_handle.clone(),
                                transform: wall.transform(&pos),
                                ..default()
                            });
                        }

                        if room.borders.left {
                            let wall = Wall::Left;
                            maze_cmd.spawn(PbrBundle {
                                mesh: meshes.add(wall.mesh()),
                                material: material_handle.clone(),
                                transform: wall.transform(&pos),
                                ..default()
                            });
                        }

                        if room.borders.bottom {
                            let wall = Wall::Bottom;
                            maze_cmd.spawn(PbrBundle {
                                mesh: meshes.add(wall.mesh()),
                                material: material_handle.clone(),
                                transform: wall.transform(&pos),
                                ..default()
                            });
                        }

                        if room.borders.right {
                            let wall = Wall::Right;
                            maze_cmd.spawn(PbrBundle {
                                mesh: meshes.add(wall.mesh()),
                                material: material_handle.clone(),
                                transform: wall.transform(&pos),
                                ..default()
                            });
                        }
                    }
                }
            }
        });
}
