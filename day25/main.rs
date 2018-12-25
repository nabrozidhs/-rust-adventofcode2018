use std::io;
use std::io::Read;

fn manhattan(a: &(i64, i64, i64, i64), b: &(i64, i64, i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn day25(input: &Vec<(i64, i64, i64, i64)>) -> usize {
    let mut constellations: Vec<Vec<(i64, i64, i64, i64)>> = vec![];

    for coord in input {
        let mut found: Vec<usize> = vec![];
        for i in 0..constellations.len() {
            for c in constellations[i].iter() {
                if manhattan(coord, &c) <= 3 {
                    found.push(i);
                    break;
                }
            }
        }

        match found.len() {
            0 => {
                constellations.push(vec![*coord]);
            }
            1 => {
                constellations[found[0]].push(*coord);
            }
            _ => {
                for i in (1..found.len()).rev() {
                    for c in constellations.remove(found[i]) {
                        constellations[found[0]].push(c);
                    }
                }
                constellations[found[0]].push(*coord);
            }
        }
    }

    constellations.len()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<(i64, i64, i64, i64)> = buffer.split("\n")
        .map(|x| {
            let a = x.split(",").map(|c| c.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
            (a[0], a[1], a[2], a[3])
        })
        .collect();

    println!("part1 {:?}", day25(&input));
}
