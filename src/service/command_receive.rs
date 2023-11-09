use crate::domain::rust_rover_aggregate::*;
fn receive_command<'a>(rover: &'a mut impl RoverActions, command_array: &[char]) -> Result<&'a mut impl RoverActions, CommandReceiveError> {
    rover.move_forward();
    Ok(rover)
}

#[derive(Debug)]
struct CommandReceiveError;



#[cfg(test)]
mod command_receive_service_tests {
    use crate::domain::rust_rover_aggregate::*;
    use crate::service::command_receive::receive_command;

    #[test]
    fn rover_can_receive_a_character_array_of_commands() {
        let mut rover: TestRover = TestRover::new();
        let command_array = ['T', 'O'];
        match receive_command(&mut rover, &command_array) {
            Ok(_) => (),
            Err(_) => panic!("Expected test to not panic"),
        }
    }

    #[test]
    fn rover_moves_forward_when_given_a_forward_command() {
        let mut rover: TestRover = TestRover::new();
        let command_array = ['f'];
        receive_command(&mut rover, &command_array).expect("expected test to not panic");
        assert_eq!(rover.test_action.expect("expected action to be taken"), TestAction::MoveForward);
    }

    struct TestRover {
        test_action: Option<TestAction>,
    }

    #[derive(Debug, PartialEq)]
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

    impl RoverActions for TestRover {
        fn move_forward(&mut self) -> () {
            self.test_action = Option::from(TestAction::MoveForward);
        }
    }
}