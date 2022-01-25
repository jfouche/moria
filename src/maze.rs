use std::fmt;

use rand::{
    prelude::{SliceRandom, ThreadRng},
    Rng,
};

pub enum Direction {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

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
    fn new(width: u32, height: u32) -> Self {
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

    fn room_index(&self, pos: &Position) -> usize {
        (pos.y * self.width + pos.x) as usize
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

    fn left_position(&self, pos: &Position) -> Option<Position> {
        if pos.x > 0 {
            Some(Position {
                x: pos.x - 1,
                y: pos.y,
            })
        } else {
            None
        }
    }

    fn right_position(&self, pos: &Position) -> Option<Position> {
        if pos.x < self.width - 1 {
            Some(Position {
                x: pos.x + 1,
                y: pos.y,
            })
        } else {
            None
        }
    }

    fn up_position(&self, pos: &Position) -> Option<Position> {
        if pos.y < self.height - 1 {
            Some(Position {
                x: pos.x,
                y: pos.y + 1,
            })
        } else {
            None
        }
    }

    fn down_position(&self, pos: &Position) -> Option<Position> {
        if pos.y > 0 {
            Some(Position {
                x: pos.x,
                y: pos.y - 1,
            })
        } else {
            None
        }
    }

    fn clear(&mut self) {
        for room in &mut self.rooms {
            room.clear();
        }
    }

    pub fn get_next_room(&self, pos: &Position, dir: Direction) -> Option<&Room> {
        let borders = &self.get_room(pos)?.borders;
        match dir {
            Direction::TOP if !borders.top => self.get_room(&Position::new(pos.x, pos.y + 1)),
            Direction::RIGHT if !borders.right => self.get_room(&Position::new(pos.x + 1, pos.y)),
            Direction::BOTTOM if !borders.bottom => self.get_room(&Position::new(pos.x, pos.y - 1)),
            Direction::LEFT if !borders.left => self.get_room(&Position::new(pos.x - 1, pos.y)),
            _ => None,
        }
    }
}

/// Get the index in the maze file
/// ```text
///  0:   ,  1: T,    2: R,    3: TR
///  4:  B,  5: TB,   6: RB,   7: TRB
///  8:  L,  9: TL,  10: RL,  11: TRL
/// 12: BL, 13: TBL, 14: RBL, 15: TRBL
/// ```
fn borders_index(borders: &CellBorders) -> usize {
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
                if let Some(room) = self.get_room(&Position::new(x, y)) {
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
                    if let Some(room) = self.get_room(&Position::new(x, y)) {
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

#[derive(Clone)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }

    /// Get the square af the distance
    pub fn distance(&self, other: &Position) -> u32 {
        let dx = self.x as i32 - other.x as i32;
        let dy = self.y as i32 - other.y as i32;
        (dx * dx + dy * dy) as u32
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
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
        let mut current_position = Position::new(0, 0);
        let mut finished = false;
        maze.visit(&current_position);
        while !finished {
            // eprintln!(" - current_position {}, backtracking: {}", current_position, backtracking);
            if let Some(next_position) = self.get_next_neighboor(&maze, &current_position) {
                // eprintln!("     - next_position = {:?}", next_position);
                self.remove_walls_between(&mut maze, &current_position, &next_position);
                // eprintln!("{}", maze);
                maze.visit(&next_position);
                backtracking.push(next_position);
                current_position = next_position;
            } else if let Some(previous_position) = backtracking.pop() {
                current_position = previous_position;
            } else {
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

    fn remove_walls_between(&self, maze: &mut Maze, p1: &Position, p2: &Position) {
        assert_eq!(p1.distance(p2), 1);
        // eprintln!(" - remove_walls_between({}, {}", p1, p2);
        if p1.x > p2.x {
            if let Some(mut r1) = maze.get_room_mut(p1) {
                r1.borders.left = false;
            }
            if let Some(mut r2) = maze.get_room_mut(p2) {
                r2.borders.right = false;
            }
        }
        if p1.x < p2.x {
            if let Some(mut r1) = maze.get_room_mut(p1) {
                r1.borders.right = false;
            }
            if let Some(mut r2) = maze.get_room_mut(p2) {
                r2.borders.left = false;
            }
        }
        if p1.y > p2.y {
            if let Some(mut r1) = maze.get_room_mut(p1) {
                r1.borders.bottom = false;
            }
            if let Some(mut r2) = maze.get_room_mut(p2) {
                r2.borders.top = false;
            }
        }
        if p1.y < p2.y {
            if let Some(mut r1) = maze.get_room_mut(p1) {
                r1.borders.top = false;
            }
            if let Some(mut r2) = maze.get_room_mut(p2) {
                r2.borders.bottom = false;
            }
        }
    }

    fn remove_random_walls(&mut self, maze: &mut Maze, n: usize) {
        let mut modifications = 0;
        while modifications < n {
            let pos = Position {
                x: self.rng.gen_range(1..self.width - 1),
                y: self.rng.gen_range(1..self.height - 1),
            };
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

#[cfg(test)]
mod test {
    use super::*;

    impl CellBorders {
        fn new(top: bool, right: bool, bottom: bool, left: bool) -> Self {
            CellBorders {
                top,
                right,
                bottom,
                left,
            }
        }
    }

    #[test]
    fn it_gets_neighbour_position() {
        let maze = Maze::new(2, 2);

        let pos = Position::new(0, 0);
        let left = maze.left_position(&pos);
        assert!(left.is_none());
        let right = maze.right_position(&pos);
        assert!(right.is_some());
        assert_eq!(right.unwrap(), Position::new(1, 0));
        let down = maze.down_position(&pos);
        assert!(down.is_none());
        let up = maze.up_position(&pos);
        assert!(up.is_some());
        assert_eq!(up.unwrap(), Position::new(0, 1));

        let pos = Position::new(1, 1);
        let left = maze.left_position(&pos);
        assert!(left.is_some());
        assert_eq!(left.unwrap(), Position::new(0, 1));
        let right = maze.right_position(&pos);
        assert!(right.is_none());
        let down = maze.down_position(&pos);
        assert!(down.is_some());
        assert_eq!(down.unwrap(), Position::new(1, 0));
        let up = maze.up_position(&pos);
        assert!(up.is_none());
    }

    #[test]
    fn it_removes_walls() {
        let (width, height) = (2, 2);
        let maze_builder = MazeBuilder::new(width, height);
        let mut maze = Maze::new(width, height);

        let p1 = Position::new(0, 0);
        let p2 = Position::new(1, 0);

        //  -- --
        // |  |  |
        //  -- --
        // |p1|p2|
        //  -- --

        maze_builder.remove_walls_between(&mut maze, &p1, &p2);

        //  -- --
        // |  |  |
        //  -- --
        // |p1 p2|
        //  -- --

        let r1 = maze.get_room(&p1).unwrap();
        assert_eq!(r1.borders().top, true);
        assert_eq!(r1.borders().right, false);
        assert_eq!(r1.borders().bottom, true);
        assert_eq!(r1.borders().left, true);
        let r2 = maze.get_room(&p2).unwrap();
        assert_eq!(r2.borders().top, true);
        assert_eq!(r2.borders().right, true);
        assert_eq!(r2.borders().bottom, true);
        assert_eq!(r2.borders().left, false);

        let p1 = Position::new(1, 1);

        //  -- --
        // |  |p1|
        //  -- --
        // |   p2|
        //  -- --

        maze_builder.remove_walls_between(&mut maze, &p1, &p2);

        //  -- --
        // |  |p1|
        //  --
        // |   p2|
        //  -- --

        let r1 = maze.get_room(&p1).unwrap();
        assert_eq!(r1.borders().top, true);
        assert_eq!(r1.borders().right, true);
        assert_eq!(r1.borders().bottom, false);
        assert_eq!(r1.borders().left, true);
        let r2 = maze.get_room(&p2).unwrap();
        assert_eq!(r2.borders().top, false);
        assert_eq!(r2.borders().right, true);
        assert_eq!(r2.borders().bottom, true);
        assert_eq!(r2.borders().left, false);
    }

    #[test]
    fn it_gives_room_index() {
        let maze = Maze::new(6, 4);
        assert_eq!(maze.room_index(&Position::new(0, 0)), 0);
        assert_eq!(maze.room_index(&Position::new(5, 0)), 5);
        assert_eq!(maze.room_index(&Position::new(0, 3)), 18);
        assert_eq!(maze.room_index(&Position::new(5, 3)), 23);
    }

    #[test]
    fn it_gives_borders_index() {
        assert_eq!(borders_index(&CellBorders::default()), 0);
        assert_eq!(borders_index(&CellBorders::new(true, true, true, true)), 0);
        assert_eq!(borders_index(&CellBorders::new(false, true, true, true)), 1);
        assert_eq!(borders_index(&CellBorders::new(true, false, true, true)), 2);
        assert_eq!(
            borders_index(&CellBorders::new(false, false, true, true)),
            3
        );
        assert_eq!(borders_index(&CellBorders::new(true, true, false, true)), 4);
        assert_eq!(
            borders_index(&CellBorders::new(false, true, false, true)),
            5
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, false, false, true)),
            6
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, false, false, true)),
            7
        );
        assert_eq!(borders_index(&CellBorders::new(true, true, true, false)), 8);
        assert_eq!(
            borders_index(&CellBorders::new(false, true, true, false)),
            9
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, false, true, false)),
            10
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, false, true, false)),
            11
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, true, false, false)),
            12
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, true, false, false)),
            13
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, false, false, false)),
            14
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, false, false, false)),
            15
        );
    }
}
