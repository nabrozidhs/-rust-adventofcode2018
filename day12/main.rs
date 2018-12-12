use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn parse(input: &Vec<&str>) -> (Vec<bool>, HashSet<(bool, bool, bool, bool, bool)>) {
    let mut initial_state: Vec<bool> = vec![];
    let first_line: &str = input.get(0).unwrap()
        .split_whitespace().collect::<Vec<&str>>().get(2).unwrap();
    for c in first_line.chars() {
        initial_state.push(c == '#');
    }

    let mut rules: HashSet<(bool, bool, bool, bool, bool)> = HashSet::new();
    for i in 2..input.len() {
        let split = input.get(i).unwrap().split_whitespace().collect::<Vec<&str>>();
        if split.get(2).unwrap().parse::<char>().unwrap() == '.' {
            continue;
        }
        let mut chars_iter = split.get(0).unwrap().chars();
        rules.insert(
            (
                chars_iter.next().unwrap() == '#',
                chars_iter.next().unwrap() == '#',
                chars_iter.next().unwrap() == '#',
                chars_iter.next().unwrap() == '#',
                chars_iter.next().unwrap() == '#',
            )
        );
    }

    (initial_state, rules)
}

fn day12(initial_state: &Vec<bool>,
         rules: &HashSet<(bool, bool, bool, bool, bool)>,
         memo: &mut HashMap<Vec<bool>, (Vec<bool>, i64)>,
         generations: u64) -> i64 {
    let mut state: Vec<bool> = initial_state.clone();
    let mut start_index = 0;

    let mut found_cycle_start: i64 = -1;
    for i in 0..generations {
        if memo.contains_key(&state) {
            found_cycle_start = i as i64;
            break;
        }
        let mut new_state: Vec<bool> = vec![];
        let mut movements = -2;
        for i in -2..state.len() as i64 + 2 {
            let to_check = (
                if i - 2 < 0 { false } else { *state.get((i - 2) as usize).unwrap_or(&false) },
                if i - 1 < 0 { false } else { *state.get((i - 1) as usize).unwrap_or(&false) },
                if i < 0 { false } else { *state.get(i as usize).unwrap_or(&false) },
                if i + 1 < 0 { false } else { *state.get((i + 1) as usize).unwrap_or(&false) },
                if i + 2 < 0 { false } else { *state.get((i + 2) as usize).unwrap_or(&false) },
            );
            new_state.push(rules.contains(&to_check));
        }
        while !new_state.get(0).unwrap() {
            movements += 1;
            new_state.remove(0);
        }
        while !new_state.get(new_state.len() - 1).unwrap() {
            new_state.pop();
        }

        memo.insert(state, (new_state.clone(), movements));
        start_index += movements;
        state = new_state;
    }

    if found_cycle_start >= 0 {
        let remaining = generations - found_cycle_start as u64;
        let mut cycle: Vec<(Vec<bool>, i64)> = vec![];
        let mut new_state = state.clone();
        for _ in found_cycle_start..generations as i64 {
            let v = memo.get(&new_state).unwrap().clone();
            if cycle.contains(&v) {
                break;
            }

            cycle.push(v.clone());
            new_state = v.0;
        }

        // find final state
        let (final_state, _) = cycle.get(remaining as usize % cycle.len()).unwrap();
        state = final_state.clone();
        let cycle_sum: i64 = cycle.iter().map(|x| x.1).sum();
        start_index += cycle_sum * (remaining as i64 / cycle.len() as i64);
        for i in 0..(remaining as usize % cycle.len()) {
            start_index += cycle.get(i).unwrap().1;
        }
    }

    let mut sum: i64 = 0;
    for i in 0..state.len() {
        if *state.get(i).unwrap() {
            sum += i as i64 + start_index;
        }
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (initial_state, rules) = parse(&input.split("\n").collect());

    println!("part1 {}", day12(&initial_state, &rules, &mut HashMap::new(), 20));
    println!("part2 {}", day12(&initial_state, &rules, &mut HashMap::new(), 50000000000));
}
