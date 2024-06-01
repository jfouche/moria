use super::*;
use bevy::prelude::*;
use rand::Rng;
use std::collections::{hash_map, hash_set, HashMap, HashSet};

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct CurrentLevel(pub usize);

#[derive(Resource)]
pub struct Level {
    maze: Maze,
    start: RoomPosition,
    end: RoomPosition,
    enemies: HashSet<RoomPosition>,
    items: HashMap<RoomPosition, Item>,
}

impl Level {
    pub fn new(width: u32, height: u32) -> Self {
        let maze = MazeBuilder::new(width, height).create_maze();

        Level {
            maze,
            start: RoomPosition(0, 0),
            end: RoomPosition(width - 1, height - 1),
            enemies: HashSet::new(),
            items: HashMap::new(),
        }
    }

    fn rnd_pos(&self) -> RoomPosition {
        let mut rng = rand::thread_rng();
        RoomPosition(
            rng.gen_range(2..self.maze.width()),
            rng.gen_range(2..self.maze.height()),
        )
    }

    /// Add enemies in maze
    ///
    /// `enemy_density` is the percent of enemies according to the number of rooms
    pub fn add_enemies(&mut self, enemy_density: f32) {
        let n_rooms = self.maze.width() * self.maze.height();
        let n_enemies = (n_rooms as f32 * enemy_density).round() as usize;
        while self.enemies.len() < n_enemies {
            self.enemies.insert(self.rnd_pos());
        }
        info!("add_enemies() : {n_enemies}")
    }

    /// Add items in maze
    ///
    /// `item_density` is the percent of item according to the number of rooms
    pub fn add_items(&mut self, item_density: f32) {
        let n_rooms = self.maze.width() * self.maze.height();
        let n_items = (n_rooms as f32 * item_density).round() as usize;
        while self.items.len() < n_items {
            let pos = self.rnd_pos();
            if !self.enemies.contains(&pos) {
                let item = Item::Potion(Potion::Life(20));
                self.items.insert(pos, item);
            }
        }
        info!("add_items() : {n_items}")
    }

    pub fn maze(&self) -> &Maze {
        &self.maze
    }

    pub fn maze_mut(&mut self) -> &mut Maze {
        &mut self.maze
    }

    pub fn start_position(&self) -> RoomPosition {
        self.start
    }

    pub fn end_position(&self) -> RoomPosition {
        self.end
    }

    pub fn enemies_start_pos(&self) -> hash_set::Iter<RoomPosition> {
        self.enemies.iter()
    }

    pub fn items(&self) -> hash_map::Iter<RoomPosition, Item> {
        self.items.iter()
    }
}
