extern crate regex;

use std::io;
use std::io::Read;

use regex::Regex;

type Position = (i64, i64, i64);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Bot {
    pos: Position,
    r: i64,
}

fn parse(input: &Vec<&str>) -> Vec<Bot> {
    let mut bots: Vec<Bot> = vec![];

    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
    for line in input {
        let cap = re.captures(line).unwrap();
        bots.push(
            Bot {
                pos: (cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()),
                r: cap[4].parse().unwrap(),
            }
        );
    }

    bots
}

fn manhattan_distance(left: &Position, right: &Position) -> i64 {
    (left.0 - right.0).abs() + (left.1 - right.1).abs() + (left.2 - right.2).abs()
}

fn day23_part1(bots: &Vec<Bot>) -> usize {
    let longest_range_bot = bots.iter().max_by_key(|x| x.r).unwrap();

    bots.iter()
        .filter(|b| manhattan_distance(&b.pos, &longest_range_bot.pos) <= longest_range_bot.r)
        .count()
}

fn day23_part2(bots: &Vec<Bot>) -> u64 {
    let mut biggest_group: Vec<Bot> = vec![];
    for start_bot in bots {
        let mut group: Vec<Bot> = vec![];
        group.push(*start_bot);
        for b in bots {
            if *b != *start_bot && group.iter()
                .all(|g| manhattan_distance(&g.pos, &b.pos) <= g.r + b.r) {
                group.push(*b);
            }
        }

        if group.len() > biggest_group.len() {
            biggest_group = group;
        }
    }

    let mut min = (i64::min_value(), i64::min_value(), i64::min_value());
    let mut max = (i64::max_value(), i64::max_value(), i64::max_value());
    for a in biggest_group.iter() {
        min.0 = min.0.max(a.pos.0 - a.r);
        max.0 = max.0.min(a.pos.0 + a.r);
    }

    let mut pos: Vec<Position> = vec![];
    let mut x = min.0;
    while x <= max.0 {
        min.1 = i64::min_value();
        max.1 = i64::max_value();
        for a in biggest_group.iter() {
            let distance_1 = a.r - (x - a.pos.0).abs();
            min.1 = min.1.max(a.pos.1 - distance_1);
            max.1 = max.1.min(a.pos.1 + distance_1);
        }
        if min.1 > max.1 {
            x += 1.max((min.1 - max.1) / 2);
            continue;
        }

        let mut y = min.1;
        while y <= max.1 {
            min.2 = i64::min_value();
            max.2 = i64::max_value();
            for a in biggest_group.iter() {
                let distance_2 = a.r - (x - a.pos.0).abs() - (y - a.pos.1).abs();
                min.2 = min.2.max(a.pos.2 - distance_2);
                max.2 = max.2.min(a.pos.2 + distance_2);
            }

            if min.2 > max.2 {
                y += 1.max((min.2 - max.2) / 2);
                continue;
            }

            for z in min.2..=max.2 {
                pos.push((x, y, z));
            }
            y += 1;
        }
        x += 1;
    }

    pos.iter()
        .map(|p| manhattan_distance(&(0, 0, 0), p) as u64)
        .min()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let bots = parse(&input.trim().split("\n").collect());

    println!("part1 {:?}", day23_part1(&bots));
    println!("part2 {:?}", day23_part2(&bots));
}
