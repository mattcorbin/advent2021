use std::fs;

fn part1(input: &str) {
    let mut all_fish: Vec<u64> = Vec::new();
    for item in input.split(",") {
        all_fish.push(item.parse().unwrap());
    }
    for _ in 0..80 {
        let mut new_fish: Vec<u64> = Vec::new();
        for fish in all_fish.iter_mut() {
            if *fish == 0 {
                new_fish.push(8);
                *fish = 6
            } else {
                *fish -= 1
            }
        }
        all_fish.append(&mut new_fish);
    }
    println!("part1: {}", all_fish.len())
}

fn part2(input: &str) {
    let mut all_fish: [u64; 9] = [0; 9];
    for item in input.split(",") {
        all_fish[item.parse::<usize>().unwrap()] += 1
    }
    for _ in 0..256 {
        let mut new_fish: [u64; 9] = [0; 9];
        for (idx, count) in all_fish.iter().enumerate() {
            if idx == 0 {
                new_fish[6] += *count;
                new_fish[8] += *count;
            } else {
                new_fish[idx-1] += *count;
            }
        }
        all_fish = new_fish;
    }
    println!("part2: {}", all_fish.iter().sum::<u64>())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
