extern crate regex;

use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Read;

use regex::Regex;

fn parse(input: &Vec<&str>, with_boost: u64) -> State {
    let mut system = System::Immune;
    let mut groups: Vec<Group> = vec![];
    for line in input {
        if line.len() == 0 {
            system = System::Infection;
            continue;
        }
        if line.chars().next().unwrap().is_digit(10) {
            groups.push(parse_line(system, line, with_boost));
        }
    }

    State { groups }
}

fn parse_line(system: System, input: &str, with_boost: u64) -> Group {
    let re = Regex::new(r"(\d+) units each with (\d+) hit points (\(?.*\)?) ?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    let mut immunities: HashSet<String> = HashSet::new();
    let mut weakness: HashSet<String> = HashSet::new();
    let immune_and_weakness = cap[3].parse::<String>().unwrap();
    if immune_and_weakness.len() > 0 {
        let p: &str = immune_and_weakness.get(1..immune_and_weakness.len() - 2).unwrap();
        for q in p.split(";") {
            let t: Vec<&str> = q.split_ascii_whitespace().into_iter().collect();
            for e in 2..t.len() {
                let d = t[e].split(",").next().unwrap().to_string();
                if t[0] == "weak" {
                    weakness.insert(d);
                } else {
                    immunities.insert(d);
                }
            }
        }
    }
    Group {
        system,
        units: cap[1].parse().unwrap(),
        hp_per_unit: cap[2].parse().unwrap(),
        attack: cap[4].parse::<u64>().unwrap() + if system == System::Immune { with_boost } else { 0 },
        attack_type: cap[5].parse().unwrap(),
        initiative: cap[6].parse().unwrap(),
        immunities,
        weakness,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum System {
    Immune,
    Infection,
}

#[derive(Debug)]
struct Group {
    system: System,
    units: u64,
    hp_per_unit: u64,
    attack_type: String,
    attack: u64,
    immunities: HashSet<String>,
    weakness: HashSet<String>,
    initiative: u64,
}

impl Group {
    fn is_dead(&self) -> bool { self.units == 0 }
    fn effective_power(&self) -> u64 { self.attack * self.units }
    fn damage_when_attacking(&self, other: &Group) -> u64 {
        self.effective_power() * if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weakness.contains(&self.attack_type) {
            2
        } else {
            1
        }
    }
    fn receive_damage(&mut self, damage: &u64) -> bool {
        let units_lost = damage / self.hp_per_unit;
        let damage_done = units_lost > 0;
        self.units = (self.units as i64 - units_lost as i64).max(0) as u64;
        damage_done
    }
}

#[derive(Debug)]
struct State {
    groups: Vec<Group>,
}

impl State {
    fn is_finished(&self) -> bool {
        self.groups.iter().filter(|x| x.system == System::Immune).all({ |x| x.is_dead() }) ||
            self.groups.iter().filter(|x| x.system == System::Infection).all({ |x| x.is_dead() })
    }

    fn result(&self) -> (u64, u64) {
        (
            self.groups.iter().filter(|x| x.system == System::Infection).map(|x| x.units).sum::<u64>(),
            self.groups.iter().filter(|x| x.system == System::Immune).map(|x| x.units).sum::<u64>()
        )
    }

    fn _attack_mapping(&self, group: &Group, defending_group: &Vec<usize>) -> Vec<usize> {
        let mut targets: Vec<usize> = defending_group.clone();
        targets.sort_by_key(|&x| -(self.groups[x].initiative as i64));
        targets.sort_by_key(|&x| -(self.groups[x].effective_power() as i64));
        targets.sort_by_key(|&x| -(group.damage_when_attacking(&self.groups[x]) as i64));
        targets.into_iter().filter(|&x| group.damage_when_attacking(&self.groups[x]) > 0).collect()
    }

    fn step(&mut self) -> (bool, bool) {
        // Target selection
        let mut immune_group: Vec<usize> = self.groups.iter().enumerate()
            .filter(|e| e.1.system == System::Immune)
            .map(|e| e.0)
            .collect();
        immune_group.sort_by_key(|&x| -(self.groups[x].initiative as i64));
        immune_group.sort_by_key(|&x| -(self.groups[x].effective_power() as i64));

        let mut infection_group: Vec<usize> = self.groups.iter().enumerate()
            .filter(|e| e.1.system == System::Infection)
            .map(|e| e.0)
            .collect();
        infection_group.sort_by_key(|&x| -(self.groups[x].initiative as i64));
        infection_group.sort_by_key(|&x| -(self.groups[x].effective_power() as i64));

        let mut attack_mapping: HashMap<usize, usize> = HashMap::new();

        for i in immune_group.iter() {
            let targets = self._attack_mapping(&self.groups[*i], &infection_group);
            let f = targets.iter()
                .filter(|&&x| !attack_mapping.values().any(|&m| m == x))
                .next();
            if let Some(&attack) = f {
                attack_mapping.insert(*i, attack);
            }
        }
        for i in infection_group.iter() {
            let targets = self._attack_mapping(&self.groups[*i], &immune_group);
            let f = targets.iter()
                .filter(|&&x| !attack_mapping.values().any(|&m| m == x))
                .next();
            if let Some(&attack) = f {
                attack_mapping.insert(*i, attack);
            }
        }

        let mut damage_done = false;
        let mut attack_vector: Vec<usize> = (0..self.groups.len()).collect();
        attack_vector.sort_by_key(|&x| -(self.groups[x].initiative as i64));
        for i in attack_vector {
            if self.groups.get(i).unwrap().is_dead() {
                continue;
            }

            if let Some(&defender_position) = attack_mapping.get(&i) {
                let damage = self.groups[i].damage_when_attacking(&self.groups[defender_position]);
                damage_done = self.groups.get_mut(defender_position).unwrap().receive_damage(&damage) || damage_done;
            }
        }

        self.groups.retain(|x| !x.is_dead());

        (self.is_finished(), damage_done)
    }
}

fn run(state: &mut State) -> (u64, u64) {
    loop {
        let (finished, damage_done) = state.step();
        if finished {
            return state.result();
        } else if !damage_done {
            return (state.result().0, 0);
        }
    }
}

fn day24_part1(state: &mut State) -> u64 {
    let (infection, immune) = run(state);
    infection.max(immune)
}

fn day24_part2(input: &Vec<&str>) -> u64 {
    let mut left = 0;
    let mut right = run(&mut parse(input, 0)).0;
    let mut result = (0, 0);
    while left <= right {
        let boost = (right + left) / 2;
        result = run(&mut parse(input, boost));
        if result.1 > result.0 {
            right = boost - 1;
        } else {
            left = boost + 1;
        }
    }
    result.1
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input = buffer.trim().split("\n").collect();

    println!("part1 {:?}", day24_part1(&mut parse(&input, 0)));
    println!("part2 {:?}", day24_part2(&input));
}
