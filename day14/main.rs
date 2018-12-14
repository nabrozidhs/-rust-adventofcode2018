use std::io;
use std::io::Read;

fn day14_part1(recipes: u64) -> String {
    let mut elves: Vec<usize> = vec![0, 1];
    let mut board: Vec<u64> = vec![3, 7];

    while board.len() < recipes as usize + 10 {
        let next = *board.get(*elves.get(0).unwrap()).unwrap() +
            *board.get(*elves.get(1).unwrap()).unwrap();
        let d = next / 10;
        let c = next % 10;
        if d != 0 {
            board.push(d);
        }
        board.push(c);

        elves = vec![
            (*elves.get(0).unwrap() + *board.get(*elves.get(0).unwrap()).unwrap() as usize + 1) % board.len(),
            (*elves.get(1).unwrap() + *board.get(*elves.get(1).unwrap()).unwrap() as usize + 1) % board.len(),
        ];
    }

    return board.iter().skip(recipes as usize)
        .take(10)
        .fold("".to_string(), |acc, x| acc + &x.to_string());
}

fn day14_part2(recipes: Vec<u64>) -> u64 {
    let mut elves: Vec<usize> = vec![0, 1];
    let mut board: Vec<u64> = vec![3, 7];

    while board[if board.len() > recipes.len() { board.len() - recipes.len() } else { 0 }..board.len()] != *recipes.as_slice() {
        let next = *board.get(*elves.get(0).unwrap()).unwrap() +
            *board.get(*elves.get(1).unwrap()).unwrap();
        let d = next / 10;
        let c = next % 10;
        if d != 0 {
            board.push(d);
            if board[if board.len() > recipes.len() { board.len() - recipes.len() } else { 0 }..board.len()] == *recipes.as_slice() {
                break
            }
        }
        board.push(c);

        elves = vec![
            (*elves.get(0).unwrap() + *board.get(*elves.get(0).unwrap()).unwrap() as usize + 1) % board.len(),
            (*elves.get(1).unwrap() + *board.get(*elves.get(1).unwrap()).unwrap() as usize + 1) % board.len(),
        ];
    }

    return (board.len() - recipes.len()) as u64;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("part1 {}", day14_part1(input.parse().unwrap()));
    println!(
        "part2 {}",
        day14_part2(
            input.chars().map(|x| x.to_digit(10).unwrap() as u64).collect()
        )
    );
}
