use std::collections::HashMap;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Command {
    input: Vec<i64>,
    command: Vec<i64>,
    output: Vec<i64>,
}

fn parse(input: &Vec<&str>) -> (Vec<Command>, Vec<Vec<i64>>) {
    let mut commands: Vec<Command> = vec![];

    let mut iter = input.iter();
    loop {
        let input = iter.next();
        if input.is_none() || input.unwrap() == &"" {
            break;
        }

        commands.push(
            Command {
                input: input.unwrap()[9..19].split(", ").map(|x| x.parse::<i64>().unwrap()).collect(),
                command: iter.next().unwrap().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect(),
                output: iter.next().unwrap()[9..19].split(", ").map(|x| x.parse::<i64>().unwrap()).collect(),
            }
        );
        iter.next();
    }

    iter.next();
    let mut program: Vec<Vec<i64>> = vec![];
    loop {
        let input = iter.next();
        if input.is_none() {
            break;
        }

        program.push(input.unwrap().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect());
    }

    (commands, program)
}

fn addr(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] + registers[command[2] as usize];
    output
}

fn addi(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] + command[2];
    output
}

fn mulr(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] * registers[command[2] as usize];
    output
}

fn muli(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] * command[2];
    output
}

fn banr(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] & registers[command[2] as usize];
    output
}

fn bani(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] & command[2];
    output
}

fn borr(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] | registers[command[2] as usize];
    output
}

fn bori(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize] | command[2];
    output
}

fn setr(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = registers[command[1] as usize];
    output
}

fn seti(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = command[1];
    output
}

fn gtir(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = if command[1] > registers[command[2] as usize] { 1 } else { 0 };
    output
}

fn gtri(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = if registers[command[1] as usize] > command[2] { 1 } else { 0 };
    output
}

fn gtrr(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = if registers[command[1] as usize] > registers[command[2] as usize] { 1 } else { 0 };
    output
}

fn eqir(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = if command[1] == registers[command[2] as usize] { 1 } else { 0 };
    output
}

fn eqri(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = if registers[command[1] as usize] == command[2] { 1 } else { 0 };
    output
}

fn eqrr(command: &Vec<i64>, registers: &Vec<i64>) -> Vec<i64> {
    let mut output = registers.clone();
    output[command[3] as usize] = if registers[command[1] as usize] == registers[command[2] as usize] { 1 } else { 0 };
    output
}

fn day16_part1(commands: &Vec<Command>) -> u64 {
    let mut same = 0;
    let ops: Vec<fn(&Vec<i64>, &Vec<i64>) -> Vec<i64>> =
        vec![addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];
    for command in commands {
        let mut same_counter = 0;
        for op in ops.iter() {
            if command.output == op(&command.command, &command.input) {
                same_counter += 1;
                if same_counter >= 3 {
                    break;
                }
            }
        }

        if same_counter >= 3 {
            same += 1;
        }
    }
    same
}

fn day16_part2(commands: &Vec<Command>, program: &Vec<Vec<i64>>) -> i64 {
    let mut mapping: HashMap<i64, fn(&Vec<i64>, &Vec<i64>) -> Vec<i64>> = HashMap::new();
    let mut missing: Vec<fn(&Vec<i64>, &Vec<i64>) -> Vec<i64>> =
        vec![addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];
    loop {
        for command in commands {
            if mapping.contains_key(&command.command[0]) {
                continue;
            }

            for command in commands {
                let mut matches = 0;
                let mut matched: Option<(i64, usize)> = None;
                for i in 0..missing.len() {
                    let m = missing[i];
                    if m(&command.command, &command.input) == command.output {
                        matches += 1;
                        matched = Some((command.command[0], i));
                    }
                }
                if matches == 1 {
                    mapping.insert(
                        matched.unwrap().0,
                        missing.remove(matched.unwrap().1),
                    );
                }
            }
        }

        if missing.is_empty() {
            break;
        }
    }

    let mut registers: Vec<i64> = vec![0, 0, 0, 0];
    for p in program {
        registers = mapping[&p[0]](&p, &registers);
    }
    registers[0]
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let (commands, program) = parse(&buffer.split("\n").collect());

    println!("part1 {}", day16_part1(&commands));
    println!("part2 {}", day16_part2(&commands, &program));
}
