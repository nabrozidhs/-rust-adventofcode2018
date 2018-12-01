use std::collections::HashSet;
use std::io;
use std::io::Read;

fn day1_part1(digits: &Vec<i64>) -> i64 {
    digits.iter().sum()
}

fn day1_part2(digits: &Vec<i64>) -> i64 {
    let mut seen = HashSet::new();
    seen.insert(0);

    let mut frequency: i64 = 0;
    loop {
        for modifier in digits {
            frequency += modifier;
            if seen.contains(&frequency) {
                return frequency;
            }
            seen.insert(frequency);
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let digits: Vec<i64> = buffer.split("\n")
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    // part1
    println!("part1 {}", day1_part1(&digits));
    // part2
    println!("part2 {}", day1_part2(&digits));
}