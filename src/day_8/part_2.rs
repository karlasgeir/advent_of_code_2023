use std::{char, collections::HashMap, fs};

use num_integer::Integer;

pub fn run() {
    let filename = "./src/day_8/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let directions = get_instructions(lines[0]);
    let map = parse_map(lines);

    let steps = map
        .keys()
        .filter(|k| k.ends_with("A"))
        // Find the steps it takes for each starting point to reach the end
        .map(|k| traverse_map(&map, k, &directions))
        // Find the least common multiple, I thought I would have to do some
        // more cycle detection, but it seems like the cycles are all the same as
        // getting from A to Z the first time
        .reduce(|a, b| a.lcm(&b))
        .unwrap();

    println!("The answer is: {:?}", steps);
}

fn traverse_map(
    map: &HashMap<String, MapLocation>,
    start: &str,
    directions: &Vec<Direction>,
) -> i64 {
    // Initial position
    let mut current_position = &map[start];
    let mut steps = 0;
    let mut direction_index = 0;
    loop {
        steps += 1;
        let next_position_id = match directions[direction_index] {
            Direction::Left => &current_position.left,
            Direction::Right => &current_position.right,
        };

        if next_position_id.ends_with("Z") {
            break;
        }

        current_position = &map[next_position_id];
        if direction_index == directions.len() - 1 {
            direction_index = 0;
        } else {
            direction_index += 1;
        }
    }

    steps
}

fn parse_map(lines: Vec<&str>) -> HashMap<String, MapLocation> {
    let mut map = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        if index == 0 || line.is_empty() {
            continue;
        }
        let parts = line.split("=").collect::<Vec<&str>>();
        let location_id = parts[0].trim();
        let next_location_parts: Vec<String> = parts[1]
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();
        let left = next_location_parts[0].trim();
        let right = next_location_parts[1].trim();
        let map_location = MapLocation {
            left: left.to_string(),
            right: right.to_string(),
        };
        map.insert(location_id.to_string(), map_location);
    }

    return map;
}

fn get_instructions(line: &str) -> Vec<Direction> {
    return line
        .chars()
        .map(|c| map_to_direction(c))
        .collect::<Vec<Direction>>();
}

fn map_to_direction(char: char) -> Direction {
    match char {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Unknown direction"),
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct MapLocation {
    left: String,
    right: String,
}
