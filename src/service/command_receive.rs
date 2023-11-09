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

    #[test]
    fn rover_moves_forward_when_given_a_forward_command() {
        let rover: TestRover = TestRover::new();
        let command_array = ['f'];
        receive_command(rover, &command_array).expect("expected test to not panic");
        assert_eq!(rover.test_action.expect("expected action to be taken"), TestAction::MoveForward);
    }

    struct TestRover {
        test_action: Option<TestAction>,
    }

    enum TestAction {
        MoveForward
    }

    impl TestRover {
        pub fn new() -> TestRover {
            TestRover {
                test_action: None,
            }
        }
    }

    impl Rover for TestRover {
        fn move_forward(&mut self) -> () {
            self.test_action = Option::from(TestAction::MoveForward);
        }
    }
}