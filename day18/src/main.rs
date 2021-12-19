use itertools::Itertools;
use std::cmp::max;
use std::fs;

#[derive(Clone)]
enum Snailfish {
    Regular(u64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl From<&str> for Snailfish {
    fn from(input: &str) -> Self {
        if !input.starts_with("[") {
            return Snailfish::Regular(input.parse().unwrap());
        }

        let mut idx = 0;
        let mut depth = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {
                    if depth == 1 {
                        idx = i;
                        break;
                    }
                }
                _ => (),
            }
        }

        let left = Snailfish::from(&input[1..idx]);
        let right = Snailfish::from(&input[idx + 1..input.len() - 1]);

        Snailfish::Pair(Box::new(left), Box::new(right))
    }
}

impl Snailfish {
    fn magnitude(&self) -> u64 {
        match self {
            Snailfish::Regular(val) => *val,
            Snailfish::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn is_regular(&self) -> bool {
        match self {
            Snailfish::Regular(_) => true,
            Snailfish::Pair(_, _) => false,
        }
    }

    fn get_val(&self) -> u64 {
        match self {
            Snailfish::Regular(val) => *val,
            Snailfish::Pair(_, _) => unreachable!(),
        }
    }

    fn explode(&mut self, current_depth: u64) -> (Option<u64>, Option<u64>, bool) {
        match self {
            Snailfish::Regular(_) => (None, None, false),
            Snailfish::Pair(left, right) => {
                if current_depth >= 4 {
                    if left.is_regular() && right.is_regular() {
                        return (Some(left.get_val()), Some(right.get_val()), true);
                    }
                    panic!()
                }

                let (left_val, right_val, exploded) = left.explode(current_depth + 1);
                if exploded {
                    if left_val.is_some() && right_val.is_some() {
                        *left = Box::new(Snailfish::Regular(0));
                    }
                    if let Some(val) = right_val {
                        right.add_left(val);
                    }
                    return (left_val, None, true);
                }

                let (left_val, right_val, exploded) = right.explode(current_depth + 1);
                if exploded {
                    if left_val.is_some() && right_val.is_some() {
                        *right = Box::new(Snailfish::Regular(0));
                    }
                    if let Some(val) = left_val {
                        left.add_right(val);
                    }
                    return (None, right_val, true);
                }

                (None, None, false)
            }
        }
    }

    fn add_left(&mut self, val: u64) -> bool {
        match self {
            Snailfish::Regular(v) => {
                *v += val;
                true
            }
            Snailfish::Pair(left, _) => left.add_left(val),
        }
    }

    fn add_right(&mut self, val: u64) -> bool {
        match self {
            Snailfish::Regular(v) => {
                *v += val;
                true
            }
            Snailfish::Pair(_, right) => right.add_right(val),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Regular(val) => {
                if *val >= 10 {
                    *self = Snailfish::Pair(
                        Box::new(Snailfish::Regular(*val / 2)),
                        Box::new(Snailfish::Regular(*val / 2 + *val % 2)),
                    );
                    return true;
                }
                return false;
            }
            Snailfish::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn reduce(&mut self) {
        loop {
            if !(self.explode(0).2 || self.split()) {
                break;
            }
        }
    }

    fn add(left: Snailfish, right: Snailfish) -> Snailfish {
        let mut new = Snailfish::Pair(Box::new(left), Box::new(right));
        new.reduce();
        new
    }
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let mut snailfish = Snailfish::from(lines.next().unwrap());
    for line in lines {
        snailfish = Snailfish::add(snailfish, Snailfish::from(line));
    }
    println!("part1: {}", snailfish.magnitude())
}

fn part2(input: &str) {
    let snailfish: Vec<Snailfish> = input.lines().map(Snailfish::from).collect();
    let mut max_magnitude = 0;
    for permutation in snailfish.iter().permutations(2) {
        max_magnitude = max(
            max_magnitude,
            Snailfish::add(permutation[0].clone(), permutation[1].clone()).magnitude(),
        );
        max_magnitude = max(
            max_magnitude,
            Snailfish::add(permutation[1].clone(), permutation[0].clone()).magnitude(),
        );
    }
    println!("part2: {}", max_magnitude)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
