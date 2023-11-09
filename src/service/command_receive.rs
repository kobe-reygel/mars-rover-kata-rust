use crate::domain::rust_rover_aggregate::*;
fn receive_command(rover: RustRover, command_array: &[char]) -> Result<RustRover, CommandReceiveError>{
    Ok(rover)
}

struct CommandReceiveError;



#[cfg(test)]
mod command_receive_service_tests {
    use crate::domain::rust_rover_aggregate::*;
    use crate::service::command_receive::receive_command;

    #[test]
    fn rover_can_receive_a_character_array_of_commands() {
        let rover: RustRover = RustRover::default();
        let command_array = ['T', 'O'];
        match receive_command(rover, &command_array) {
            Ok(_) => (),
            Err(_) => panic!("Expected test to not panic"),
        }
    }
}