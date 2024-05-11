use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Life {
    current: u16,
    max: u16,
}

impl Life {
    pub fn new(life: u16) -> Self {
        Life {
            current: life,
            max: life,
        }
    }

    pub fn get(&self) -> u16 {
        self.current
    }

    pub fn hit(&mut self, damage: u16) {
        if damage > self.current {
            self.current = 0;
        } else {
            self.current -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }

    pub fn add(&mut self, life: u16) {
        self.current = std::cmp::max(self.current + life, self.max);
    }
}
