use std::collections::HashMap;
use std::io;
use std::io::Read;

fn parse(input: &Vec<&str>) -> (usize, Vec<(String, Vec<i64>)>) {
    let mut iter = input.iter();
    let pointer = iter.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

    let mut program: Vec<(String, Vec<i64>)> = vec![];
    for next in input.iter().skip(1) {
        let mut splitted = next.split_whitespace();
        program.push(
            (
                splitted.next().unwrap().to_string(),
                vec![
                    0,
                    splitted.next().unwrap().parse().unwrap(),
                    splitted.next().unwrap().parse().unwrap(),
                    splitted.next().unwrap().parse().unwrap(),
                ]
            )
        );
    }
    (pointer, program)
}

fn addr(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] + registers[command[2] as usize];
}

fn addi(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] + command[2];
}

fn mulr(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] * registers[command[2] as usize];
}

fn muli(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] * command[2];
}

fn banr(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] & registers[command[2] as usize];
}

fn bani(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] & command[2];
}

fn borr(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] | registers[command[2] as usize];
}

fn bori(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize] | command[2];
}

fn setr(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = registers[command[1] as usize];
}

fn seti(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = command[1];
}

fn gtir(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = if command[1] > registers[command[2] as usize] { 1 } else { 0 };
}

fn gtri(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = if registers[command[1] as usize] > command[2] { 1 } else { 0 };
}

fn gtrr(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = if registers[command[1] as usize] > registers[command[2] as usize] { 1 } else { 0 };
}

fn eqir(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = if command[1] == registers[command[2] as usize] { 1 } else { 0 };
}

fn eqri(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = if registers[command[1] as usize] == command[2] { 1 } else { 0 };
}

fn eqrr(command: &Vec<i64>, registers: &mut Vec<i64>) {
    registers[command[3] as usize] = if registers[command[1] as usize] == registers[command[2] as usize] { 1 } else { 0 };
}

fn day19(program: &Vec<(String, Vec<i64>)>, pointer: usize, registers: &mut Vec<i64>) -> usize {
    let mut functions: HashMap<String, fn(&Vec<i64>, &mut Vec<i64>)> = HashMap::new();
    functions.insert("addr".to_string(), addr);
    functions.insert("addi".to_string(), addi);
    functions.insert("mulr".to_string(), mulr);
    functions.insert("muli".to_string(), muli);
    functions.insert("banr".to_string(), banr);
    functions.insert("bani".to_string(), bani);
    functions.insert("borr".to_string(), borr);
    functions.insert("bori".to_string(), bori);
    functions.insert("setr".to_string(), setr);
    functions.insert("seti".to_string(), seti);
    functions.insert("gtir".to_string(), gtir);
    functions.insert("gtri".to_string(), gtri);
    functions.insert("gtrr".to_string(), gtrr);
    functions.insert("eqir".to_string(), eqir);
    functions.insert("eqri".to_string(), eqri);
    functions.insert("eqrr".to_string(), eqrr);
    while registers[pointer] >= 0 && (registers[pointer] as usize) < program.len() {
        functions[&program[registers[pointer] as usize].0](&program[registers[pointer] as usize].1, registers);
        registers[pointer] += 1;
    }

    registers[0] as usize
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (pointer, program) = parse(&input.split("\n").collect());

    println!("part1 {:?}", day19(&program, pointer, &mut vec![0, 0, 0, 0, 0, 0]));
//    println!("part2 {:?}", day19(&program, pointer, &mut vec![1, 0, 0, 0, 0, 0]));
}
