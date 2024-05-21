use super::*;
use bevy::prelude::*;
use rand::Rng;
use std::collections::{hash_set, HashSet};

#[derive(Resource)]
pub struct Level {
    maze: Maze,
    start: Position,
    end: Position,
    enemies: HashSet<Position>,
}

impl Level {
    pub fn new(width: u32, height: u32) -> Self {
        let maze = MazeBuilder::new(width, height).create_maze();

        let mut rng = rand::thread_rng();
        let mut enemies = HashSet::new();
        while enemies.len() < 5 {
            let (x, y) = (rng.gen_range(2..width), rng.gen_range(2..height));
            enemies.insert(Position(x, y));
        }

        Level {
            maze,
            start: Position(0, 0),
            end: Position(width - 1, height - 1),
            enemies,
        }
    }

    pub fn maze(&self) -> &Maze {
        &self.maze
    }

    pub fn maze_mut(&mut self) -> &mut Maze {
        &mut self.maze
    }

    pub fn start_position(&self) -> Position {
        self.start
    }

    pub fn end_position(&self) -> Position {
        self.end
    }

    pub fn enemies_start_pos(&self) -> hash_set::Iter<Position> {
        self.enemies.iter()
    }
}
