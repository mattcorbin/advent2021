use std::cmp::max;

fn in_box(current_x: i64, current_y: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
    min_x <= current_x && current_x <= max_x &&  min_y <= current_y && current_y <= max_y
}

fn past_box(current_x: i64, current_y: i64, max_x: i64, min_y: i64) -> bool {
    current_x > max_x || current_y < min_y
}

fn run_simulation(initial_h: i64, initial_v: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> (bool, i64) {
    let mut current_x = 0;
    let mut current_y = 0;
    let mut current_h = initial_h;
    let mut current_v = initial_v;
    let mut max_height = i64::MIN;
    loop {
        if in_box(current_x, current_y, min_x, max_x, min_y, max_y) {
            return (true, max_height)
        } else if past_box(current_x, current_y, max_x, min_y) {
            return (false, 0)
        }
        current_x += current_h;
        current_y += current_v;
        max_height = max(max_height, current_y);
        current_h = max(0, current_h - 1);
        current_v -= 1;
    }
}


fn part1() {
    let mut max_height = i64::MIN;
    for x in 1..1000 {
        for y in -500..10000 {
            let (success, height) = run_simulation(x, y, 230, 283, -107, -57);
            if success {
                max_height = max(max_height, height);
            }
        }
    }
    println!("part1: {}", max_height)
}

fn part2() {
    let mut pairs_for_success = Vec::new();
    for x in 1..1000 {
        for y in -500..10000 {
            let (success, _) = run_simulation(x, y, 230, 283, -107, -57);
            if success {
                pairs_for_success.push((x,y));
            }
        }
    }
    println!("part2: {}", pairs_for_success.len())
}

fn main() {
    part1();
    part2();
}
