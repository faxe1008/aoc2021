use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Down,
    Up,
}
struct NavigationCommand {
    direction: Direction,
    amount: u32,
}

#[derive(Default)]
struct Submarine {
    horizontal_pos: u32,
    depth: u32,
}

impl NavigationCommand {
    fn new(input_line: String) -> Result<NavigationCommand, &'static str> {
        let components: Vec<&str> = input_line.split_whitespace().collect();
        if components.len() != 2 {
            return Err("Bad command");
        }
        let direction = match components[0] {
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            _ => {
                panic!("Unknown direction");
            }
        };
        let amount: u32 = components[1].parse().unwrap_or(0);
        Ok(NavigationCommand { direction, amount })
    }
}

impl Submarine {
    fn execute_command(&mut self, command: &NavigationCommand) {
        match command.direction {
            Direction::Down => self.depth += command.amount,
            Direction::Forward => self.horizontal_pos += command.amount,
            Direction::Up => self.depth -= command.amount,
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the command input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);
    let commands = reader
        .lines()
        .map(|x| NavigationCommand::new(x.unwrap_or_default()).expect("Error parsing command"))
        .collect::<Vec<NavigationCommand>>();

    let mut submarine = Submarine::default();
    for command in &commands {
        submarine.execute_command(command);
    }
    println!(
        "Submarine final position is {}:{}",
        submarine.horizontal_pos, submarine.depth
    );
}
