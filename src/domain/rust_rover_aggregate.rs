#[derive(PartialEq, Debug, Clone)]
pub struct RustRover {
    coordinate: Coordinate,
    direction: Direction,
}

impl RustRover {
    pub fn new(coordinate: Coordinate, direction: Direction) -> RustRover {
        RustRover {
            coordinate,
            direction
        }
    }
}

pub trait RoverActions {
    fn move_forward(&mut self) -> ();
    fn move_backward(&mut self) -> ();
}

impl RoverActions for RustRover {
    fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.coordinate.y += 1,
            Direction::East => self.coordinate.x += 1,
            Direction::South => self.coordinate.y -= 1,
            Direction::West => self.coordinate.x -= 1,
        }
    }

    fn move_backward(&mut self) -> () {
        match self.direction {
            Direction::North => self.coordinate.y -= 1,
            Direction::East => self.coordinate.x -= 1,
            Direction::South => self.coordinate.y += 1,
            _ => ()
        }
    }
}

impl Default for RustRover {
    fn default() -> Self {
        RustRover::new(Coordinate::from(0,0), Direction::North)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn from(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}



#[cfg(test)]
mod rust_rover_tests {
    mod initialization_tests {
        use crate::domain::rust_rover_aggregate::*;

        #[test]
        fn rover_can_be_initialized_with_starting_point_and_direction() {
            let rover: RustRover = RustRover::new(Coordinate::from(14,0), Direction::North);
            assert_eq!(rover.coordinate, Coordinate::from(14,0));
            assert_eq!(rover.direction, Direction::North);
        }

        #[test]
        fn rover_can_have_north_direction() {
            let rover: RustRover = construct_rover_with_direction(Direction::North);
            assert_eq!(rover.direction, Direction::North);
        }

        #[test]
        fn rover_can_have_east_direction() {
            let rover: RustRover = construct_rover_with_direction(Direction::East);
            assert_eq!(rover.direction, Direction::East);
        }

        #[test]
        fn rover_can_have_south_direction() {
            let rover: RustRover = construct_rover_with_direction(Direction::South);
            assert_eq!(rover.direction, Direction::South);
        }

        #[test]
        fn rover_can_have_west_direction() {
            let rover: RustRover = construct_rover_with_direction(Direction::West);
            assert_eq!(rover.direction, Direction::West);
        }

        fn construct_rover_with_direction(direction: Direction) -> RustRover {
            RustRover::new(Coordinate::from(0,0), direction)
        }
    }

    mod movement_tests {
        use crate::domain::rust_rover_aggregate::*;
        #[test]
        fn move_forward_when_rover_facing_north_then_moves_up() {
            let mut rover: RustRover = RustRover::new(Coordinate::from(0,0), Direction::North);
            rover.move_forward();
            assert_eq!(rover.coordinate, Coordinate::from(0,1));
        }

        #[test]
        fn move_forward_when_rover_facing_east_then_moves_right() {
            let mut rover: RustRover = RustRover::new(Coordinate::from(0,0), Direction::East);
            rover.move_forward();
            assert_eq!(rover.coordinate, Coordinate::from(1,0));
        }

        #[test]
        fn move_forward_when_rover_facing_south_then_moves_down() {
            let mut rover: RustRover = RustRover::new(Coordinate::from(0,0), Direction::South);
            rover.move_forward();
            assert_eq!(rover.coordinate, Coordinate::from(0,-1));
        }

        #[test]
        fn move_forward_when_rover_facing_west_then_moves_left() {
            let mut rover: RustRover = RustRover::new(Coordinate::from(0,0), Direction::West);
            rover.move_forward();
            assert_eq!(rover.coordinate, Coordinate::from(-1,0));
        }

        #[test]
        fn move_backward_when_rover_facing_north_then_moves_down() {
            let mut rover: RustRover = RustRover::new(Coordinate::from(0, 0), Direction::North);
            rover.move_backward();
            assert_eq!(rover.coordinate, Coordinate::from(0, -1));
        }

        #[test]
        fn move_backward_when_rover_facing_east_then_moves_left() {
            let mut rover: RustRover = RustRover::new(Coordinate::from(0, 0), Direction::East);
            rover.move_backward();
            assert_eq!(rover.coordinate, Coordinate::from(-1, 0));
        }

        #[test]
        fn move_backward_when_rover_facing_south_then_moves_up() {
            let mut rover: RustRover = RustRover::new(Coordinate::from(0, 0), Direction::South);
            rover.move_backward();
            assert_eq!(rover.coordinate, Coordinate::from(0, 1));
        }
    }
}
