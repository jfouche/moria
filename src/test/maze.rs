#[allow(clippy::all)]
#[cfg(test)]
mod test {
    use crate::maze::{borders_index, CellBorders, Maze, MazeBuilder, Position, ROOM_WIDTH};
    use bevy::prelude::*;

    impl CellBorders {
        fn new(top: bool, right: bool, bottom: bool, left: bool) -> Self {
            CellBorders {
                top,
                right,
                bottom,
                left,
            }
        }
    }

    #[test]
    fn it_gets_neighbour_position() {
        let maze = Maze::new(2, 2);

        let pos = Position(0, 0);
        let left = maze.left_position(&pos);
        assert!(left.is_none());
        let right = maze.right_position(&pos);
        assert!(right.is_some());
        assert_eq!(right.unwrap(), Position(1, 0));
        let down = maze.down_position(&pos);
        assert!(down.is_none());
        let up = maze.up_position(&pos);
        assert!(up.is_some());
        assert_eq!(up.unwrap(), Position(0, 1));

        let pos = Position(1, 1);
        let left = maze.left_position(&pos);
        assert!(left.is_some());
        assert_eq!(left.unwrap(), Position(0, 1));
        let right = maze.right_position(&pos);
        assert!(right.is_none());
        let down = maze.down_position(&pos);
        assert!(down.is_some());
        assert_eq!(down.unwrap(), Position(1, 0));
        let up = maze.up_position(&pos);
        assert!(up.is_none());
    }

    #[test]
    fn it_removes_walls() {
        let (width, height) = (2, 2);
        let maze_builder = MazeBuilder::new(width, height);
        let mut maze = Maze::new(width, height);

        let p1 = Position(0, 0);
        let p2 = Position(1, 0);

        //  -- --
        // |  |  |
        //  -- --
        // |p1|p2|
        //  -- --

        maze_builder.remove_walls_between(&mut maze, &p1, &p2);

        //  -- --
        // |  |  |
        //  -- --
        // |p1 p2|
        //  -- --

        let r1 = maze.get_room(&p1).unwrap();
        assert_eq!(r1.borders().top, true);
        assert_eq!(r1.borders().right, false);
        assert_eq!(r1.borders().bottom, true);
        assert_eq!(r1.borders().left, true);
        let r2 = maze.get_room(&p2).unwrap();
        assert_eq!(r2.borders().top, true);
        assert_eq!(r2.borders().right, true);
        assert_eq!(r2.borders().bottom, true);
        assert_eq!(r2.borders().left, false);

        let p1 = Position(1, 1);

        //  -- --
        // |  |p1|
        //  -- --
        // |   p2|
        //  -- --

        maze_builder.remove_walls_between(&mut maze, &p1, &p2);

        //  -- --
        // |  |p1|
        //  --
        // |   p2|
        //  -- --

        let r1 = maze.get_room(&p1).unwrap();
        assert_eq!(r1.borders().top, true);
        assert_eq!(r1.borders().right, true);
        assert_eq!(r1.borders().bottom, false);
        assert_eq!(r1.borders().left, true);
        let r2 = maze.get_room(&p2).unwrap();
        assert_eq!(r2.borders().top, false);
        assert_eq!(r2.borders().right, true);
        assert_eq!(r2.borders().bottom, true);
        assert_eq!(r2.borders().left, false);
    }

    #[test]
    fn it_gives_room_index() {
        let maze = Maze::new(6, 4);
        assert_eq!(maze.room_index(&Position(0, 0)), 0);
        assert_eq!(maze.room_index(&Position(5, 0)), 5);
        assert_eq!(maze.room_index(&Position(0, 3)), 18);
        assert_eq!(maze.room_index(&Position(5, 3)), 23);
    }

    #[test]
    fn it_gives_borders_index() {
        assert_eq!(borders_index(&CellBorders::default()), 0);
        assert_eq!(borders_index(&CellBorders::new(true, true, true, true)), 0);
        assert_eq!(borders_index(&CellBorders::new(false, true, true, true)), 1);
        assert_eq!(borders_index(&CellBorders::new(true, false, true, true)), 2);
        assert_eq!(
            borders_index(&CellBorders::new(false, false, true, true)),
            3
        );
        assert_eq!(borders_index(&CellBorders::new(true, true, false, true)), 4);
        assert_eq!(
            borders_index(&CellBorders::new(false, true, false, true)),
            5
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, false, false, true)),
            6
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, false, false, true)),
            7
        );
        assert_eq!(borders_index(&CellBorders::new(true, true, true, false)), 8);
        assert_eq!(
            borders_index(&CellBorders::new(false, true, true, false)),
            9
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, false, true, false)),
            10
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, false, true, false)),
            11
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, true, false, false)),
            12
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, true, false, false)),
            13
        );
        assert_eq!(
            borders_index(&CellBorders::new(true, false, false, false)),
            14
        );
        assert_eq!(
            borders_index(&CellBorders::new(false, false, false, false)),
            15
        );
    }

    // #[test]
    // fn it_converts_positions() {
    //     let win_size = WinSize {
    //         w: 30. + 64. * 5. + 30.,
    //         h: 30. + 64. * 4. + 30.,
    //     };
    //     let pos_converter = PositionConverter::new(&win_size);

    //     // assert_eq!(pos_converter.to_position(Vec3::new(0., 0., 0.)), None);
    //     assert_eq!(
    //         pos_converter.to_position(&Vec3::new(40., 40., 0.)),
    //         Position(0, 0)
    //     );
    // }

    #[test]
    fn convert_position() {
        let pos = Position(0, 0);
        let world_pos = pos.world_pos();
        assert_eq!(Vec3::ZERO, world_pos);

        let pos = Position(0, 1);
        let world_pos = pos.world_pos();
        assert_eq!(Vec3::new(0., 0., -ROOM_WIDTH), world_pos);

        let pos = Position(1, 0);
        let world_pos = pos.world_pos();
        assert_eq!(Vec3::new(ROOM_WIDTH, 0., 0.), world_pos);
    }
}
