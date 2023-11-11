use crate::domain::rust_rover_aggregate::*;
pub fn receive_command<'a>(rover: &'a mut impl RoverActions, command_array: &[char]) -> Result<&'a mut impl RoverActions, CommandReceiveError> {
    for command in command_array {
        if *command == 'f' {
            rover.move_forward();
        } else if *command == 'b' {
            rover.move_backward();
        }
    }
    Ok(rover)

}

#[derive(Debug)]
pub struct CommandReceiveError;

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
        assert_eq!(rover.test_actions, vec![TestAction::MoveForward]);
    }

    #[test]
    fn rover_moves_backward_when_given_a_backward_command() {
        let mut rover: TestRover = TestRover::new();
        let command_array = ['b'];
        receive_command(&mut rover, &command_array).expect("expected test to not panic");
        assert_eq!(rover.test_actions, vec![TestAction::MoveBackward]);
    }

    #[test]
    fn rover_can_process_multiple_commands_correctly() {
        let mut rover: TestRover = TestRover::new();
        let command_array = ['b', 'f'];
        receive_command(&mut rover, &command_array).expect("expected test to not panic");
        assert_eq!(rover.test_actions, vec![TestAction::MoveBackward, TestAction::MoveForward]);
    }

    struct TestRover {
        test_actions: Vec<TestAction>,
    }

    #[derive(Debug, PartialEq)]
    enum TestAction {
        MoveForward,
        MoveBackward,
    }

    impl TestRover {
        pub fn new() -> TestRover {
            TestRover {
                test_actions: vec![],
            }
        }
    }

    impl RoverActions for TestRover {
        fn move_forward(&mut self) -> () {
            self.test_actions.push(TestAction::MoveForward);
        }

        fn move_backward(&mut self) -> () {
            self.test_actions.push(TestAction::MoveBackward);
        }
    }
}