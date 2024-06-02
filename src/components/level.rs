use super::*;
use bevy::prelude::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct CurrentLevel(pub usize);

#[derive(Resource)]
pub struct Level {
    pub maze: Maze,
    pub start: Position,
    pub end: Position,
    pub enemies: HashSet<Position>,
    pub items: HashMap<Position, Item>,
    pub enemy_bonus: f32,
}

impl Level {
    pub fn new(config: &LevelConfig) -> Self {
        // Create maze
        let (n_col, n_row) = (config.rows, config.cols);
        let maze = MazeBuilder::new(n_col, n_row).create_maze();

        let mut rng = rand::thread_rng();
        let n_rooms = (config.cols * config.rows) as f32;
        let mut rng_pos = || Position(rng.gen_range(2..n_col), rng.gen_range(2..n_row));

        // Add enemies
        let mut enemies = HashSet::new();
        let n_enemies = (n_rooms * config.enemy_density).round() as usize;
        while enemies.len() < n_enemies {
            enemies.insert(rng_pos());
        }
        info!("Added {n_enemies} enemies");

        // Add items
        let mut items = HashMap::new();
        let n_items = (n_rooms * config.item_density).round() as usize;
        while items.len() < n_items {
            let pos = rng_pos();
            if !enemies.contains(&pos) {
                let item = Item::Potion(Potion::Life(20));
                items.insert(pos, item);
            }
        }
        info!("add_items() : {n_items}");

        Level {
            maze,
            start: Position(0, 0),
            end: Position(n_col - 1, n_row - 1),
            enemies,
            items,
            enemy_bonus: config.enemy_bonus,
        }
    }
}
