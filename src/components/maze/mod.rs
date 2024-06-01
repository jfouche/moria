mod builder;
mod end_level;
mod maze_iter;
mod position;
mod room;
mod wall;

#[cfg(test)]
mod test;

pub use builder::*;
pub use end_level::*;
pub use maze_iter::*;
pub use position::*;
pub use room::*;
pub use wall::*;

/// ```text
/// ^ (0, h)     (w, h)
/// |  
/// | (0, 0)     (w, 0)
/// + ----------------->
/// ```
pub struct Maze {
    width: u32,
    height: u32,
    rooms: Vec<Room>,
}

impl Maze {
    pub(crate) fn new(width: u32, height: u32) -> Self {
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

    pub(crate) fn room_index(&self, pos: &RoomPosition) -> usize {
        (pos.1 * self.width + pos.0) as usize
    }

    fn get_position(&self, index: usize) -> RoomPosition {
        let x = index as u32 % self.width;
        let y = index as u32 / self.width;
        RoomPosition(x, y)
    }

    pub fn get_room(&self, pos: &RoomPosition) -> Option<&Room> {
        self.rooms.get(self.room_index(pos))
    }

    fn get_room_mut(&mut self, pos: &RoomPosition) -> Option<&mut Room> {
        let index = self.room_index(pos);
        self.rooms.get_mut(index)
    }

    pub fn visit(&mut self, pos: &RoomPosition) {
        if let Some(room) = self.get_room_mut(pos) {
            room.visit();
        }
    }

    pub(crate) fn left_position(&self, pos: &RoomPosition) -> Option<RoomPosition> {
        if pos.0 > 0 {
            Some(RoomPosition(pos.0 - 1, pos.1))
        } else {
            None
        }
    }

    pub(crate) fn right_position(&self, pos: &RoomPosition) -> Option<RoomPosition> {
        if pos.0 < self.width - 1 {
            Some(RoomPosition(pos.0 + 1, pos.1))
        } else {
            None
        }
    }

    pub(crate) fn up_position(&self, pos: &RoomPosition) -> Option<RoomPosition> {
        if pos.1 < self.height - 1 {
            Some(RoomPosition(pos.0, pos.1 + 1))
        } else {
            None
        }
    }

    pub(crate) fn down_position(&self, pos: &RoomPosition) -> Option<RoomPosition> {
        if pos.1 > 0 {
            Some(RoomPosition(pos.0, pos.1 - 1))
        } else {
            None
        }
    }

    fn clear(&mut self) {
        for room in &mut self.rooms {
            room.clear();
        }
    }

    // pub fn get_next_room(&self, pos: &Position, dir: Direction) -> Option<&Room> {
    //     let borders = &self.get_room(pos)?.borders;
    //     match dir {
    //         Direction::TOP if !borders.top => self.get_room(&Position(pos.0, pos.1 + 1)),
    //         Direction::RIGHT if !borders.right => self.get_room(&Position(pos.0 + 1, pos.1)),
    //         Direction::BOTTOM if !borders.bottom => self.get_room(&Position(pos.0, pos.1 - 1)),
    //         Direction::LEFT if !borders.left => self.get_room(&Position(pos.0 - 1, pos.1)),
    //         _ => None,
    //     }
    // }
}

impl<'m> Maze {
    pub fn iter(&'m self) -> MazeIter<'m> {
        MazeIter::new(self)
    }
}
