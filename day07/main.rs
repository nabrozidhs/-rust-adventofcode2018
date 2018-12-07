use std::collections::HashMap;
use std::io;
use std::io::Read;

fn parse(input: &str) -> (char, char) {
    let result: Vec<&str> = input.split_whitespace().collect();

    (
        result.get(1).unwrap().chars().last().unwrap(),
        result.get(7).unwrap().chars().last().unwrap()
    )
}

fn job_time(step: char, delay_per_step: i64, enable_task_delay: bool) -> i64 {
    if enable_task_delay {
        step as i64 - 64 + delay_per_step
    } else {
        delay_per_step
    }
}

fn day07(input: &Vec<(char, char)>, num_workers: u8, delay_per_step: i64, enable_task_delay: bool) -> (String, u64) {
    let mut connections: HashMap<char, Vec<char>> = HashMap::new();
    let mut results: HashMap<char, i64> = HashMap::new();
    let mut workers: HashMap<u8, i64> = HashMap::new();
    for w in 0..num_workers {
        workers.insert(w, -1);
    }

    // Build graph
    for connection in input {
        if connections.contains_key(&connection.0) {
            connections.get_mut(&connection.0).unwrap().push(connection.1);
        } else {
            connections.insert(connection.0, vec![connection.1]);
        }

        if !connections.contains_key(&connection.1) {
            connections.insert(connection.1, vec![]);
        }
    }

    let mut queue: Vec<char> = vec![];
    for starting_node in connections.iter().filter(|x| connections.iter().all(|e| !e.1.contains(x.0))) {
        queue.push(*starting_node.0);
        queue.sort();
    }
    let mut seconds = 0;
    while results.len() != connections.len() {
        for finished_job in results.iter().filter(|e| *e.1 == seconds) {
            for new_job in connections.get(finished_job.0).unwrap().iter()
                .filter(|c| {
                    connections.iter()
                        .filter(|x| x.1.contains(c))
                        .all(|x| results.get(x.0).map(|v| *v <= seconds).unwrap_or(false))
                }) {
                queue.push(*new_job);
            };
            queue.sort();
        }

        while !queue.is_empty() && workers.iter().any(|w| *w.1 <= seconds) {
            let new_job = queue.remove(0);
            let finish_time = seconds + job_time(new_job, delay_per_step, enable_task_delay);
            let next_worker = *workers.iter().find(|w| *w.1 <= seconds).unwrap().0;
            workers.insert(next_worker, finish_time);
            results.insert(new_job, finish_time);
        }
        seconds += 1;
    }

    let mut output: Vec<(&char, &i64)> = results.iter().collect();
    output.sort_by_key(|x| x.1);
    (
        output.into_iter().map(|x| x.0).collect::<String>(),
        *results.iter().map(|x| x.1).max().unwrap() as u64
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parsed: Vec<(char, char)> = input.split("\n").map(|x| parse(x)).collect();

    println!("part1 {}", day07(&parsed, 1, 1, false).0);
    println!("part2 {}", day07(&parsed, 5, 60, true).1);
}