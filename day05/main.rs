use std::io;
use std::io::Read;

fn day5_part1(input: &mut String) -> u64 {
    let mut index: usize = 1;

    while index < input.len() {
        let a = input.as_bytes()[index];
        let b = input.as_bytes()[index - 1];
        if a == b + 32 || a + 32 == b {
            input.remove(index);
            input.remove(index - 1);
            index = (index as i64 - 2).max(1) as usize;
        } else {
            index += 1;
        }
    }

    input.len() as u64
}

fn clean_input(input: &mut String, to_remove: u8) {
    for i in (0..input.len() - 1).rev() {
        let c = input.as_bytes()[i];
        if c == to_remove || c == to_remove + 32 {
            input.remove(i);
        }
    }
}

fn day5_part2(original: &String) -> u64 {
    let mut min = u64::max_value();
    for c in 65..90 as u8 {
        let input = &mut original.clone();
        clean_input(input, c);
        if input.len() == original.len() {
            continue;
        }
        min = day5_part1(input).min(min);
    }
    min
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("part1 {}", day5_part1(&mut input.clone()));
    println!("part2 {}", day5_part2(&input));
}
