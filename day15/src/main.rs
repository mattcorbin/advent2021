use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;
use std::fs;

fn get_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let min_x;
    let max_x;
    let min_y;
    let max_y;
    match x {
        0 => {
            min_x = 0;
            max_x = x + 1;
        }
        99 => {
            min_x = x - 1;
            max_x = 99;
        }
        _ => {
            min_x = x - 1;
            max_x = x + 1;
        }
    }
    match y {
        0 => {
            min_y = 0;
            max_y = y + 1;
        }
        99 => {
            min_y = y - 1;
            max_y = 99;
        }
        _ => {
            min_y = y - 1;
            max_y = y + 1;
        }
    }
    for i in min_x..=max_x {
        if i == x {
            continue;
        }
        neighbours.push((i, y));
    }
    for j in min_y..=max_y {
        if j == y {
            continue;
        }
        neighbours.push((x, j));
    }
    neighbours
}

fn part1(input: &str) {
    let mut node_enter_cost = HashMap::new();
    let mut graph: GraphMap<(usize, usize), usize, Directed> = DiGraphMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, val) in line.chars().enumerate() {
            node_enter_cost.insert((x, y), val.to_string().parse::<usize>().unwrap());
        }
    }
    for (node, cost) in node_enter_cost.iter() {
        for neighbour in get_neighbours(node.0, node.1) {
            graph.add_edge(neighbour, *node, *cost);
        }
    }
    let lowest_risk_level = dijkstra(&graph, (0, 0), Some((99,99)), |e| *e.2);
    println!("part1: {:?}", lowest_risk_level.get(&(99, 99)).unwrap())
}


fn get_neighbours_5x(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let min_x;
    let max_x;
    let min_y;
    let max_y;
    match x {
        0 => {
            min_x = 0;
            max_x = x + 1;
        }
        499 => {
            min_x = x - 1;
            max_x = 499;
        }
        _ => {
            min_x = x - 1;
            max_x = x + 1;
        }
    }
    match y {
        0 => {
            min_y = 0;
            max_y = y + 1;
        }
        499 => {
            min_y = y - 1;
            max_y = 499;
        }
        _ => {
            min_y = y - 1;
            max_y = y + 1;
        }
    }
    for i in min_x..=max_x {
        if i == x {
            continue;
        }
        neighbours.push((i, y));
    }
    for j in min_y..=max_y {
        if j == y {
            continue;
        }
        neighbours.push((x, j));
    }
    neighbours
}

fn looping_add(a: usize, b: usize) -> usize {
    let mut retval = a + b;
    if retval > 9 {
        retval -= 9;
    }
    retval
}

fn part2(input: &str) {
    let mut node_enter_cost = HashMap::new();
    let mut graph: GraphMap<(usize, usize), usize, Directed> = DiGraphMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, val) in line.chars().enumerate() {
            let cost = val.to_string().parse::<usize>().unwrap();
            node_enter_cost.insert((x, y), cost);
        }
    }
    for (node, cost) in node_enter_cost.clone() {
        for i in 1..5 {
            let x_mod = i*100;
            node_enter_cost.insert((node.0 + x_mod, node.1), looping_add(cost, i));
        }
    }
    for (node, cost) in node_enter_cost.clone() {
        for i in 1..5 {
            let y_mod = i*100;
            node_enter_cost.insert((node.0, node.1 + y_mod), looping_add(cost, i));
        }
    }
    for (node, cost) in node_enter_cost.iter() {
        for neighbour in get_neighbours_5x(node.0, node.1) {
            graph.add_edge(neighbour, *node, *cost);
        }
    }
    let lowest_risk_level = dijkstra(&graph, (0, 0), Some((499,499)), |e| *e.2);
    println!("part2: {:?}", lowest_risk_level.get(&(499, 499)).unwrap())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
