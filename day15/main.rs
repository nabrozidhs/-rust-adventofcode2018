extern crate core;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

type Position = (i32, i32);

fn adjacent_positions(position: &Position) -> Vec<Position> {
    return vec![
        (position.0, position.1 - 1),
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 + 1),
    ];
}

#[derive(Clone, Debug, Copy)]
struct Unit {
    unit_type: char,
    hp: i64,
    attack_power: i64,
}

impl Unit {
    fn is_dead(&self) -> bool { self.hp <= 0 }
}

struct State {
    walls: HashSet<Position>,
    units: HashMap<Position, Unit>,
    width: i32,
    height: i32,
    elf_died: bool,
}

impl State {
    fn sorted_unit_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = self.units.keys().copied()
            .filter(|x| !self.units.get(x).unwrap().is_dead())
            .collect();
        positions.sort_by_key(|x| x.0);
        positions.sort_by_key(|x| x.1);
        positions
    }

    fn total_hp(&self) -> i64 { self.sorted_units().iter().map(|x| x.hp).sum() }

    fn sorted_units(&self) -> Vec<&Unit> {
        self.sorted_unit_positions().iter().map(|x| self.units.get(x).unwrap())
            .collect()
    }

    fn is_position_blocked(&self, position: &Position) -> bool {
        let has_unit_alive = match self.units.get(position) {
            Some(unit) => !unit.is_dead(),
            _ => false,
        };
        return self.walls.contains(position) || has_unit_alive;
    }

    fn _find_position_for_unit(&self, unit: &Unit) -> Position {
        for (pos, u) in self.units.iter() {
            if self::std::ptr::eq(u, unit) {
                return *pos;
            }
        }
        panic!();
    }

    fn is_finished(&self) -> bool {
        let filtered: Vec<&Unit> = self.sorted_units();
        filtered.iter().all(|u| u.unit_type == 'G') || filtered.iter().all(|u| u.unit_type == 'E')
    }

    fn adjacent_unit_to_attack(&self, attacking_unit_position: &Position) -> Option<Position> {
        let attacking_unit = self.units.get(attacking_unit_position).unwrap();
        let attack_type = if attacking_unit.unit_type == 'G' { 'E' } else { 'G' };
        adjacent_positions(&attacking_unit_position).into_iter()
            .filter(|p| {
                let unit = self.units.get(p);
                unit.is_some() && unit.unwrap().unit_type == attack_type && !unit.unwrap().is_dead()
            })
            .min_by_key(|x| self.units.get(x).unwrap().hp)
    }

    fn find_targets_for(&self, moving_unit: &Unit) -> HashSet<Position> {
        let moving_unit_position = self._find_position_for_unit(moving_unit);
        let mut all_targets: HashSet<Position> = HashSet::new();

        let for_type = if moving_unit.unit_type == 'G' { 'E' } else { 'G' };
        for unit in self.units.values().filter(|x| x.unit_type == for_type && !x.is_dead()) {
            for p in adjacent_positions(&self._find_position_for_unit(unit)) {
                all_targets.insert(p);
            }
        }

        let mut targets: HashSet<Position> = HashSet::new();
        if all_targets.contains(&moving_unit_position) {
            return targets;
        }

        for p in all_targets {
            if !self.is_position_blocked(&p) {
                targets.insert(p);
            }
        }

        targets
    }

    fn _find_closest_target_to_position(&self, position: &Position, targets: &HashSet<Position>) -> Option<Position> {
        if targets.is_empty() {
            return None;
        }
        if targets.contains(position) {
            return Some(*position);
        }
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(*position);

        let mut matches: HashSet<Position> = HashSet::new();
        let mut bfs: Vec<Vec<Position>> = vec![];
        bfs.push(adjacent_positions(position));
        while !bfs.is_empty() {
            let mut next_batch: Vec<Position> = vec![];
            matches.clear();
            for pos in bfs.pop().unwrap() {
                if self.is_position_blocked(&pos) || visited.contains(&pos) {
                    continue;
                }

                visited.insert(pos);
                if targets.contains(&pos) {
                    matches.insert(pos);
                }
                for p in adjacent_positions(&pos) {
                    next_batch.push(p);
                }
            }

            if !next_batch.is_empty() {
                bfs.push(next_batch);
            }
            if !matches.is_empty() {
                break;
            }
        }

        if matches.is_empty() {
            return None;
        } else {
            let mut sorted_matches: Vec<Position> = matches.into_iter().collect();
            sorted_matches.sort_by_key(|x| x.0);
            sorted_matches.sort_by_key(|x| x.1);
            return Some(*sorted_matches.first().unwrap());
        }
    }

    fn move_to_closest(&mut self, start_position: &mut Position) {
        let targets = self.find_targets_for(
            self.units.get(start_position).unwrap()
        );

        let closest_target = self._find_closest_target_to_position(&start_position, &targets);
        if closest_target.is_none() {
            return;
        }

        let mut near_targets: HashSet<Position> = HashSet::new();
        for p in adjacent_positions(&start_position) {
            if !self.is_position_blocked(&p) {
                near_targets.insert(p);
            }
        }

        if let Some(where_to_move) = self._find_closest_target_to_position(
            &closest_target.unwrap(),
            &near_targets,
        ) {
            let unit = self.units.remove(&start_position).unwrap();
            self.units.insert(where_to_move, unit);
            start_position.0 = where_to_move.0;
            start_position.1 = where_to_move.1;
        }
    }

    fn round(&mut self) -> bool {
        if self.is_finished() {
            return false;
        }

        let mut skip_positions: HashSet<Position> = HashSet::new();
        let mut positions = self.sorted_unit_positions();
        let mut is_completed_early = false;
        for position in positions.iter_mut() {
            if skip_positions.contains(&position) {
                continue;
            }

            if is_completed_early {
                return false;
            }

            let unit = *self.units.get(&position).unwrap();

            self.move_to_closest(position);
            if let Some(enemy_position) = self.adjacent_unit_to_attack(&position) {
                let enemy = self.units.get_mut(&enemy_position).unwrap();
                enemy.hp -= unit.attack_power;
                if enemy.is_dead() {
                    skip_positions.insert(enemy_position);
                    if enemy.unit_type == 'E' {
                        self.elf_died = true;
                    }
                    is_completed_early = self.is_finished();
                }
            }
        }

        return true;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x as i32, y as i32);
                if self.walls.contains(&pos) {
                    print!("#");
                } else {
                    print!(
                        "{}",
                        match self.units.get(&pos) {
                            Some(unit) =>
                                if unit.is_dead() { '.' } else { unit.unit_type },
                            None => '.',
                        }
                    );
                }
            }
            println!();
        }

        for unit in self.sorted_units() {
            if !unit.is_dead() {
                println!("{}: {}", unit.unit_type, unit.hp);
            }
        }
    }
}

fn parse(input: &Vec<&str>, elf_attack_power: i64) -> State {
    let mut walls: HashSet<Position> = HashSet::new();
    let mut units: HashMap<Position, Unit> = HashMap::new();

    let mut y = 0;
    let mut x = 0;
    for line in input {
        x = 0;
        for c in line.trim().chars() {
            if c == '#' {
                walls.insert((x as i32, y as i32));
            } else if c == 'G' || c == 'E' {
                units.insert(
                    (x, y),
                    Unit {
                        unit_type: c,
                        hp: 200,
                        attack_power: if c == 'G' { 3 } else { elf_attack_power },
                    },
                );
            }
            x += 1;
        }

        y += 1;
    }

    State { walls, units, width: x, height: y, elf_died: false }
}

fn day15(start_state: State) -> (u64, bool) {
    let mut state = start_state;
    let mut rounds = 0;

    while state.round() {
        rounds += 1;
    }

    state.print();
    (rounds * state.total_hp() as u64, state.elf_died)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input = buffer.trim().split("\n").collect();

    let (score, _) = day15(parse(&input, 3));
    println!("part1 {}", score);

    for elf_attack_power in 1.. {
        let state = parse(&input, elf_attack_power);
        let (score, elf_died) = day15(state);
        if !elf_died {
            println!("part2 {}", score);
            break;
        }
    }
}
