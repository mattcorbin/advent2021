use std::collections::{HashMap, HashSet};
use std::fs;

fn is_upper(test: &str) -> bool {
    test.chars().all(|x| x.is_ascii_uppercase())
}

fn all_paths(
    start: &str,
    visited: &HashMap<String, bool>,
    connections: &HashMap<String, Vec<String>>,
) -> usize {
    if start == "end" {
        return 1;
    }
    if *visited.get(start).expect(&format!("{} exists", start)) {
        if !is_upper(start) {
            return 0;
        }
    }
    let mut local_visited = visited.clone();
    local_visited.insert(start.to_string(), true);
    let mut visits = 0;
    if let Some(paths) = connections.get(start) {
        for path in paths {
            visits += all_paths(path, &local_visited, connections);
        }
    }
    return visits;
}

fn all_paths_with_revisit(
    start: &str,
    visited: &HashMap<String, bool>,
    connections: &HashMap<String, Vec<String>>,
    revisited: bool,
) -> usize {
    if start == "end" {
        return 1;
    }
    let mut new_revisited = revisited;
    if *visited.get(start).expect(&format!("{} exists", start)) {
        if !is_upper(start) {
            if !revisited && start != "start" {
                new_revisited = true;
            } else {
                return 0;
            }
        }
    }
    let mut new_visited = visited.clone();
    new_visited.insert(start.to_string(), true);
    let mut visits = 0;
    if let Some(paths) = connections.get(start) {
        for path in paths {
            visits += all_paths_with_revisit(path, &new_visited, connections, new_revisited);
        }
    }
    return visits;
}

fn part1(input: &str) {
    let mut locations = HashSet::new();
    let mut connections = HashMap::new();
    for line in input.lines() {
        let splits = line.split("-").collect::<Vec<&str>>();
        if splits[0] == "start" || splits[1] == "end" {
            match connections.get(splits[0]) {
                None => {
                    connections.insert(splits[0].to_string(), vec![splits[1].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[1].to_string());
                    connections.insert(splits[0].to_string(), new_vec);
                }
            }
        } else if splits[1] == "start" || splits[0] == "end" {
            match connections.get(splits[1]) {
                None => {
                    connections.insert(splits[1].to_string(), vec![splits[0].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[0].to_string());
                    connections.insert(splits[1].to_string(), new_vec);
                }
            }
        } else {
            match connections.get(splits[0]) {
                None => {
                    connections.insert(splits[0].to_string(), vec![splits[1].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[1].to_string());
                    connections.insert(splits[0].to_string(), new_vec);
                }
            }
            match connections.get(splits[1]) {
                None => {
                    connections.insert(splits[1].to_string(), vec![splits[0].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[0].to_string());
                    connections.insert(splits[1].to_string(), new_vec);
                }
            }
        }
        for split in splits {
            locations.insert(split.to_string());
        }
    }
    let mut visited = HashMap::new();
    for location in locations {
        visited.insert(location, false);
    }
    let all_paths = all_paths("start", &visited, &connections);
    println!("part1: {}", all_paths)
}

fn part2(input: &str) {
    let mut locations = HashSet::new();
    let mut connections = HashMap::new();
    for line in input.lines() {
        let splits = line.split("-").collect::<Vec<&str>>();
        if splits[0] == "start" || splits[1] == "end" {
            match connections.get(splits[0]) {
                None => {
                    connections.insert(splits[0].to_string(), vec![splits[1].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[1].to_string());
                    connections.insert(splits[0].to_string(), new_vec);
                }
            }
        } else if splits[1] == "start" || splits[0] == "end" {
            match connections.get(splits[1]) {
                None => {
                    connections.insert(splits[1].to_string(), vec![splits[0].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[0].to_string());
                    connections.insert(splits[1].to_string(), new_vec);
                }
            }
        } else {
            match connections.get(splits[0]) {
                None => {
                    connections.insert(splits[0].to_string(), vec![splits[1].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[1].to_string());
                    connections.insert(splits[0].to_string(), new_vec);
                }
            }
            match connections.get(splits[1]) {
                None => {
                    connections.insert(splits[1].to_string(), vec![splits[0].to_string()]);
                }
                Some(v) => {
                    let mut new_vec = v.clone();
                    new_vec.push(splits[0].to_string());
                    connections.insert(splits[1].to_string(), new_vec);
                }
            }
        }
        for split in splits {
            locations.insert(split.to_string());
        }
    }
    let mut visited = HashMap::new();
    for location in locations {
        visited.insert(location, false);
    }
    let all_paths = all_paths_with_revisit("start", &visited, &connections, false);
    println!("part2: {}", all_paths)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
