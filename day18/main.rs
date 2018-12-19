use std::collections::HashMap;
use std::io;
use std::io::Read;

fn parse(input: &Vec<&str>) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = vec![];

    for line in input {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        board.push(row);
    }

    board
}

fn surrounding(board: &Vec<Vec<char>>, position: (usize, usize)) -> (usize, usize) {
    let mut trees = 0;
    let mut lumberyards = 0;
    for y in position.1 as i64 - 1..position.1 as i64 + 2 {
        if y < 0 || y >= board.len() as i64 {
            continue;
        }
        for x in position.0 as i64 - 1..position.0 as i64 + 2 {
            if x < 0 ||
                x >= board[0].len() as i64 ||
                (x == position.0 as i64 && y == position.1 as i64) {
                continue;
            }
            match board[y as usize][x as usize] {
                '|' => trees += 1,
                '#' => lumberyards += 1,
                _ => {}
            }
        }
    }

    (trees, lumberyards)
}

fn day18(starting_board: &Vec<Vec<char>>, minutes: usize) -> usize {
    let mut board: Vec<Vec<char>> = starting_board.clone();

    let mut found_cycle_start: i64 = -1;
    let mut memo: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();
    for i in 0..minutes {
        let mut new_board: Vec<Vec<char>> = vec![];
        for y in 0..board.len() {
            let mut row: Vec<char> = vec![];
            for x in 0..board[0].len() {
                let pos = (x, y);
                let (trees, lumberyard) = surrounding(&board, pos);
                row.push(
                    match board[y][x] {
                        '.' => if trees >= 3 { '|' } else { '.' }
                        '|' => if lumberyard >= 3 { '#' } else { '|' }
                        '#' => if lumberyard >= 1 && trees >= 1 { '#' } else { '.' }
                        _ => panic!()
                    }
                )
            }
            new_board.push(row);
        }
        if memo.insert(board.clone(), new_board.clone()).is_some() {
            found_cycle_start = i as i64;
            board = new_board;
            break;
        }
        board = new_board;
    }

    if found_cycle_start >= 0 {
        let mut cycle: Vec<Vec<Vec<char>>> = vec![];
        while !cycle.contains(&board) {
            cycle.push(board.clone());
            board = memo[&board].clone();
        }
        let remaining = minutes - 1 - found_cycle_start as usize;
        board = cycle[remaining % cycle.len()].clone();
    }


    let mut trees = 0;
    let mut lumberyards = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            trees += if board[y][x] == '|' { 1 } else { 0 };
            lumberyards += if board[y][x] == '#' { 1 } else { 0 };
        }
    }
    trees * lumberyards
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let board = parse(&buffer.split("\n").collect());

    println!("part1 {}", day18(&board, 10));
    println!("part2 {}", day18(&board, 1000000000));
}
