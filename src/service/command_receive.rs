
#[cfg(test)]
mod command_receive_service_tests {
    #[test]
    fn rover_can_receive_a_character_array_of_commands() {
        let rover: RustRover = RustRover::new(Coordinate::from(0,0), Direction::North);
        let command_array = ['T'];
        match receive_command(rover, &command_array) {
            Ok(_) => (),
            Err(_) => panic!("Expected test to not panic"),
        }
    }


}