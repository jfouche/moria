use super::*;
use bevy::prelude::*;
use rand::Rng;
use std::collections::{hash_map, hash_set, HashMap, HashSet};

#[derive(Resource)]
pub struct Level {
    maze: Maze,
    start: Position,
    end: Position,
    enemies: HashSet<Position>,
    items: HashMap<Position, Item>,
}

impl Level {
    pub fn new(width: u32, height: u32) -> Self {
        let maze = MazeBuilder::new(width, height).create_maze();

        let mut rng = rand::thread_rng();
        let mut rnd_pos = || Position(rng.gen_range(2..width), rng.gen_range(2..height));

        // Add enemies
        let mut enemies = HashSet::new();
        while enemies.len() < 5 {
            enemies.insert(rnd_pos());
        }

        // Add items
        let mut items = HashMap::new();
        while items.len() < 2 {
            let pos = rnd_pos();
            if !enemies.contains(&pos) {
                items.insert(pos, Item::Potion(Potion::Life(20)));
            }
        }

        Level {
            maze,
            start: Position(0, 0),
            end: Position(width - 1, height - 1),
            enemies,
            items,
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

    pub fn items(&self) -> hash_map::Iter<Position, Item> {
        self.items.iter()
    }
}
