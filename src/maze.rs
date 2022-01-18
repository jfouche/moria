use std::fmt;

use rand::{
    prelude::{SliceRandom, ThreadRng},
    Rng,
};

pub struct Maze {
    width: u32,
    height: u32,
    rooms: Vec<Room>,
}

impl Maze {
    fn new(width: u32, height: u32) -> Self {
        Maze {
            width,
            height,
            rooms: vec![Room::new(0, 0); (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_room(&self, pos: &Position) -> Option<&Room> {
        let index = (pos.y * self.height + pos.x) as usize;
        self.rooms.get(index)
    }

    fn get_room_mut(&mut self, pos: &Position) -> Option<&mut Room> {
        let index = (pos.y * self.height + pos.x) as usize;
        self.rooms.get_mut(index)
    }

    fn visit(&mut self, pos: &Position) {
        let index = (pos.y * self.height + pos.x) as usize;
        if let Some(room) = self.rooms.get_mut(index) {
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

    fn top_position(&self, pos: &Position) -> Option<Position> {
        if pos.y > 0 {
            Some(Position {
                x: pos.x,
                y: pos.y - 1,
            })
        } else {
            None
        }
    }

    fn bottom_position(&self, pos: &Position) -> Option<Position> {
        if pos.x < self.width - 1 {
            Some(Position {
                x: pos.x ,
                y: pos.y + 1,
            })
        } else {
            None
        }
    }

    fn clear(&mut self) {
        for room in &mut self.rooms {
            room.visit();
        }
    }
}

/// Get the index in the maze file
///  0:   ,  1: T,    2: R,    3: TR
///  4:  B,  5: TB,   6: RB,   7: TRB
///  8:  L,  9: TL,  10: RL,  11: TRL
/// 12: BL, 13: TBL, 14: RBL, 15: TRBL
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
        //
        "   ", 
        "   ", 
        "   ",
    ],
    &[
        // T
        " █ ", 
        " █ ", 
        "   ",
    ],
    &[
        // R
        "   ", 
        " ██", 
        "   ",
    ],
    &[
        // TR
        " █ ", 
        " ██", 
        "   ",
    ],
    &[
        // B
        "   ", 
        " █ ", 
        " █ ",
    ],
    &[
        // TB
        " █ ", 
        " █ ", 
        " █ ",
    ],
    &[
        // RB
        "   ", 
        " ██", 
        " █ ",
    ],
    &[
        // TRB
        " █ ", 
        " ██", 
        " █ ",
    ],
    &[
        // L
        "   ", 
        "██ ", 
        "   ",
    ],
    &[
        // TL
        " █ ", 
        "██ ", 
        "   ",
    ],
    &[
        // RL
        "   ",
        "███",
        "   ",
    ],
    &[
        // TRL
        " █ ",
        "███",
        "   ",
    ],
    &[
        // BL
        "   ", 
        "██ ", 
        " █ ",
    ],
    &[
        // TBL
        " █ ", 
        " ██", 
        "   ",
    ],
    &[
        // RBL
        "   ",
        "███",
        " █ ",
    ],
    &[
        // TRBL
        " █ ",
        "███",
        " █ ",
    ],
];

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(room) = self.get_room(&Position::new(x, y)) {
                    write!(f, " {}", borders_index(room.borders()))?
                };
            }
            write!(f, "\n")?;
        }
 
        write!(f, "    ")?;
        for _ in 0..self.width {
            write!(f, "┌─┐ ")?;
        }
        write!(f, "\n")?;
        for y in 0..self.height {
            for i in 0..3 {
                write!(f, "{} : ", y)?;
                for x in 0..self.width {
                    if let Some(room) = self.get_room(&Position::new(x, y)) {
                        let s = CELL_DISPLAY[borders_index(room.borders())][i];
                        write!(f, "{} ", s)?;
                    } 
                    else {
                        write!(f, "??? ")?;
                    }
                }
                write!(f, "\n")?;
            }
        }
        for _ in 0..self.width {
            write!(f, "└─┘ ")?;
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
}

#[derive(Clone)]
pub struct Room {
    col: u32,
    row: u32,
    borders: CellBorders,
    visited: bool,
    //public item: Item = undefined;
}

impl Room {
    fn new(row: u32, col: u32) -> Self {
        Room {
            col,
            row,
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
            eprintln!(" - current_position {:?}, backtracking: {:?}", current_position, backtracking);
            if let Some(next_position) = self.get_next_neighboor(&maze, &current_position) {
                eprintln!("     - next_position = {:?}", next_position);
                self.remove_walls_between(&mut maze, &current_position, &next_position);
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
        self.remove_random_walls(&mut maze, 0);

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
        if let Some(top) = maze.top_position(pos) {
            if let Some(room) = maze.get_room(&top) {
                if !room.visited {
                    neighbors.push(top);
                }
            }
        }
        if let Some(bottom) = maze.bottom_position(pos) {
            if let Some(room) = maze.get_room(&bottom) {
                if !room.visited {
                    neighbors.push(bottom);
                }
            }
        }

        if let Some(pos) = neighbors.choose(&mut self.rng) {
            Some(pos.clone())
        } else {
            None
        }
    }

    fn remove_walls_between(&self, maze: &mut Maze, p1: &Position, p2: &Position) {
        eprintln!(" - remove_walls_between({:?}, {:?}", p1, p2);
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
                r1.borders.top = false;
            }
            if let Some(mut r2) = maze.get_room_mut(p2) {
                r2.borders.bottom = false;
            }
        }
        if p1.y < p2.y {
            if let Some(mut r1) = maze.get_room_mut(p1) {
                r1.borders.bottom = false;
            }
            if let Some(mut r2) = maze.get_room_mut(p2) {
                r2.borders.top = false;
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
                // 0 => {
                //     if room.borders.top {
                //         if let Some(top) = maze.top_position(&pos) {
                //             self.remove_walls_between(maze, &pos, &top);
                //             modifications += 1;
                //         }
                //     }
                // },

                // 1 => {
                //     if room.borders.right {
                //         if let Some(right) = maze.right_position(&pos) {
                //             self.remove_walls_between(maze, &pos, &right);
                //             modifications += 1;
                //         }
                //     }
                // },

                // 2 => {
                //     if room.borders.bottom {
                //         if let Some(bottom) = maze.bottom_position(&pos) {
                //             self.remove_walls_between(maze, &pos, &bottom);
                //             modifications += 1;
                //         }
                //     }
                // },

                3 => {
                    if room.borders.left {
                        if let Some(left) = maze.left_position(&pos) {
                            self.remove_walls_between(maze, &pos, &left);
                            modifications += 1;
                        }
                    }
                    break;
                },
                _ => { 
                    panic!("Shouldn't be here");
                }
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
        let top = maze.top_position(&pos);
        assert!(top.is_none());
        let bottom = maze.bottom_position(&pos);
        assert!(bottom.is_some());
        assert_eq!(bottom.unwrap(), Position::new(0, 1));

        let pos = Position::new(1, 1);
        let left = maze.left_position(&pos);
        assert!(left.is_some());
        assert_eq!(left.unwrap(), Position::new(0, 1));
        let right = maze.right_position(&pos);
        assert!(right.is_none());
        let top = maze.top_position(&pos);
        assert!(top.is_some());
        assert_eq!(top.unwrap(), Position::new(1, 0));
        let bottom = maze.bottom_position(&pos);
        assert!(bottom.is_none());
    }

    #[test]
    fn it_removes_walls() {
        let maze_builder = MazeBuilder::new(2, 2);
        let mut maze = Maze::new(2, 2);

        let p1 = Position::new(0, 0);
        let p2 = Position::new(1, 0);
        maze_builder.remove_walls_between(&mut maze, &p1, &p2);
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
        maze_builder.remove_walls_between(&mut maze, &p1, &p2);
        let r1 = maze.get_room(&p1).unwrap();
        assert_eq!(r1.borders().top, false);
        assert_eq!(r1.borders().right, true);
        assert_eq!(r1.borders().bottom, true);
        assert_eq!(r1.borders().left, true);
        let r2 = maze.get_room(&p2).unwrap();
        assert_eq!(r2.borders().top, true);
        assert_eq!(r2.borders().right, true);
        assert_eq!(r2.borders().bottom, false);
        assert_eq!(r2.borders().left, false);
    }

    #[test]
    fn it_gives_borders_index() {
        assert_eq!(borders_index(&CellBorders::default()), 0);
        assert_eq!(borders_index(&CellBorders::new(true,  true,  true,  true )),  0);
        assert_eq!(borders_index(&CellBorders::new(false, true,  true,  true )),  1);
        assert_eq!(borders_index(&CellBorders::new(true,  false, true,  true )),  2);
        assert_eq!(borders_index(&CellBorders::new(false, false, true,  true )),  3);
        assert_eq!(borders_index(&CellBorders::new(true,  true,  false, true )),  4);
        assert_eq!(borders_index(&CellBorders::new(false, true,  false, true )),  5);
        assert_eq!(borders_index(&CellBorders::new(true,  false, false, true )),  6);
        assert_eq!(borders_index(&CellBorders::new(false, false, false, true )),  7);
        assert_eq!(borders_index(&CellBorders::new(true,  true,  true,  false)),  8);
        assert_eq!(borders_index(&CellBorders::new(false, true,  true,  false)),  9);
        assert_eq!(borders_index(&CellBorders::new(true,  false, true,  false)), 10);
        assert_eq!(borders_index(&CellBorders::new(false, false, true,  false)), 11);
        assert_eq!(borders_index(&CellBorders::new(true,  true,  false, false)), 12);
        assert_eq!(borders_index(&CellBorders::new(false, true,  false, false)), 13);
        assert_eq!(borders_index(&CellBorders::new(true,  false, false, false)), 14);
        assert_eq!(borders_index(&CellBorders::new(false, false, false, false)), 15);
    }
}