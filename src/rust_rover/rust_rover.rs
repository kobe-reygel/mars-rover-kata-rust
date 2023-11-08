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

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    North,
}



#[cfg(test)]
mod rust_rover_tests {
    use crate::rust_rover::rust_rover::*;

    #[test]
    fn rover_can_be_initialized_with_starting_point_and_direction() {
        let starting_coordinate: Coordinate = Coordinate { x: 0, y: 0 };
        let starting_direction: Direction = Direction::North;
        let rover: RustRover = RustRover::new(starting_coordinate.clone(), starting_direction.clone());
        assert_eq!(rover.coordinate, starting_coordinate);
        assert_eq!(rover.direction, starting_direction);
    }
}
