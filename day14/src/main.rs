use std::collections::HashMap;
use std::fs;

fn part1(input: &str) {
    let mut polymer = "".to_string();
    let mut insertions = HashMap::new();
    for (idx, line) in input.lines().enumerate() {
        if idx == 0 {
            polymer = line.to_string();
        } else if line.contains(" -> ") {
            let mut splits = line.split(" -> ");
            let pair = splits.next().unwrap().to_string();
            let new = splits.next().unwrap().chars().next().unwrap();
            insertions.insert(pair, new);
        }
    }
    for _ in 0..10 {
        let chars = polymer.chars().collect::<Vec<char>>();
        let mut new_string = String::new();
        for i in 0..polymer.len() {
            if i == 0 {
                continue;
            }
            new_string.push(chars[i - 1]);
            let test = format! {"{}{}", chars[i-1], chars[i]};
            if let Some(insertion) = insertions.get(&test) {
                new_string.push(*insertion);
            }
        }
        new_string.push(chars[chars.len() - 1]);
        polymer = new_string;
    }
    let mut char_map = HashMap::new();
    for c in polymer.chars() {
        if let Some(count) = char_map.get_mut(&c) {
            *count += 1;
        } else {
            char_map.insert(c, 1);
        }
    }
    println!(
        "part1: {}",
        char_map.iter().map(|x| *x.1).max().unwrap() - char_map.iter().map(|x| *x.1).min().unwrap()
    )
}

fn part2(input: &str) {
    let mut polymer = HashMap::new();
    let mut insertions = HashMap::new();
    for (idx, line) in input.lines().enumerate() {
        if idx == 0 {
            let chars = line.chars().collect::<Vec<char>>();
            for i in 0..chars.len() {
                if i == 0 {
                    continue;
                }
                let pair = format!("{}{}", chars[i - 1], chars[i]);
                if let Some(val) = polymer.get_mut(&pair) {
                    *val += 1;
                } else {
                    polymer.insert(pair, 1);
                }
            }
        } else if line.contains(" -> ") {
            let mut splits = line.split(" -> ");
            let pair = splits.next().unwrap().to_string();
            let new = splits.next().unwrap().chars().next().unwrap();
            insertions.insert(pair, new);
        }
    }

    for _ in 0..40 {
        let mut new_polymer = polymer.clone();
        for (pair, current_count) in polymer.iter() {
            if let Some(insertion) = insertions.get(pair) {
                let mut chars = pair.chars();
                let pair1 = format!("{}{}", chars.next().unwrap(), *insertion);
                let pair2 = format!("{}{}", *insertion, chars.next().unwrap());
                if let Some(count) = new_polymer.get_mut(pair) {
                    *count -= *current_count;
                }
                if let Some(count) = new_polymer.get_mut(&pair1) {
                    *count += *current_count;
                } else {
                    new_polymer.insert(pair1, *current_count);
                }
                if let Some(count) = new_polymer.get_mut(&pair2) {
                    *count += *current_count;
                } else {
                    new_polymer.insert(pair2, *current_count);
                }
            }
        }
        polymer = new_polymer;
    }

    let mut char_map: HashMap<char, u64> = HashMap::new();
    for (pair, occurrences) in polymer.iter() {
        for c in pair.chars() {
            if let Some(count) = char_map.get_mut(&c) {
                *count += *occurrences;
            } else {
                char_map.insert(c, *occurrences);
            }
        }
    }
    println!("{:?}", char_map);
    println!(
        "part2: {}",
        char_map.iter().map(|x| (*x.1 + 1)/2).max().unwrap() - char_map.iter().map(|x| (*x.1 + 1)/2).min().unwrap()
    )
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
