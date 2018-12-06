use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn parse(input: &str) -> (u64, u64) {
    let result: Vec<&str> = input.split(", ").collect();

    (result.get(0).unwrap().parse().unwrap(), result.get(1).unwrap().parse().unwrap())
}

fn distance_to(left: &(u64, u64), right: &(u64, u64)) -> u64 {
    ((left.0 as i64 - right.0 as i64).abs() + (left.1 as i64 - right.1 as i64).abs()) as u64
}

fn day6(input: &Vec<(u64, u64)>) -> (u64, u64) {
    let min_x = input.iter().min_by_key(|x| x.0).unwrap().0;
    let max_x = input.iter().max_by_key(|x| x.0).unwrap().0;
    let min_y = input.iter().min_by_key(|x| x.1).unwrap().1;
    let max_y = input.iter().max_by_key(|x| x.1).unwrap().1;
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut board: Vec<Vec<i64>> = Vec::new();
    let mut infinite_coordinates: HashSet<i64> = HashSet::new();
    infinite_coordinates.insert(-1);
    let mut areas: HashMap<i64, u64> = HashMap::new();
    let mut part2 = 0;
    for x in 0..width {
        let mut row: Vec<i64> = Vec::new();
        let real_x = x + min_x;
        for y in 0..height {
            let real_y = y + min_y;
            let current_pos = (real_x, real_y);

            let mut found_index: i64 = -1;
            let mut min_distance = u64::max_value();
            let mut more_than_one = false;
            for i in 0..input.len() {
                let c = input.get(i).unwrap();
                let d = distance_to(c, &current_pos);
                if d < min_distance {
                    found_index = i as i64;
                    min_distance = d;
                    more_than_one = false;
                } else if d == min_distance {
                    more_than_one = true;
                }
            }

            if more_than_one {
                row.push(-1);
            } else {
                row.push(found_index);
            }

            let just_inserted = *row.last().unwrap();
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                infinite_coordinates.insert(just_inserted);
            }
            let s: u64 = input.iter().map(|e| distance_to(&current_pos, e))
                .sum();
            if s < 10000 {
                part2 += 1;
            }
            match areas.get(&just_inserted).cloned() {
                Some(v) => { areas.insert(just_inserted, v + 1); }
                None => { areas.insert(just_inserted, 1); }
            }
        }
        board.push(row);
    }

    (
        *areas.iter().filter(|e| !infinite_coordinates.contains(e.0))
            .max_by_key(|e| e.1)
            .unwrap().1,
        part2
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parsed: Vec<(u64, u64)> = input.split("\n").map(|x| parse(x)).collect();

    let (part1, part2) = day6(&parsed);
    println!("part1 {}", part1);
    println!("part2 {}", part2);
}
