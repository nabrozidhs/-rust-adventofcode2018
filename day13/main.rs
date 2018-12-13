use std::collections::HashSet;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Cart {
    position: (i64, i64),
    velocity: (i64, i64),
    intersections_taken: u64,
}

fn parse(input: &Vec<&str>) -> (Vec<Vec<char>>, Vec<Cart>) {
    let mut map: Vec<Vec<char>> = vec![];
    let mut carts: Vec<Cart> = vec![];

    let mut y = 0;
    for line in input {
        let mut x = 0;
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(
                match c {
                    '<' | '>' => '-',
                    '^' | 'v' => '|',
                    _ => c,
                }
            );

            if c == '>' {
                carts.push(Cart {
                    position: (x, y),
                    velocity: (1, 0),
                    intersections_taken: 0,
                })
            } else if c == '<' {
                carts.push(Cart {
                    position: (x, y),
                    velocity: (-1, 0),
                    intersections_taken: 0,
                })
            } else if c == '^' {
                carts.push(Cart {
                    position: (x, y),
                    velocity: (0, -1),
                    intersections_taken: 0,
                })
            } else if c == 'v' {
                carts.push(Cart {
                    position: (x, y),
                    velocity: (0, 1),
                    intersections_taken: 0,
                })
            }
            x += 1;
        }
        y += 1;
        map.push(row);
    }

    (map, carts)
}

fn day13(map: &Vec<Vec<char>>, initial_carts: Vec<Cart>) -> ((i64, i64), (i64, i64)) {
    let mut carts: Vec<Cart> = initial_carts;
    let mut first_crash: (i64, i64) = (-1, -1);
    loop {
        carts.sort_by_key(|x| x.position.0);
        carts.sort_by_key(|x| x.position.1);
        let mut positions_to_check: HashSet<(i64, i64)> = HashSet::new();
        let mut new_carts: Vec<Cart> = vec![];
        for c in carts {
            if positions_to_check.contains(&c.position) {
                if first_crash.0 == -1 {
                    first_crash = c.position;
                }

                new_carts = new_carts.into_iter().filter(|x| x.position != c.position)
                    .collect();
                continue;
            }
            let new_position = (
                c.position.0 + c.velocity.0,
                c.position.1 + c.velocity.1,
            );

            let v = map.get(new_position.1 as usize).unwrap()
                .get(new_position.0 as usize).unwrap();
            let new_intersections_taken = if *v == '+' {
                c.intersections_taken + 1
            } else {
                c.intersections_taken
            };

            let new_velocity = match v {
                '\\' => (c.velocity.1, c.velocity.0),
                '/' => (-c.velocity.1, -c.velocity.0),
                '+' => {
                    match new_intersections_taken % 3 {
                        1 => (c.velocity.1, -c.velocity.0),
                        2 => c.velocity,
                        0 => (-c.velocity.1, c.velocity.0),
                        _ => panic!(),
                    }
                }
                _ => c.velocity,
            };

            if !positions_to_check.insert(new_position) {
                if first_crash.0 == -1 {
                    first_crash = new_position;
                }

                new_carts = new_carts.into_iter().filter(|x| x.position != new_position)
                    .collect();
            } else {
                new_carts.push(Cart {
                    position: new_position,
                    velocity: new_velocity,
                    intersections_taken: new_intersections_taken,
                });
            }
        }

        if new_carts.len() <= 1 {
            return match new_carts.first() {
                Some(v) => (first_crash, v.position),
                None => (first_crash, (-1, -1)),
            };
        }
        carts = new_carts;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (map, carts) = parse(&input.split("\n").collect());

    let (part1, part2) = day13(&map, carts);
    println!("part1 {:?}", part1);
    println!("part2 {:?}", part2);
}
