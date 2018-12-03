extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: u64,
    start_x: u64,
    start_y: u64,
    width: u64,
    height: u64,
}

fn parse(input: &str) -> Claim {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    Claim {
        id: cap[1].parse().unwrap(),
        start_x: cap[2].parse().unwrap(),
        start_y: cap[3].parse().unwrap(),
        width: cap[4].parse().unwrap(),
        height: cap[5].parse().unwrap(),
    }
}

fn day3_part1(lines: &Vec<Claim>) -> u64 {
    let mut map = HashMap::new();

    for claim in lines {
        for x in claim.start_x..claim.start_x + claim.width {
            for y in claim.start_y..claim.start_y + claim.height {
                let pos = (x, y);
                match map.get(&pos).cloned() {
                    Some(v) => { map.insert(pos, v + 1); }
                    None => { map.insert(pos, 1); }
                }
            }
        }
    }

    map.values().filter(|x| x > &&1).count() as u64
}

fn collide(left: &Claim, right: &Claim) -> bool {
    left.start_x < right.start_x + right.width &&
        right.start_x < left.start_x + left.width &&
        left.start_y < right.start_y + right.height &&
        right.start_y < left.start_y + left.height
}

fn day3_part2(claims: &Vec<Claim>) -> &Claim {
    for i in 0..claims.len() {
        let mut found = false;
        for j in 0..claims.len() {
            if i == j {
                continue;
            }
            if collide(&claims[i], &claims[j]) {
                found = true;
                break;
            }
        }

        if !found {
            return &claims[i];
        }
    }
    panic!("Could not found claim")
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let digits: Vec<Claim> = buffer.lines()
        .map(|x| parse(x))
        .collect();

    println!("part1 {}", day3_part1(&digits));
    println!("part2 id #{}", day3_part2(&digits).id);
}
