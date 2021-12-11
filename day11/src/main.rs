use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Default)]
struct Octopus {
    location: Point,
    energy: u64,
    flashed: bool
}

impl Octopus {
    fn new(energy: u64, x: usize, y: usize) -> Octopus {
        Octopus {
            energy,
            location: Point{x, y},
            flashed: false,
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        let mut neighbours = Vec::new();
        let min_x;
        let max_x;
        let min_y;
        let max_y;
        match self.location.x {
            0 => {
                min_x = 0;
                max_x = self.location.x + 1;
            }
            9 => {
                min_x = self.location.x - 1;
                max_x = 9;
            }
            _ => {
                min_x = self.location.x - 1;
                max_x = self.location.x + 1;
            }
        }
        match self.location.y {
            0 => {
                min_y = 0;
                max_y = self.location.y + 1;
            }
            9 => {
                min_y = self.location.y - 1;
                max_y = 9;
            }
            _ => {
                min_y = self.location.y - 1;
                max_y = self.location.y + 1;
            }
        }
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if x == self.location.x && y == self.location.y {
                    continue;
                }
                neighbours.push(Point { x, y })
            }
        }
        neighbours
    }
}

struct Grid {
    octopodes: [[Octopus; 10]; 10],
}

impl Grid {
    fn increment(&mut self) {
        for x in 0..self.octopodes.len() {
            for y in 0..self.octopodes[x].len() {
                self.octopodes[x][y].energy += 1;
            }
        }
    }

    fn process_flashes(&mut self) -> bool {
        let mut flashed = false;
        for x in 0..self.octopodes.len() {
            for y in 0..self.octopodes[x].len() {
                if self.octopodes[x][y].energy > 9 && !self.octopodes[x][y].flashed {
                    flashed = true;
                    self.octopodes[x][y].flashed = true;
                    for point in self.octopodes[x][y].neighbours() {
                        self.octopodes[point.x][point.y].energy += 1;
                    }
                }
            }
        }
        flashed
    }

    fn reset_flashed(&mut self) -> u64 {
        let mut flashes = 0;
        for x in 0..self.octopodes.len() {
            for y in 0..self.octopodes[x].len() {
                if self.octopodes[x][y].flashed {
                    flashes += 1;
                    self.octopodes[x][y].energy = 0;
                    self.octopodes[x][y].flashed = false;
                }
            }
        }
        flashes
    }

    fn step(&mut self) -> u64 {
        self.increment();
        loop {
            if !self.process_flashes() {
                break;
            }
        }
        self.reset_flashed()
    }
}

impl FromStr for Grid {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut octopodes = [[Octopus::default(); 10]; 10];
        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                octopodes[x][y] = Octopus::new(c.to_string().parse()?,x ,y);
            }
        }
        Ok(Grid{octopodes})
    }
}

fn part1(input: &str) {
    let mut grid = Grid::from_str(input).expect("invalid grid input");
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += grid.step();
    }
    println!("part1: {}", total_flashes)
}

fn part2(input: &str) {
    let mut grid = Grid::from_str(input).expect("invalid grid input");
    let mut step = 0;
    let mut all_flashed = false;
    while !all_flashed {
        step += 1;
        if grid.step() == 100 {
            all_flashed = true;
        }
    }
    println!("part2: {}", step)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
