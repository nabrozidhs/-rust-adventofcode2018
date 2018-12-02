use std::collections::HashMap;
use std::io;
use std::io::Read;

fn day2_part1(lines: &Vec<&str>) -> i64 {
    let mut checksum = (0, 0);
    for line in lines {
        let mut map = HashMap::new();
        for c in line.chars() {
            match map.get(&c).cloned() {
                None => { map.insert(c, 1); }
                Some(v) => { map.insert(c, v + 1); }
            }
        }
        let result = (
            map.values().filter(|x| **x == 2).count() as i64,
            map.values().filter(|x| **x == 3).count() as i64,
        );

        checksum.0 += result.0.min(1);
        checksum.1 += result.1.min(1);
    }
    checksum.0 * checksum.1
}

fn day2_part2(lines: &Vec<&str>) -> String {
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            let mut base = lines[i].chars();
            let mut other = lines[j].chars();
            let mut diff = 0;
            let mut position = 0;
            for x in 0..lines[i].len() {
                if base.next() != other.next() {
                    position = x;
                    diff += 1;
                }

                if diff > 1 {
                    break;
                }
            }
            if diff == 1 {
                let mut string = lines[i].to_string();
                string.remove(position);
                return string;
            }
        }
    }
    panic!("Could not find match");
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let digits: Vec<&str> = buffer.split("\n")
        .collect();

    println!("part1 {}", day2_part1(&digits));
    println!("part2 {}", day2_part2(&digits));
}
