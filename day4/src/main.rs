use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Cell {
    val: usize,
    drawn: bool,
}

impl Cell {
    pub fn new(val: usize, drawn: bool) -> Cell {
        Cell { val, drawn }
    }
}
impl FromStr for Cell {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val: usize = s.parse()?;
        Ok(Cell::new(val, false))
    }
}

#[derive(Clone)]
struct Board {
    id: usize,
    grid: Vec<Vec<Cell>>,
}

impl Board {
    pub fn mark(&mut self, val: usize) {
        for row in self.grid.iter_mut() {
            for mut cell in row {
                if cell.val == val {
                    cell.drawn = true;
                }
            }
        }
    }

    pub fn check_for_win(&self) -> bool {
        for row in &self.grid {
            if row.iter().all(|x| x.drawn) {
                return true;
            }
        }
        for i in 0..self.grid.len() {
            let mut new_vec: Vec<Cell> = Vec::new();
            for j in 0..self.grid[i].len() {
                new_vec.push(self.grid[j][i]);
            }
            if new_vec.iter().all(|x| x.drawn) {
                return true;
            }
        }
        false
    }

    pub fn get_unmarked(&self) -> Vec<usize> {
        let mut unmarked = Vec::new();
        for row in &self.grid {
            for cell in row {
                if !cell.drawn {
                    unmarked.push(cell.val);
                }
            }
        }
        unmarked
    }
}

fn get_draws(input: &str) -> Vec<usize> {
    input
        .split("\n")
        .next()
        .expect("should be at least one line of input")
        .split(",")
        .map(|x| x.parse().expect("should be an integer"))
        .collect()
}

fn build_boards(input: &str) -> Vec<Board> {
    let mut current_grid: Vec<Vec<Cell>> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut i = 0;
    for line in input.split("\n").skip(2) {
        if line.len() == 0 {
            let new_board = Board {
                id: i,
                grid: current_grid.clone(),
            };
            boards.push(new_board);
            current_grid = Vec::new();
            i += 1;
        } else {
            let mut row: Vec<Cell> = Vec::new();
            for item in line.split_ascii_whitespace() {
                let cell = Cell::from_str(item).expect("should be a valid number");
                row.push(cell)
            }
            current_grid.push(row);
        }
    }
    boards
}

fn part1(draws: &Vec<usize>, boards: &mut Vec<Board>) {
    let mut score: usize = 0;
    'draws: for draw in draws {
        for board in boards.iter_mut() {
            board.mark(*draw);
            if board.check_for_win() {
                let sum: usize = board.get_unmarked().iter().sum();
                score = draw * sum;
                break 'draws;
            }
        }
    }
    println!("p1 score: {}", score);
}

fn part2(draws: &Vec<usize>, boards: &mut Vec<Board>) {
    let mut winning_boards: HashSet<usize> = HashSet::new();
    let num_boards = boards.len();
    let mut score: usize = 0;
    'draws: for draw in draws {
        for board in boards.iter_mut() {
            board.mark(*draw);
            if board.check_for_win() {
                if winning_boards.len() == num_boards - 1 && !winning_boards.contains(&board.id) {
                    let sum: usize = board.get_unmarked().iter().sum();
                    score = draw * sum;
                    break 'draws;
                } else {
                    winning_boards.insert(board.id);
                }
            }
        }
    }
    println!("p2 score: {}", score);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    let draws = get_draws(&input);
    let mut boards = build_boards(&input);
    part1(&draws, &mut boards);
    part2(&draws, &mut boards);
}
