use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Hash)]
enum Axis {
    X,
    Y,
}

impl FromStr for Axis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
struct Fold {
    axis: Axis,
    line: usize,
}

fn perform_fold(grid: &[[bool; 1400]; 1400], fold: Fold) -> [[bool; 1400]; 1400] {
    let local_grid = grid.clone();
    let mut new_grid = [[false; 1400]; 1400];
    match fold.axis {
        Axis::X => {
            for x in 0..fold.line {
                for y in 0..local_grid[x].len() {
                    new_grid[fold.line-1-x][y] = local_grid[fold.line+1+x][y];
                }
            }
            for x in 0..fold.line {
                for y in 0..local_grid[x].len() {
                    if !new_grid[x][y] {
                        new_grid[x][y] = local_grid[x][y];
                    }
                }
            }
        }
        Axis::Y => {
            for x in 0..local_grid.len() {
                for y in 0..fold.line {
                    new_grid[x][fold.line-1-y] = local_grid[x][fold.line+1+y];
                }
            }
            for x in 0..local_grid.len() {
                for y in 0..fold.line {
                    if !new_grid[x][y] {
                        new_grid[x][y] = local_grid[x][y];
                    }
                }
            }
        }
    }
    new_grid
}

fn count_dots(grid: &[[bool; 1400]; 1400]) -> u64 {
    let mut count = 0;
    for row in grid {
        for cell in row {
            if *cell {
                count += 1;
            }
        }
    }
    count
}

fn write_grid(grid: &[[bool; 1400]; 1400]) {
    let mut file = File::create("output.txt").unwrap();
    for i in 0..grid.len() {
        let mut output = String::new();
        for j in 0..grid[i].len() {
            match grid[j][i] {
                true => output.push('#'),
                false => output.push('.')
            };
        }
        output.push('\n');
        file.write_all(output.as_ref());
    }
    file.flush();
}

fn part1(input: &str) {
    let mut grid: [[bool; 1400]; 1400] = [[false; 1400]; 1400];
    let mut folds = Vec::new();
    for line in input.lines() {
        if line.contains(",") {
            let mut splits = line.split(",");
            let x = splits.next().unwrap().parse::<usize>().unwrap();
            let y = splits.next().unwrap().parse::<usize>().unwrap();
            grid[x][y] = true;
        } else if line.contains("fold along ") {
            let trimmed = line.replace("fold along ", "");
            let mut splits = trimmed.split("=");
            let axis = Axis::from_str(splits.next().unwrap()).unwrap();
            let line = splits.next().unwrap().parse::<usize>().unwrap();
            folds.push(Fold { axis, line });
        }
    }
    let grid = perform_fold(&grid, folds[0]);
    println!("part1: {}", count_dots(&grid))
}

fn part2(input: &str) {
    let mut grid: [[bool; 1400]; 1400] = [[false; 1400]; 1400];
    let mut folds = Vec::new();
    for line in input.lines() {
        if line.contains(",") {
            let mut splits = line.split(",");
            let x = splits.next().unwrap().parse::<usize>().unwrap();
            let y = splits.next().unwrap().parse::<usize>().unwrap();
            grid[x][y] = true;
        } else if line.contains("fold along ") {
            let trimmed = line.replace("fold along ", "");
            let mut splits = trimmed.split("=");
            let axis = Axis::from_str(splits.next().unwrap()).unwrap();
            let line = splits.next().unwrap().parse::<usize>().unwrap();
            folds.push(Fold { axis, line });
        }
    }
    for fold in folds {
        grid = perform_fold(&grid, fold);
    }
    write_grid(&grid);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
