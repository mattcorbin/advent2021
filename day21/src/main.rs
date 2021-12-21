use cached::proc_macro::cached;
use std::cmp::{max, min};

fn wrapping_move(start: u64, distance: u64) -> u64 {
    let mut total = start + distance;
    while total > 10 {
        total -= 10
    }
    total
}

fn next_roll(current: u64) -> u64 {
    if current + 1 > 100 {
        return 1;
    }
    current + 1
}

fn part1() {
    let mut p1_pos = 3;
    let mut p1_score = 0;
    let mut p2_pos = 5;
    let mut p2_score = 0;
    let mut die = 1;
    let mut turn = true;
    let mut rolls = 0;
    loop {
        let mut moves = die;
        die = next_roll(die);
        moves += die;
        die = next_roll(die);
        moves += die;
        die = next_roll(die);
        rolls += 3;
        if turn {
            p1_pos = wrapping_move(p1_pos, moves);
            p1_score += p1_pos;
        } else {
            p2_pos = wrapping_move(p2_pos, moves);
            p2_score += p2_pos;
        }
        if p1_score >= 1000 || p2_score >= 1000 {
            break;
        }
        turn = !turn;
    }
    println!("part1: {}", rolls * min(p1_score, p2_score));
}

#[cached]
fn dirac(p1_pos: u64, p1_score: u64, p2_pos: u64, p2_score: u64, turn: bool) -> (u64, u64) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for roll1 in 1..=3 {
        for roll2 in 1..=3 {
            for roll3 in 1..=3 {
                if turn {
                    let new_pos = wrapping_move(p1_pos, roll1 + roll2 + roll3);
                    let new_score = p1_score + new_pos;
                    if new_score >= 21 {
                        p1_wins += 1;
                    } else {
                        let (p1, p2) = execute(new_pos, new_score, p2_pos, p2_score, false);
                        p1_wins += p1;
                        p2_wins += p2;
                    }
                } else {
                    let new_pos = wrapping_move(p2_pos, roll1 + roll2 + roll3);
                    let new_score = p2_score + new_pos;
                    if new_score >= 21 {
                        p2_wins += 1;
                    } else {
                        let (p1, p2) = execute(p1_pos, p1_score, new_pos, new_score, true);
                        p1_wins += p1;
                        p2_wins += p2;
                    }
                }
            }
        }
    }
    (p1_wins, p2_wins)
}

fn part2() {
    let (p1_wins, p2_wins) = dirac(3, 0, 5, 0, true);
    println!("part2: {}", max(p1_wins, p2_wins))
}

fn main() {
    part1();
    part2();
}
