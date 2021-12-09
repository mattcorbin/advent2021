use std::fs;
use itertools::Itertools;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
struct Point {
    x: usize,
    y: usize,
    value: u64,
    basin: bool,
    sized: bool,
}

impl Point {
    fn new(x: usize, y: usize, value: u64) -> Point {
        Point {
            x,
            y,
            value,
            basin: value != 9,
            sized: false,
        }
    }
}

fn part1(input: &str) {
    let mut grid: [[Point; 100]; 100] = [[Point::default(); 100]; 100];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = Point::new(i, j, c.to_string().parse().unwrap());
        }
    }
    let mut low_points: Vec<u64> = Vec::new();
    for i in 0..100 {
        for j in 0..100 {
            let mut min_i = i;
            let mut max_i = i;
            let mut min_j = j;
            let mut max_j = j;
            if i != 0 {
                min_i = i - 1;
            }
            if j != 0 {
                min_j = j - 1;
            }
            if i != 99 {
                max_i = i + 1;
            }
            if j != 99 {
                max_j = j + 1;
            }
            let mut lowest = true;
            for test_i in min_i..=max_i {
                for test_j in min_j..=max_j {
                    if test_i == i && test_j == j {
                        continue;
                    }
                    if !(grid[i][j].value < grid[test_i][test_j].value) {
                        lowest = false;
                    }
                }
            }
            if lowest {
                low_points.push(grid[i][j].value);
            }
        }
    }
    let sum: u64 = low_points.iter().map(|x| *x + 1).sum();
    println!("part1: {}", sum)
}

fn size_basin(grid: &mut [[Point; 100]; 100], x: isize, y: isize) -> u64 {
    if x > 99 || x < 0 || y > 99 || y < 0 {
        return 0;
    } else if grid[x as usize][y as usize].sized == true || !grid[x as usize][y as usize].basin {
        return 0;
    }
    grid[x as usize][y as usize].sized = true;
    1 + size_basin(grid, x + 1, y)
        + size_basin(grid, x, y + 1)
        + size_basin(grid, x - 1, y)
        + size_basin(grid, x, y - 1)
}

fn part2(input: &str) {
    let mut grid: [[Point; 100]; 100] = [[Point::default(); 100]; 100];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = Point::new(i, j, c.to_string().parse().unwrap());
        }
    }
    let mut basin_sizes = Vec::new();
    for i in 0..100 {
        for j in 0..100 {
            let size = size_basin(&mut grid, i, j);
            if size != 0 {
                basin_sizes.push(size);
            }
        }
    }
    let sorted_sizes: Vec<u64> = basin_sizes.into_iter().sorted().rev().collect();
    println!("part2: {}", sorted_sizes[0] * sorted_sizes[1] * sorted_sizes[2])
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
