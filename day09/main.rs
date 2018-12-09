extern crate core;

use std::collections::HashMap;
use std::io;
use std::io::Read;

// I should be using a circular double linked iterator but meh
fn day09(num_players: u64, last_marble: u64) -> u64 {
    let mut board: Vec<u64> = Vec::with_capacity(last_marble as usize + 1);
    board.push(0);
    board.push(1);
    let mut scores: HashMap<u64, u64> = HashMap::new();

    let mut current_position = 1;
    for i in 2..last_marble + 1 {
        if i % 23 == 0 {
            current_position = (current_position + board.len() - 7) % board.len();
            let removed_marble = board.remove(current_position);
            let player = (i - 1) % num_players;
            let previous_score = scores.get(&player).unwrap_or(&0).clone();
            scores.insert(player, previous_score + removed_marble + i);
        } else {
            let insert_after = current_position + 1;
            if insert_after == board.len() - 1 {
                current_position = insert_after + 1;
            } else {
                current_position = (current_position + 2) % board.len();
            }
            board.insert(current_position, i);
        }
    }

    *scores.iter().max_by_key(|e| e.1).unwrap().1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parsed: Vec<&str> = input.split_whitespace().collect();

    let num_players: u64 = parsed.get(0).unwrap().parse().unwrap();
    let last_marble: u64 = parsed.get(6).unwrap().parse().unwrap();

    println!("part1 {}", day09(num_players, last_marble));
    // Yes it was running for 2 hours while I went for grocery shopping :D
    println!("part2 {}", day09(num_players, last_marble * 100));
}