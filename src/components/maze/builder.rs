use super::*;
use rand::{
    prelude::{SliceRandom, ThreadRng},
    Rng,
};

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
