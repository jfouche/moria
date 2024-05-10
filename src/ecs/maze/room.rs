use bevy::reflect::Reflect;

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
    pub borders: CellBorders,
    pub visited: bool,
    //public item: Item = undefined;
}

impl Room {
    pub fn new() -> Self {
        Room {
            borders: CellBorders::default(),
            visited: false,
        }
    }

    pub fn visited(&self) -> bool {
        self.visited
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn clear(&mut self) {
        self.visited = false;
    }

    pub fn borders(&self) -> &CellBorders {
        &self.borders
    }
}
