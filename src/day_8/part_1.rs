use std::{char, collections::HashMap, fs};

pub fn run() {
    let filename = "./src/day_8/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let directions = get_instructions(lines[0]);
    let map = parse_map(lines);

    let steps = traverse_map(map, directions);

    println!("The answer is: {:?}", steps);
}

fn traverse_map(map: HashMap<String, MapLocation>, directions: Vec<Direction>) -> i32 {
    // Initial position
    let mut current_position = &map["AAA"];
    let mut steps = 0;
    let mut direction_index = 0;
    loop {
        steps += 1;
        let next_position_id = match directions[direction_index] {
            Direction::Left => &current_position.left,
            Direction::Right => &current_position.right,
        };

        if next_position_id == "ZZZ" {
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
