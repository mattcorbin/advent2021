use std::fs;

fn count_lights(grid: &[[u8; 1000]; 1000]) -> u64 {
    let mut count = 0;
    for row in grid {
        for cell in row {
            if *cell == 1 {
                count += 1;
            }
        }
    }
    count
}

fn part1(input: &str) {
    let mut grid = [[0u8; 1000]; 1000];
    let mut lines = input.lines();
    let key = lines.next().unwrap();
    for (x, line) in lines.enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                grid[x+450][y+450] = 1;
            }
        }
    }
    let mut new_grid = [[1u8; 1000]; 1000];
    for x in 350..650 {
        for y in 350..650 {
            let mut lookup = String::new();
            lookup.push_str(&grid[y-1][x-1].to_string());
            lookup.push_str(&grid[y-1][x].to_string());
            lookup.push_str(&grid[y-1][x+1].to_string());
            lookup.push_str(&grid[y][x-1].to_string());
            lookup.push_str(&grid[y][x].to_string());
            lookup.push_str(&grid[y][x+1].to_string());
            lookup.push_str(&grid[y+1][x-1].to_string());
            lookup.push_str(&grid[y+1][x].to_string());
            lookup.push_str(&grid[y+1][x+1].to_string());
            let key_idx = usize::from_str_radix(&lookup, 2).unwrap();
            let new_value = key.as_bytes()[key_idx] as char;
            if new_value == '#' {
                new_grid[y][x] = 1;
            } else {
                new_grid[y][x] = 0;
            }
        }
    }
    grid = new_grid.clone();
    let mut new_grid = [[0u8; 1000]; 1000];
    for x in 300..700 {
        for y in 300..700 {
            let mut lookup = String::new();
            lookup.push_str(&grid[y-1][x-1].to_string());
            lookup.push_str(&grid[y-1][x].to_string());
            lookup.push_str(&grid[y-1][x+1].to_string());
            lookup.push_str(&grid[y][x-1].to_string());
            lookup.push_str(&grid[y][x].to_string());
            lookup.push_str(&grid[y][x+1].to_string());
            lookup.push_str(&grid[y+1][x-1].to_string());
            lookup.push_str(&grid[y+1][x].to_string());
            lookup.push_str(&grid[y+1][x+1].to_string());
            let key_idx = usize::from_str_radix(&lookup, 2).unwrap();
            let new_value = key.as_bytes()[key_idx] as char;
            if new_value == '#' {
                new_grid[y][x] = 1;
            } else {
                new_grid[y][x] = 0;
            }
        }
    }
    grid = new_grid.clone();
    println!("part1: {}", count_lights(&grid));
}

fn part2(input: &str) {
    let mut grid = [[0u8; 1000]; 1000];
    let mut lines = input.lines();
    let key = lines.next().unwrap();
    for (x, line) in lines.enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                grid[x+450][y+450] = 1;
            }
        }
    }
    let mut base = 1u8;
    for _ in 0..50 {
        let mut new_grid = [[base; 1000]; 1000];
        for x in 100..900 {
            for y in 100..900 {
                let mut lookup = String::new();
                lookup.push_str(&grid[y-1][x-1].to_string());
                lookup.push_str(&grid[y-1][x].to_string());
                lookup.push_str(&grid[y-1][x+1].to_string());
                lookup.push_str(&grid[y][x-1].to_string());
                lookup.push_str(&grid[y][x].to_string());
                lookup.push_str(&grid[y][x+1].to_string());
                lookup.push_str(&grid[y+1][x-1].to_string());
                lookup.push_str(&grid[y+1][x].to_string());
                lookup.push_str(&grid[y+1][x+1].to_string());
                let key_idx = usize::from_str_radix(&lookup, 2).unwrap();
                let new_value = key.as_bytes()[key_idx] as char;
                if new_value == '#' {
                    new_grid[y][x] = 1;
                } else {
                    new_grid[y][x] = 0;
                }
            }
        }
        grid = new_grid.clone();
        base ^= 1;
    }
    println!("part2: {}", count_lights(&grid));
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
