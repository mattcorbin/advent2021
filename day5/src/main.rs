use std::cmp::{max, min};
use std::fs;

fn parse_line(input: &str) -> ((usize, usize), (usize, usize)) {
    let s1 = input.split(" -> ").collect::<Vec<&str>>();
    let from = s1[0].split(",").collect::<Vec<&str>>();
    let to = s1[1].split(",").collect::<Vec<&str>>();

    ((from[0].parse().unwrap(), from[1].parse().unwrap()), (to[0].parse().unwrap(), to[1].parse().unwrap()))
}

fn part1(input: &str) {
    let mut overlaps: usize = 0;
    let mut plane: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    for line in input.lines() {
        let (start, end) = parse_line(line);
        let x_same = start.0 == end.0;
        let y_same = start.1 == end.1;
        if x_same || y_same {
            if x_same {
                let s = min(start.1, end.1);
                let e = max(start.1, end.1);
                for y in s..=e {
                    plane[start.0][y] += 1
                }
            } else {
                let s = min(start.0, end.0);
                let e = max(start.0, end.0);
                for x in s..=e {
                    plane[x][start.1] += 1
                }
            }
        }
    }

    for x in 0..plane.len() {
        for y in 0..plane[x].len() {
            if plane[x][y] > 1 {
                overlaps += 1;
            }
        }
    }
    println!("part1: {}", overlaps)
}

fn part2(input: &str) {
    let mut overlaps: usize = 0;
    let mut plane: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    for line in input.lines() {
        let (start, end) = parse_line(line);
        let x_same = start.0 == end.0;
        let y_same = start.1 == end.1;
        if x_same || y_same {
            if x_same {
                let s = min(start.1, end.1);
                let e = max(start.1, end.1);
                for y in s..=e {
                    plane[start.0][y] += 1
                }
            } else {
                let s = min(start.0, end.0);
                let e = max(start.0, end.0);
                for x in s..=e {
                    plane[x][start.1] += 1
                }
            }
        } else {
            if start.0 > end.0 && start.1 > end.1 {
                for i in 0..=start.0-end.0 {
                    plane[start.0-i][start.1-i] += 1
                }
            } else if end.0 > start.0 && end.1 > start.1  {
                for i in 0..=end.0-start.0 {
                    plane[start.0+i][start.1+i] += 1
                }
            } else if start.0 > end.0 && end.1 > start.1 {
                for i in 0..=start.0-end.0 {
                    plane[start.0-i][start.1+i] += 1
                }
            } else {
                for i in 0..=end.0-start.0 {
                    plane[start.0+i][start.1-i] += 1
                }
            }
        }
    }

    for x in 0..plane.len() {
        for y in 0..plane[x].len() {
            if plane[x][y] > 1 {
                overlaps += 1;
            }
        }
    }
    println!("part2: {}", overlaps)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
