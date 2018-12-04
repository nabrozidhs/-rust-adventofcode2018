extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

#[derive(Debug)]
struct Log {
    minute: u8,
    action: String,
}

fn parse(input: &str) -> Log {
    let re = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})] (.+)").unwrap();
    let cap = re.captures(input).unwrap();
    Log {
        minute: cap[1].parse().unwrap(),
        action: cap[2].parse().unwrap(),
    }
}

fn day4(logs: &Vec<Log>, f: fn(&(&u64, &HashMap<u8, u64>)) -> u64) -> u64 {
    let mut sleep_schedule: HashMap<u64, HashMap<u8, u64>> = HashMap::new();

    let mut current_guard: u64 = 0;
    let mut start_sleep = 0;
    for log in logs {
        if log.action.starts_with("Guard") {
            current_guard = log.action.split_whitespace().skip(1).next().unwrap()[1..].parse().unwrap();
        } else if log.action == "falls asleep" {
            start_sleep = log.minute
        } else {
            if !sleep_schedule.contains_key(&current_guard) {
                sleep_schedule.insert(current_guard, HashMap::new());
            }

            let mut m = sleep_schedule.get_mut(&current_guard).unwrap();
            for x in start_sleep..log.minute {
                match m.get(&x).cloned() {
                    Some(v) => { m.insert(x, v + 1); }
                    None => { m.insert(x, 1); }
                }
            }
        }
    }

    let (guard, schedule) = sleep_schedule.iter().max_by_key(f).unwrap();
    let minute = schedule.iter().max_by_key(|x| x.1).unwrap().0;

    guard * (*minute as u64)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut digits: Vec<&str> = buffer.lines().collect();
    digits.sort_by_key(|x| &x[1..17]);
    let logs: Vec<Log> = digits.iter().map(|x| parse(x)).collect();

    println!("part1 {}", day4(&logs, |x| x.1.values().sum::<u64>()));
    println!("part2 {}", day4(&logs, |x| *x.1.values().max().unwrap()));
}
