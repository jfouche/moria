use super::*;

#[allow(clippy::all)]
#[test]
fn it_gets_neighbour_position() {
    let maze = Maze::new(2, 2);

    let pos = RoomPosition(0, 0);
    let left = maze.left_position(&pos);
    assert!(left.is_none());
    let right = maze.right_position(&pos);
    assert!(right.is_some());
    assert_eq!(right.unwrap(), RoomPosition(1, 0));
    let down = maze.down_position(&pos);
    assert!(down.is_none());
    let up = maze.up_position(&pos);
    assert!(up.is_some());
    assert_eq!(up.unwrap(), RoomPosition(0, 1));

    let pos = RoomPosition(1, 1);
    let left = maze.left_position(&pos);
    assert!(left.is_some());
    assert_eq!(left.unwrap(), RoomPosition(0, 1));
    let right = maze.right_position(&pos);
    assert!(right.is_none());
    let down = maze.down_position(&pos);
    assert!(down.is_some());
    assert_eq!(down.unwrap(), RoomPosition(1, 0));
    let up = maze.up_position(&pos);
    assert!(up.is_none());
}

#[allow(clippy::all)]
#[test]
fn it_removes_walls() {
    let (width, height) = (2, 2);
    let maze_builder = MazeBuilder::new(width, height);
    let mut maze = Maze::new(width, height);

    let p1 = RoomPosition(0, 0);
    let p2 = RoomPosition(1, 0);

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

    let p1 = RoomPosition(1, 1);

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

#[allow(clippy::all)]
#[test]
fn it_gives_room_index() {
    let maze = Maze::new(6, 4);
    assert_eq!(maze.room_index(&RoomPosition(0, 0)), 0);
    assert_eq!(maze.room_index(&RoomPosition(5, 0)), 5);
    assert_eq!(maze.room_index(&RoomPosition(0, 3)), 18);
    assert_eq!(maze.room_index(&RoomPosition(5, 3)), 23);
}

#[test]
fn it_gives_room_position() {
    let maze = Maze::new(3, 4);
    assert_eq!(maze.get_position(0), RoomPosition(0, 0));
    assert_eq!(maze.get_position(2), RoomPosition(2, 0));
    assert_eq!(maze.get_position(3), RoomPosition(0, 1));
}
