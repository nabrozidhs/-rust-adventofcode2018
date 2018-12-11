use std::io;
use std::io::Read;

fn power_level(x: u64, y: u64, serial_number: u64) -> i64 {
    let rack_id = x + 10;
    let power_level = ((rack_id * y + serial_number) * rack_id / 100) % 10;
    power_level as i64 - 5
}

fn day10(serial_number: u64, search_for_size: bool) -> (usize, usize, u64) {
    let mut grid: Vec<Vec<i64>> = vec![];
    for x in 0..300 {
        let mut row: Vec<i64> = vec![];
        for y in 0..300 {
            row.push(power_level(x as u64, y as u64, serial_number));
        }
        grid.push(row);
    }

    let mut max_pos = (0, 0);
    let mut max_level = i64::min_value();
    let mut size = 1;
    let size_start_position = if search_for_size { 0 } else { 3 };
    let size_end_position = if search_for_size { 300 } else { 4 };
    for current_size in size_start_position..size_end_position {
        for x_outer in 0..301 - current_size {
            for y_outer in 0..301 - current_size {
                let mut current_level = 0;
                for x in x_outer..x_outer + current_size {
                    for y in y_outer..y_outer + current_size {
                        current_level += grid.get(x).unwrap().get(y).unwrap();
                    }
                }
                if current_level > max_level {
                    max_pos = (x_outer, y_outer);
                    max_level = current_level;
                    size = current_size as u64;
                }
            }
        }
    }

    (max_pos.0, max_pos.1, size)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parsed: u64 = input.parse().unwrap();

    println!("part1 {:?}", day10(parsed, false));
    println!("part2 {:?}", day10(parsed, true));
}
