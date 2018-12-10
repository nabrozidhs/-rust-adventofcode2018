extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

fn parse(input: &str) -> ((i64, i64), (i64, i64)) {
    let re = Regex::new(r"position=< ?([\-0-9]+),  ?([\-0-9]+)> velocity=< ?([\-0-9]+),  ?([\-0-9]+)>").unwrap();
    let cap = re.captures(input).unwrap();
    (
        (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap()
        ),
        (
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap()
        )
    )
}

fn day10(input: &Vec<((i64, i64), (i64, i64))>) -> u64 {
    let mut map: HashMap<usize, ((i64, i64), (i64, i64))> = HashMap::new();
    for i in 0..input.len() {
        map.insert(i, *input.get(i).unwrap());
    }

    let mut seconds = 0;
    loop {
        let min_x = (map.values().min_by_key(|x| (x.0).0).unwrap().0).0;
        let max_x = (map.values().max_by_key(|x| (x.0).0).unwrap().0).0;
        let min_y = (map.values().min_by_key(|x| (x.0).1).unwrap().0).1;
        let max_y = (map.values().max_by_key(|x| (x.0).1).unwrap().0).1;

        if max_y - min_y <= 10 {
            for y in min_y..max_y + 1 {
                for x in min_x..max_x + 1 {
                    let pos = (x, y);
                    if map.values().any(|x| x.0 == pos) {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
                println!()
            }
            return seconds;
        }

        for i in 0..input.len() {
            let v = map.get(&i).unwrap().clone();
            let new_v = (((v.0).0 + (v.1).0, (v.0).1 + (v.1).1), v.1);
            map.insert(i, new_v);
        }
        seconds += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parsed: Vec<((i64, i64), (i64, i64))> = input.split("\n")
        .map(|x| parse(x))
        .collect();

    println!("part2 {}", day10(&parsed));
}
