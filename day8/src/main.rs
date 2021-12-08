use itertools::Itertools;
use std::fs;

fn part1(input: &str) {
    let mut total = 0;
    let known_values: [usize; 4] = [2, 3, 4, 7];
    for line in input.lines() {
        let output = line.split(" | ").skip(1).next().unwrap();
        let chunks = output.split_ascii_whitespace().collect::<Vec<&str>>();
        for chunk in chunks {
            if known_values.contains(&chunk.len()) {
                total += 1;
            }
        }
    }
    println!("part1: {}", total);
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let inputs;
    let outputs;
    let mut splits = line.split(" | ");
    let raw_inputs = splits.next().unwrap();
    inputs = raw_inputs.split(" ").map(|x| x.to_string()).collect();
    let raw_outputs = splits.next().unwrap();
    outputs = raw_outputs.split(" ").map(|x| x.to_string()).collect();
    (inputs, outputs)
}

fn get_text_diff(a: &str, b: &str) -> String {
    let mut retval = String::new();
    let a_chars = a.chars().sorted().collect::<Vec<char>>();
    let b_chars = b.chars().sorted().collect::<Vec<char>>();
    for c in a_chars {
        if !b.contains(c) && !retval.contains(c) {
            retval.push(c)
        }
    }
    for c in b_chars {
        if !a.contains(c) && !retval.contains(c) {
            retval.push(c)
        }
    }
    retval
}

fn get_single_char_diff(longer: &str, shorter: &str) -> char {
    for c in longer.chars() {
        if !shorter.contains(c) {
            return c;
        }
    }
    '0'
}

fn fully_contains(longer: &str, shorter: &str) -> bool {
    for c in shorter.chars() {
        if !longer.contains(c) {
            return false;
        }
    }
    true
}

fn part2(input: &str) {
    let mut total: u64 = 0;
    for line in input.lines() {
        let (inputs, outputs) = parse_line(line);
        let mut wires_used: [String; 10] = [
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];
        for input in &inputs {
            match input.len() {
                2 => wires_used[1] = input.chars().sorted().collect::<String>(),
                3 => wires_used[7] = input.chars().sorted().collect::<String>(),
                4 => wires_used[4] = input.chars().sorted().collect::<String>(),
                5 => {}
                6 => {}
                7 => wires_used[8] = input.chars().sorted().collect::<String>(),
                _ => unreachable!(),
            }
        }
        let top_row = get_single_char_diff(&wires_used[7], &wires_used[1]);
        let mut bottom_row = '0';
        let mut mostly_nine = String::new();
        mostly_nine.push(top_row);
        for c in wires_used[4].chars() {
            mostly_nine.push(c);
        }
        for input in &inputs {
            let i = input.chars().sorted().collect::<String>();
            if i.len() == 6 {
                if get_text_diff(&i, &mostly_nine).len() == 1 {
                    bottom_row = get_single_char_diff(&i, &mostly_nine);
                    wires_used[9] = i;
                }
            }
        }
        let mut mostly_three = String::new();
        mostly_three.push(bottom_row);
        for c in wires_used[7].chars() {
            mostly_three.push(c);
        }
        for input in &inputs {
            let i = input.chars().sorted().collect::<String>();
            if i.len() == 5 {
                if get_text_diff(&i, &mostly_three).len() == 1 {
                    wires_used[3] = i;
                }
            }
        }
        for input in &inputs {
            let i = input.chars().sorted().collect::<String>();
            if i.len() == 6 {
                if fully_contains(&i, &wires_used[1]) && i != wires_used[9] {
                    wires_used[0] = i;
                }
            }
        }
        for input in &inputs {
            let i = input.chars().sorted().collect::<String>();
            if i.len() == 6 {
                if i != wires_used[0] && i != wires_used[9] {
                    wires_used[6] = i.clone();
                }
            }
        }
        for input in &inputs {
            let i = input.chars().sorted().collect::<String>();
            if i.len() == 5 {
                if get_text_diff(&i, &wires_used[6]).len() == 1 {
                    wires_used[5] = i;
                }
            }
        }
        for input in &inputs {
            let i = input.chars().sorted().collect::<String>();
            if !wires_used.contains(&i) {
                wires_used[2] = i;
            }
        }
        let mut digits = Vec::new();
        for output in outputs {
            let digit = wires_used
                .iter()
                .position(|x| *x == output.chars().sorted().collect::<String>())
                .unwrap();
            digits.push(digit);
        }
        let number = format!("{}{}{}{}", digits[0], digits[1], digits[2], digits[3]);
        total += number.parse::<u64>().unwrap();
    }
    println!("part2: {}", total);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
