use rand::{
    prelude::{SliceRandom, ThreadRng}, Rng,
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
            rooms: vec![],
        }
    }

    fn get_room(&self, pos: &Position) -> Option<&Room> {
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

    fn left(&self, pos: &Position) -> Option<Position> {
        if pos.x > 0 {
            Some(Position {
                x: pos.x - 1,
                y: pos.y,
            })
        } else {
            None
        }
    }

    fn right(&self, pos: &Position) -> Option<Position> {
        if pos.x < self.width - 1 {
            Some(Position {
                x: pos.x + 1,
                y: pos.y,
            })
        } else {
            None
        }
    }

    fn top(&self, pos: &Position) -> Option<Position> {
        if pos.y > 0 {
            Some(Position {
                x: pos.x,
                y: pos.y - 1,
            })
        } else {
            None
        }
    }

    fn bottom(&self, pos: &Position) -> Option<Position> {
        if pos.x < self.width - 1 {
            Some(Position {
                x: pos.x + 1,
                y: pos.y,
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

pub struct CellBorders {
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}

impl CellBorders {
    fn new() -> Self {
        CellBorders {
            top: true,
            right: true,
            bottom: true,
            left: true,
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }
}

struct Room {
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
            borders: CellBorders::new(),
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn clear(&mut self) {
        self.visited = false;
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
            if let Some(pos) = self.get_next_neighboor(&maze, &current_position) {
                maze.visit(&current_position);
                backtracking.push(pos);
                self.remove_walls_between(&mut maze, &current_position, &pos);
                current_position = pos;
            } else if backtracking.len() > 0 {
                current_position = backtracking.pop().unwrap();
            } else {
                finished = true
            }
        }

        // Clear all rooms as they have been visited
        maze.clear();

        // Add items

        // Remove some more random walls
        self.remove_random_walls(&mut maze, 8);

        maze
    }

    fn get_next_neighboor(&mut self, maze: &Maze, pos: &Position) -> Option<Position> {
        let mut neighbors = vec![];

        if let Some(left) = maze.left(pos) {
            if let Some(room) = maze.get_room(&left) {
                if !room.visited {
                    neighbors.push(left);
                }
            }
        }
        if let Some(right) = maze.right(pos) {
            if let Some(room) = maze.get_room(&right) {
                if !room.visited {
                    neighbors.push(right);
                }
            }
        }
        if let Some(top) = maze.top(pos) {
            if let Some(room) = maze.get_room(&top) {
                if !room.visited {
                    neighbors.push(top);
                }
            }
        }
        if let Some(bottom) = maze.bottom(pos) {
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
                x: self.rng.gen_range(1..self.width-1),
                y: self.rng.gen_range(1..self.height-1)
            };
            let room = maze.get_room(&pos).unwrap();
            match self.rng.gen_range(0..=3) {
                0 => {
                    if room.borders.top {
                        if let Some(top) = maze.top(&pos) {
                            self.remove_walls_between(maze, &pos, &top);
                            modifications += 1;
                        }
                    }
                },

                1 => {
                    if room.borders.right {
                        if let Some(right) = maze.right(&pos) {
                            self.remove_walls_between(maze, &pos, &right);
                            modifications += 1;
                        }
                    }
                },

                2 => {
                    if room.borders.bottom {
                        if let Some(bottom) = maze.bottom(&pos) {
                            self.remove_walls_between(maze, &pos, &bottom);
                            modifications += 1;
                        }
                    }
                },

                3 => {
                    if room.borders.left {
                        if let Some(left) = maze.left(&pos) {
                            self.remove_walls_between(maze, &pos, &left);
                            modifications += 1;
                        }
                    }
                    break;
                } ,
                _ => {}
            }
        }
    }
}
