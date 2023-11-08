



#[cfg(test)]
mod rust_rover_tests {

    #[test]
    fn rover_can_be_initialized_with_starting_point_and_direction() {
        let starting_coordinate: Coordinate = Coordinate { x: 0, y: 0 };
        let starting_direction: Direction = Direction::North;
        let rover: RustRover = RustRover::new(starting_coordinate, starting_direction);
        assert_eq!(rover.coordinate, starting_coordinate);
        assert_eq!(rover.direction, starting_direction);
    }
}
