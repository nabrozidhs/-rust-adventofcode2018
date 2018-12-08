use std::io;
use std::io::Read;

fn day08_part1(input: &mut Vec<u64>) -> u64 {
    let nodes = input.remove(0);
    let metadata_count = input.remove(0);

    let mut metadata = 0;
    for _ in 0..nodes {
        metadata += day08_part1(input);
    }

    for _ in 0..metadata_count {
        metadata += input.remove(0);
    }

    metadata
}

fn day08_part2(input: &mut Vec<u64>) -> u64 {
    let nodes = input.remove(0);
    let metadata_count = input.remove(0);

    let mut values: Vec<u64> = vec![];
    for _ in 0..nodes {
        values.push(day08_part2(input));
    }

    let mut current_value: u64 = 0;
    for _ in 0..metadata_count {
        let metadata = input.remove(0);
        if nodes != 0 {
            current_value += values.get((metadata - 1) as usize).unwrap_or(&0);
        } else {
            current_value += metadata;
        }
    }

    current_value
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<u64> = buffer.split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    println!("part1 {}", day08_part1(&mut input.clone()));
    println!("part2 {}", day08_part2(&mut input.clone()));
}
