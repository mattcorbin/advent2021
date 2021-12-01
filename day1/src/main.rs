use std::fs;

fn part1(input: &str) {
    let mut count = 0;
    let mut last_measurement = 0;
    for (idx, measurement) in input
        .split("\n")
        .map(|x| x.parse::<isize>().expect("should parsable to an integer"))
        .enumerate()
    {
        if idx == 0 {
            last_measurement = measurement;
            continue;
        }
        if measurement > last_measurement {
            count += 1;
        }
        last_measurement = measurement;
    }
    println!("part1: {}", count);
}

fn part2(input: &str) {
    let mut count = 0;
    let mut windows: Vec<isize> = Vec::new();
    let measurements = input
        .split("\n")
        .map(|x| x.parse::<isize>().expect("should parsable to an integer"))
        .collect::<Vec<isize>>();
    for idx in 0..measurements.len() {
        if idx < 2 {
            continue;
        }
        windows.push(measurements[idx - 2] + measurements[idx - 1] + measurements[idx]);
    }
    for idx in 0..windows.len() {
        if idx == 0 {
            continue;
        }
        if windows[idx] > windows[idx - 1] {
            count += 1;
        }
    }
    println!("part2: {}", count);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
