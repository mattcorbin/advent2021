use std::cmp::{max, min};
use std::fs;

struct Instruction {
    state: bool,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl Instruction {
    fn is_small(&self) -> bool {
        self.x_min >= -50
            && self.x_max <= 50
            && self.y_min >= -50
            && self.y_max <= 50
            && self.z_min >= -50
            && self.z_max <= 50
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let state;
        let x_min;
        let x_max;
        let y_min;
        let y_max;
        let z_min;
        let z_max;

        let mut splits = input.split(" ");
        match splits.next().unwrap() {
            "on" => state = true,
            "off" => state = false,
            _ => unreachable!(),
        }
        let mut bounds = splits.next().unwrap().split(",");

        let x = bounds.next().unwrap().replace("x=", "");
        let x = x.split("..").collect::<Vec<&str>>();
        let x0 = x[0].parse::<i64>().unwrap();
        let x1 = x[1].parse::<i64>().unwrap();
        match x0 < x1 {
            true => {
                x_min = x0;
                x_max = x1;
            }
            false => {
                x_min = x1;
                x_max = x0;
            }
        }

        let y = bounds.next().unwrap().replace("y=", "");
        let y = y.split("..").collect::<Vec<&str>>();
        let y0 = y[0].parse::<i64>().unwrap();
        let y1 = y[1].parse::<i64>().unwrap();
        match y0 < y1 {
            true => {
                y_min = y0;
                y_max = y1;
            }
            false => {
                y_min = y1;
                y_max = y0;
            }
        }

        let z = bounds.next().unwrap().replace("z=", "");
        let z = z.split("..").collect::<Vec<&str>>();
        let z0 = z[0].parse::<i64>().unwrap();
        let z1 = z[1].parse::<i64>().unwrap();
        match z0 < z1 {
            true => {
                z_min = z0;
                z_max = z1;
            }
            false => {
                z_min = z1;
                z_max = z0;
            }
        }

        Instruction {
            state,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }
}

fn count_on(grid: &[[[bool; 200]; 200]; 200]) -> usize {
    let mut count = 0;
    for x in 0..200 {
        for y in 0..200 {
            for z in 0..200 {
                if grid[x][y][z] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part1(input: &str) {
    let mut grid = [[[false; 200]; 200]; 200];
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Instruction::from(line));
    }
    for instruction in instructions {
        if instruction.is_small() {
            for x in instruction.x_min..=instruction.x_max {
                for y in instruction.y_min..=instruction.y_max {
                    for z in instruction.z_min..=instruction.z_max {
                        grid[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] =
                            instruction.state;
                    }
                }
            }
        }
    }
    println!("part1: {}", count_on(&grid))
}

#[derive(Debug)]
struct Cube {
    // 6------7.
    // |`.    | `.
    // |  `2--+---3
    // |   |  |   |              z
    // 4---+--5   |          y   |
    //  `. |    `.|           `. |
    //    `0------1             `+-----x
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
    state: bool,
    sub_cubes: Vec<Cube>,
}

impl Cube {
    fn intersects(&self, other: &Self) -> bool {
        self.x_max >= other.x_min
            && self.x_min <= other.x_max
            && self.y_max >= other.y_min
            && self.y_min <= other.y_max
            && self.z_max >= other.z_min
            && self.z_min <= other.z_max
    }

    fn check_intersect_and_update(&mut self, other: &Self) {
        if self.intersects(other) {
            let x_min = max(self.x_min, other.x_min);
            let x_max = min(self.x_max, other.x_max);
            let y_min = max(self.y_min, other.y_min);
            let y_max = min(self.y_max, other.y_max);
            let z_min = max(self.z_min, other.z_min);
            let z_max = min(self.z_max, other.z_max);
            let mut new_cube = Cube{
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
                state: other.state,
                sub_cubes: Vec::new(),
            };
            for cube in self.sub_cubes.iter_mut() {
                cube.check_intersect_and_update(&new_cube);
            }
            if new_cube.state == self.state {
                new_cube.state = !self.state;
            }
            self.sub_cubes.push(new_cube);
        }
    }

    fn active(&self) -> i64 {
        let l = self.x_max - self.x_min + 1;
        let w = self.y_max - self.y_min + 1;
        let h = self.z_max - self.z_min + 1;
        let mut active = l * w * h;
        if !self.state {
            active *= -1;
        }
        for cube in &self.sub_cubes {
            active += cube.active();
        }
        active
    }
}

impl From<Instruction> for Cube {
    fn from(i: Instruction) -> Self {
        Cube {
            x_min: i.x_min,
            x_max: i.x_max,
            y_min: i.y_min,
            y_max: i.y_max,
            z_min: i.z_min,
            z_max: i.z_max,
            state: i.state,
            sub_cubes: Vec::new(),
        }
    }
}

fn part2(input: &str) {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Instruction::from(line));
    }
    let mut cubes: Vec<Cube> = Vec::new();
    for instruction in instructions {
        let new_cube = Cube::from(instruction);
        for cube in cubes.iter_mut() {
            cube.check_intersect_and_update(&new_cube);
        }
        cubes.push(new_cube);
    }
    let mut sum = 0;
        for cube in cubes {
            if cube.state {
                sum += cube.active();
            }
        }

    println!("part2: {}", sum)
}

fn main() {
    let test = fs::read_to_string("test.txt").expect("Unable to read input");
    part1(&test);
    part2(&test);
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
