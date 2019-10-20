use std::collections::{HashMap, HashSet};

type Position = (usize, usize);

fn adjacent_positions(p: &Position) -> Vec<Position> {
    let mut positions = vec![(p.0, p.1 + 1), (p.0 + 1, p.1)];
    if p.0 > 0 {
        positions.push((p.0 - 1, p.1));
    }
    if p.1 > 0 {
        positions.push((p.0, p.1 - 1));
    }
    positions
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl Region {
    fn from_usize(value: usize) -> Region {
        match value % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => panic!(),
        }
    }

    fn risk_level(&self) -> usize {
        match self {
            Region::Rocky => 0,
            Region::Wet => 1,
            Region::Narrow => 2,
        }
    }

    fn available_tools(&self) -> HashSet<Tool> {
        let mut tools = HashSet::new();
        match self {
            Region::Rocky => {
                tools.insert(Tool::ClimbingGear);
                tools.insert(Tool::Torch);
            }
            Region::Wet => {
                tools.insert(Tool::ClimbingGear);
                tools.insert(Tool::Neither);
            }
            Region::Narrow => {
                tools.insert(Tool::Torch);
                tools.insert(Tool::Neither);
            }
        }
        tools
    }
}

struct Cave {
    erosion: HashMap<Position, usize>,
    target: Position,
    depth: usize,
}

impl Cave {
    fn init(depth: usize, target: Position) -> Cave {
        Cave { erosion: HashMap::new(), target, depth }
    }

    fn _get_erosion_level(&mut self, position: &Position) -> usize {
        if let Some(erosion) = self.erosion.get(position) {
            return *erosion;
        }

        let (x, y) = *position;
        let erosion_level = if *position == self.target {
            self.depth % 20183
        } else if x == 0 || y == 0 {
            ((x * 16807) + (y * 48271) + self.depth) % 20183
        } else {
            (self._get_erosion_level(&(x, y - 1)) * self._get_erosion_level(&(x - 1, y)) + self.depth) % 20183
        };
        self.erosion.insert(*position, erosion_level);
        erosion_level
    }

    fn get_risk_level(&mut self, position: &Position) -> Region {
        Region::from_usize(self._get_erosion_level(position))
    }

    fn risk_level(&mut self) -> usize {
        let mut sum = 0;
        for y in 0..self.target.1 + 1 {
            for x in 0..self.target.0 + 1 {
                sum += self.get_risk_level(&(x, y)).risk_level();
            }
        }
        sum
    }
}

struct Path {
    path: Vec<Position>,
    has_reached: bool,
    minutes: usize,
    current_tool: Tool,
}

impl Path {
    fn init() -> Path {
        Path { path: vec![(0, 0)], has_reached: false, minutes: 0, current_tool: Tool::Torch }
    }

    fn adjacent_paths(&self, cave: &mut Cave) -> Vec<Path> {
        let mut new_paths: Vec<Path> = vec![];
        let last = self.path.last().unwrap();

        if *last == cave.target {
            let mut path = vec![];
            for p in self.path.iter() {
                path.push(*p);
            }
            new_paths.push(Path { path, has_reached: true, minutes: self.minutes + 7, current_tool: Tool::Torch });
            return new_paths;
        }

        for pos in adjacent_positions(last) {
            if self.path.contains(&pos) {
                continue;
            }

            for tool in cave.get_risk_level(last).available_tools().intersection(&cave.get_risk_level(&pos).available_tools()) {
                let mut path = vec![];
                for p in self.path.iter() {
                    path.push(*p);
                }
                path.push(pos);

                let minutes = if self.current_tool != *tool {
                    self.minutes + 7 + 1
                } else {
                    self.minutes + 1
                };
                let has_reached = cave.target == pos && *tool == Tool::Torch;

                new_paths.push(Path { path, minutes, has_reached, current_tool: *tool });
            }
        }
        new_paths
    }
}

fn day22(depth: usize, target: Position) -> (usize, usize) {
    let mut cave = Cave::init(depth, target);

    let mut paths: Vec<Path> = vec![];
    paths.push(Path::init());
    let mut visited: HashMap<(Position, Tool), usize> = HashMap::new();
    visited.insert(((0, 0), Tool::Torch), 0);

    loop {
        let path = paths.pop().unwrap();
        if path.has_reached {
            return (cave.risk_level(), path.minutes);
        }
        for p in path.adjacent_paths(&mut cave) {
            let last = p.path.last().unwrap();
            let key = (*last, p.current_tool);
            let minutes = visited.get(&key).unwrap_or(&usize::max_value());
            if *minutes > p.minutes {
                visited.insert(key, p.minutes);
                paths.push(p);
            }
        }
        paths.sort_by_key(|x| -(x.minutes as i64));
    }
}

fn main() {
//    let (part1, part2) = day22(510, (10, 10));
    let (part1, part2) = day22(4848, (15, 700));
    println!("part1 {:?}", part1);
    println!("part2 {:?}", part2);
}
