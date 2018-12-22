extern crate regex;

use std::io;
use std::io::Read;

use regex::Regex;

fn parse(input: &Vec<&str>) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = vec![];

    let re_x = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
    let re_y = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
    let mut xs: Vec<(usize, (usize, usize))> = vec![];
    let mut ys: Vec<(usize, (usize, usize))> = vec![];
    for line in input {
        let opt_cap_x = re_x.captures(line);
        if opt_cap_x.is_some() {
            let cap_x = opt_cap_x.unwrap();
            xs.push(
                (
                    cap_x[1].parse::<usize>().unwrap(),
                    (cap_x[2].parse::<usize>().unwrap(), cap_x[3].parse::<usize>().unwrap())
                )
            );
            continue;
        }

        let cap_y = re_y.captures(line).unwrap();
        ys.push(
            (
                cap_y[1].parse::<usize>().unwrap(),
                (cap_y[2].parse::<usize>().unwrap(), cap_y[3].parse::<usize>().unwrap())
            )
        );
    }

    let mut min_x = xs.iter().map(|x| x.0).min().unwrap();
    min_x = ys.iter().map(|x| (x.1).0).min().unwrap().min(min_x) - 1;
    let mut max_x = xs.iter().map(|x| x.0).max().unwrap();
    max_x = ys.iter().map(|x| (x.1).0).max().unwrap().max(max_x) + 2;

    let mut min_y = ys.iter().map(|y| y.0).min().unwrap();
    min_y = xs.iter().map(|y| (y.1).0).min().unwrap().min(min_y);
    let mut max_y = ys.iter().map(|y| y.0).max().unwrap();
    max_y = xs.iter().map(|y| (y.1).1).max().unwrap().max(max_y) + 1;

    for _ in min_y..max_y + 1 {
        let mut row: Vec<char> = vec![];
        for _ in 0..(max_x - min_x) {
            row.push(' ');
        }
        board.push(row);
    }

    for r in xs {
        let x = r.0;
        for y in (r.1).0..(r.1).1 + 1 {
            board[y - min_y + 1][x - min_x] = '#';
        }
    }
    for r in ys {
        let y = r.0;
        for x in (r.1).0..(r.1).1 + 1 {
            board[y - min_y + 1][x - min_x] = '#';
        }
    }

    board[0][500 - min_x] = '+';

    board
}

fn day17(start_board: &Vec<Vec<char>>) -> (usize, usize) {
    let mut board: Vec<Vec<char>> = start_board.clone();
    let mut queue: Vec<(usize, usize)> = vec![(board[0].iter().position(|x| *x == '+').unwrap(), 0)];
    while !queue.is_empty() {
        let mut pos = queue.pop().unwrap();
        let mut changed = true;
        while changed {
            changed = false;
            let (x, y) = pos;
            if y + 1 >= board.len() {
                break;
            } else if board[y + 1][x] == ' ' {
                board[y + 1][x] = '|';
                pos = (x, y + 1);
                changed = true;
            } else if (board[y + 1][x] == '#' || board[y + 1][x] == '~') && board[y][x] == '|' {
                let mut is_open = false;
                let mut start_x = x;
                let mut end_x = x;
                for s in (0..x).rev() {
                    if board[y][s] == '#' {
                        start_x = s + 1;
                        break;
                    } else if board[y + 1][s] == ' ' || board[y + 1][s] == '|' {
                        start_x = s;
                        is_open = true;
                        break;
                    }
                }
                for s in x + 1..board[0].len() {
                    if board[y][s] == '#' {
                        end_x = s - 1;
                        break;
                    } else if board[y + 1][s] == ' ' || board[y + 1][s] == '|' {
                        end_x = s;
                        is_open = true;
                        break;
                    }
                }

                if is_open {
                    for s in start_x..end_x + 1 {
                        board[y][s] = '|';
                    }
                    queue.push((x, y - 1));
                    if pos != (start_x, y) {
                        queue.push((start_x, y));
                    }
                    if pos != (end_x, y) {
                        queue.push((end_x, y));
                    }
                } else {
                    for s in start_x..end_x + 1 {
                        board[y][s] = '~';
                    }
                    pos = (pos.0, (pos.1 as i64 - 1) as usize);
                    changed = true;
                }
            }
        }
    }

    (
        board.iter().map(|x| x.iter().filter(|c| **c == '|' || **c == '~').count()).sum(),
        board.iter().map(|x| x.iter().filter(|c| **c == '~').count()).sum()
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let board = parse(&input.split("\n").collect());

    let (part1, part2) = day17(&board);
    println!("part1 {:?}", part1);
    println!("part2 {:?}", part2);
}
