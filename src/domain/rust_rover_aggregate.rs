#[derive(PartialEq, Debug, Clone)]
struct RustRover {
    coordinate: Coordinate,
    direction: Direction,
}

impl RustRover {
    fn new(coordinate: Coordinate, direction: Direction) -> RustRover {
        RustRover {
            coordinate,
            direction
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn from(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}



#[cfg(test)]
mod rust_rover_tests {
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