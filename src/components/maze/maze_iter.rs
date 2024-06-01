use super::*;

/// Iterator that return a `(&'m Room, Position)`
pub struct MazeIter<'m> {
    maze: &'m Maze,
    index: usize,
}

impl<'m> MazeIter<'m> {
    pub fn new(maze: &'m Maze) -> Self {
        MazeIter { maze, index: 0 }
    }
}

impl<'m> Iterator for MazeIter<'m> {
    type Item = (&'m Room, RoomPosition);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.maze.get_position(self.index);
        self.index += 1;
        self.maze.get_room(&pos).map(|room| (room, pos))
    }
}
