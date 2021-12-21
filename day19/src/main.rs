use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::ops::{Add, Sub};
use itertools::Itertools;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Position {
    fn from(input: &str) -> Self {
        let mut splits = input.split(",");

        let x = splits.next().unwrap().parse().unwrap();
        let y = splits.next().unwrap().parse().unwrap();
        let z = splits.next().unwrap().parse().unwrap();
        Position { x, y, z }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

fn get_rotation_by_index(v: &Position, index: usize) -> Position {
    return match index {
        0 => Position {
            x: v.z,
            y: v.y,
            z: -v.x,
        },
        1 => Position {
            x: -v.z,
            y: -v.y,
            z: -v.x,
        },
        2 => Position {
            x: -v.z,
            y: -v.x,
            z: v.y,
        },
        3 => Position {
            x: -v.z,
            y: v.x,
            z: -v.y,
        },
        4 => Position {
            x: -v.z,
            y: v.y,
            z: v.x,
        },
        5 => Position {
            x: -v.y,
            y: -v.z,
            z: v.x,
        },
        6 => Position {
            x: -v.y,
            y: -v.x,
            z: -v.z,
        },
        7 => Position {
            x: -v.y,
            y: v.x,
            z: v.z,
        },
        8 => Position {
            x: -v.y,
            y: v.z,
            z: -v.x,
        },
        9 => Position {
            x: -v.x,
            y: -v.z,
            z: -v.y,
        },
        10 => Position {
            x: -v.x,
            y: -v.y,
            z: v.z,
        },
        11 => Position {
            x: -v.x,
            y: v.y,
            z: -v.z,
        },
        12 => Position {
            x: -v.x,
            y: v.z,
            z: v.y,
        },
        13 => Position {
            x: v.x,
            y: -v.z,
            z: v.y,
        },
        14 => Position {
            x: v.x,
            y: -v.y,
            z: -v.z,
        },
        15 => Position {
            x: v.x,
            y: v.y,
            z: v.z,
        },
        16 => Position {
            x: v.x,
            y: v.z,
            z: -v.y,
        },
        17 => Position {
            x: v.y,
            y: -v.z,
            z: -v.x,
        },
        18 => Position {
            x: v.y,
            y: -v.x,
            z: v.z,
        },
        19 => Position {
            x: v.y,
            y: v.x,
            z: -v.z,
        },
        20 => Position {
            x: v.y,
            y: v.z,
            z: v.x,
        },
        21 => Position {
            x: v.z,
            y: -v.y,
            z: v.x,
        },
        22 => Position {
            x: v.z,
            y: -v.x,
            z: -v.y,
        },
        23 => Position {
            x: v.z,
            y: v.x,
            z: v.y,
        },
        _ => Position { x: 0, y: 0, z: 0 },
    };
}

fn has_overlaps(
    scanner1: &Vec<Position>,
    scanner2: &Vec<Position>,
    offset: &Position,
    orientation: usize,
) -> bool {
    let mut same = 0;
    for pos1 in scanner1 {
        for pos2 in scanner2 {
            let shifted = *offset + get_rotation_by_index(pos2, orientation);
            if *pos1 == shifted {
                same += 1;
                if same >= 12 {
                    return true;
                }
                break;
            }
        }
    }
    false
}

fn find_overlaps(scanner1: &Vec<Position>, scanner2: &Vec<Position>) -> Option<(Position, usize)> {
    for i in 0..scanner1.len() {
        let pos1 = scanner1[i];
        for j in 0..scanner2.len() {
            let pos2 = scanner2[j];
            for orientation in 0..24 {
                let offset = pos1 - get_rotation_by_index(&pos2, orientation);
                if has_overlaps(scanner1, scanner2, &offset, orientation) {
                    return Some((offset, orientation));
                }
            }
        }
    }
    None
}

fn transform(scanner: &Vec<Position>, offset: &Position, orientation: usize) -> Vec<Position> {
    let mut transformed_scanner = Vec::new();
    for position in scanner {
        transformed_scanner.push(*offset + get_rotation_by_index(position, orientation));
    }
    transformed_scanner
}

fn get_scanners(input: &str) -> Vec<Vec<Position>> {
    let mut scanners = Vec::new();
    let mut current_scanner = Vec::new();
    for line in input.lines() {
        if line.contains("scanner") {
            // do nothing
        } else if line.contains(",") {
            current_scanner.push(Position::from(line));
        } else {
            scanners.push(current_scanner);
            current_scanner = Vec::new();
        }
    }
    scanners.push(current_scanner);
    scanners
}

fn manhattan(p: &Position, q: &Position) -> i64 {
    (p.x - q.x).abs() + (p.y - q.y).abs() + (p.z - q.z).abs()
}

fn part1(input: &str) {
    let mut scanners = get_scanners(input);
    let mut orientation = Vec::new();
    orientation.resize(scanners.len(), 0);
    let mut is_oriented = Vec::new();
    is_oriented.resize(scanners.len(), false);
    is_oriented[0] = true;

    loop {
        let mut done = true;
        for i in 0..scanners.len() - 1 {
            for j in i + 1..scanners.len() {
                if i == j || is_oriented[i] == is_oriented[j] {
                    continue;
                }
                done = false;
                let oriented;
                let unoriented;
                match is_oriented[i] {
                    true => {
                        oriented = i;
                        unoriented = j;
                    }
                    false => {
                        oriented = j;
                        unoriented = i;
                    }
                }
                if let Some((offset, orientation)) =
                    find_overlaps(&scanners[oriented], &scanners[unoriented])
                {
                    scanners[unoriented] = transform(&scanners[unoriented], &offset, orientation);
                    is_oriented[unoriented] = true;
                }
            }
        }
        if done {
            break;
        }
    }

    let mut all_beacons = HashSet::new();
    for scanner in scanners {
        for position in scanner {
            all_beacons.insert(position);
        }
    }

    println!("part1: {}", all_beacons.len());
}

fn part2(input: &str) {
    let mut scanners = get_scanners(input);
    let mut offsets = Vec::new();
    offsets.resize(scanners.len(), Position { x: 0, y: 0, z: 0 });
    let mut orientation = Vec::new();
    orientation.resize(scanners.len(), 0);
    let mut is_oriented = Vec::new();
    is_oriented.resize(scanners.len(), false);
    is_oriented[0] = true;

    loop {
        let mut done = true;
        for i in 0..scanners.len() - 1 {
            for j in i + 1..scanners.len() {
                if i == j || is_oriented[i] == is_oriented[j] {
                    continue;
                }
                done = false;
                let oriented;
                let unoriented;
                match is_oriented[i] {
                    true => {
                        oriented = i;
                        unoriented = j;
                    }
                    false => {
                        oriented = j;
                        unoriented = i;
                    }
                }
                if let Some((offset, orientation)) =
                find_overlaps(&scanners[oriented], &scanners[unoriented])
                {
                    offsets[unoriented] = offset;
                    scanners[unoriented] = transform(&scanners[unoriented], &offset, orientation);
                    is_oriented[unoriented] = true;
                }
            }
        }
        if done {
            break;
        }
    }

    let mut max_distance = i64::MIN;

    for permutation in offsets.iter().permutations(2) {
        max_distance = max(max_distance, manhattan(permutation[0], permutation[1]));
    }

    println!("part2: {}", max_distance)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
