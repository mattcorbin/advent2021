use std::fs;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
struct Command {
    direction: Direction,
    distance: isize,
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut retval: Vec<Command> = Vec::new();
    for item in input.split("\n") {
        let splits: Vec<&str> = item.split(" ").collect();
        let command = Command{
            direction: splits[0].parse().unwrap(),
            distance: splits[1].parse().unwrap(),
        };
        retval.push(command);
    }
    retval
}

fn part1(commands: &Vec<Command>) {
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command.direction {
            Direction::Forward => {horizontal += command.distance}
            Direction::Down => {depth += command.distance}
            Direction::Up => {depth -= command.distance}
        }
    }
    println!("part1: {}", horizontal*depth)
}

fn part2(commands: &Vec<Command>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command.direction {
            Direction::Forward => {
                horizontal += command.distance;
                depth += command.distance * aim;
            }
            Direction::Down => {
                aim += command.distance;
            }
            Direction::Up => {
                aim -= command.distance;
            }
        }
    }
    println!("part2: {}", horizontal*depth)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    let commands = parse_input(&input);
    part1(&commands);
    part2(&commands);
}
