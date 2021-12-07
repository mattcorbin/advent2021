use std::cmp::min;
use std::fs;

fn part1(input: &str) {
    let positions: Vec<u64> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let highest_pos = *positions.iter().max().unwrap();
    let lowest_post = *positions.iter().min().unwrap();
    let mut min_fuel = u64::MAX;
    for i in lowest_post..highest_pos {
        let mut fuel_required = 0;
        for pos in &positions {
            if *pos > i {
                fuel_required += *pos-i;
            } else {
                fuel_required += i-*pos;
            }
        }
        min_fuel = min(min_fuel, fuel_required)
    }
    println!("part1: {}", min_fuel);
}

fn part2(input: &str) {
    let positions: Vec<u64> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let highest_pos = *positions.iter().max().unwrap();
    let lowest_post = *positions.iter().min().unwrap();
    let mut min_fuel = u64::MAX;
    for i in lowest_post..highest_pos {
        let mut fuel_required = 0;
        for pos in &positions {
            if *pos > i {
                fuel_required += (0..=*pos-i).sum::<u64>();
            } else {
                fuel_required += (0..=i-*pos).sum::<u64>();
            }
        }
        min_fuel = min(min_fuel, fuel_required)
    }
    println!("part2: {}", min_fuel);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
