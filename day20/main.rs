use std::collections::HashMap;
use std::io;
use std::io::Read;

fn find_end_parenthesis(input: &[char]) -> usize {
    let mut count = 1;
    for i in 0..input.len() {
        let c = input[i];
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
            if count == 0 {
                return i;
            }
        }
    }
    panic!()
}

fn recursive(board: &mut HashMap<(i32, i32), u64>,
             original_start_position: (i32, i32),
             original_progress: u64,
             input: &[char]) {
    let mut start_position = original_start_position;
    let mut progress = original_progress;
    let mut i = 0;
    while i < input.len() {
        let c = input[i];
        if c == 'N' {
            start_position = (start_position.0, start_position.1 - 1);
            progress += 1;
            if !board.contains_key(&start_position) {
                board.insert(start_position, progress);
            }
        } else if c == 'S' {
            start_position = (start_position.0, start_position.1 + 1);
            progress += 1;
            if !board.contains_key(&start_position) {
                board.insert(start_position, progress);
            }
        } else if c == 'W' {
            start_position = (start_position.0 - 1, start_position.1);
            progress += 1;
            if !board.contains_key(&start_position) {
                board.insert(start_position, progress);
            }
        } else if c == 'E' {
            start_position = (start_position.0 + 1, start_position.1);
            progress += 1;
            if !board.contains_key(&start_position) {
                board.insert(start_position, progress);
            }
        } else if c == '(' {
            let end = i + find_end_parenthesis(&input[i + 1..]) + 1;
            recursive(board, start_position, progress, &input[i + 1..end + 1]);
            i = end;
        } else if c == '|' {
            start_position = original_start_position;
            progress = original_progress;
        }

        i += 1;
    }
}

fn day20(input: &Vec<char>) -> (u64, usize) {
    let mut board: HashMap<(i32, i32), u64> = HashMap::new();

    recursive(&mut board, (0, 0), 0, &input[1..input.len() - 1]);

    let part1 = *board.values().max().unwrap_or(&0);
    let part2 = board.values().filter(|x| **x >= 1000).count();
    return (part1, part2);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let input_vec = input.chars().collect();
    let (part1, part2) = day20(&input_vec);
    println!("part1 {:?}", part1);
    println!("part2 {:?}", part2);
}
