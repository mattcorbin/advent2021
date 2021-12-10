use itertools::Itertools;
use std::fs;

fn part1(input: &str) {
    let mut points = 0;
    for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),
                ')' => {
                    if let Some(val) = stack.pop() {
                        if val != '(' {
                            points += 3;
                        }
                    }
                }
                ']' => {
                    if let Some(val) = stack.pop() {
                        if val != '[' {
                            points += 57;
                        }
                    }
                }
                '}' => {
                    if let Some(val) = stack.pop() {
                        if val != '{' {
                            points += 1197;
                        }
                    }
                }
                '>' => {
                    if let Some(val) = stack.pop() {
                        if val != '<' {
                            points += 25137;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    println!("part1: {}", points)
}

fn part2(input: &str) {
    let mut scores = Vec::new();
    'lines: for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut score: u64 = 0;
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),
                ')' => {
                    if let Some(val) = stack.pop() {
                        if val != '(' {
                            continue 'lines;
                        }
                    }
                }
                ']' => {
                    if let Some(val) = stack.pop() {
                        if val != '[' {
                            continue 'lines;
                        }
                    }
                }
                '}' => {
                    if let Some(val) = stack.pop() {
                        if val != '{' {
                            continue 'lines;
                        }
                    }
                }
                '>' => {
                    if let Some(val) = stack.pop() {
                        if val != '<' {
                            continue 'lines;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        while let Some(val) = stack.pop() {
            score *= 5;
            match val {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => unreachable!(),
            }
        }
        scores.push(score);
    }
    let sorted_scores = scores.into_iter().sorted().collect::<Vec<u64>>();
    let middle_score = sorted_scores[sorted_scores.len() / 2];
    println!("part2: {}", middle_score)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
