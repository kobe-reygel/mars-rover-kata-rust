use std::io;
use crate::domain::rust_rover_aggregate::{Coordinate, Direction, RustRover};
use crate::service::command_receive::receive_command;

mod domain;
mod service;

fn main() {
    let mut rover: RustRover = RustRover::new(Coordinate::from(0, 0), Direction::North);
    println!("{:?}", rover);
    loop {
        println!("Give commands to the mars rover");

        let mut commands = String::new();

        io::stdin()
            .read_line(&mut commands)
            .expect("Failed to read commands");

        let command_array: Vec<char> = commands
            .trim()
            .chars()
            .collect();
        receive_command(&mut rover, &command_array).unwrap();

        println!("{:?}", rover);
    }
}
