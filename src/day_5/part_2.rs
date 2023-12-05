use std::{collections::HashMap, fs, ops::RangeInclusive};

pub fn run() {
    let filename = "./src/day_5/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let input_seed_pairs = parse_seeds(lines[0]);

    let maps = parse_maps(lines);

    for location in 0..std::i64::MAX {
        let humidity = reverse_traverse_map(location, maps.get("humidity-to-location").unwrap());
        let temperature =
            reverse_traverse_map(humidity, maps.get("temperature-to-humidity").unwrap());
        let light = reverse_traverse_map(temperature, maps.get("light-to-temperature").unwrap());
        let water = reverse_traverse_map(light, maps.get("water-to-light").unwrap());
        let fertiliser = reverse_traverse_map(water, maps.get("fertilizer-to-water").unwrap());
        let soil = reverse_traverse_map(fertiliser, maps.get("soil-to-fertilizer").unwrap());
        let seed = reverse_traverse_map(soil, maps.get("seed-to-soil").unwrap());

        for seed_pair in &input_seed_pairs {
            if seed_pair.contains(&seed) {
                println!("The answer is: {:?}", location);
                return;
            }
        }
    }

    println!("Could not find answer");
}

fn reverse_traverse_map(destination: i64, map_lines: &Vec<MapLine>) -> i64 {
    for line in map_lines {
        if (destination >= line.destination) && (destination < line.destination + line.range) {
            return line.source + (destination - line.destination);
        }
    }

    return destination;
}

fn parse_seeds(line: &str) -> Vec<RangeInclusive<i64>> {
    let seed_pairs_full_string = line.replace("seeds: ", "");
    let seed_pairs_string = seed_pairs_full_string
        .split_whitespace()
        .collect::<Vec<&str>>();
    let seed_pairs_flat = seed_pairs_string
        .iter()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seed_pairs: Vec<RangeInclusive<i64>> = Vec::new();
    for i in (0..seed_pairs_flat.len()).step_by(2) {
        seed_pairs.push(seed_pairs_flat[i]..=seed_pairs_flat[i] + seed_pairs_flat[i + 1] - 1);
    }

    return seed_pairs;
}

fn parse_maps(lines: Vec<&str>) -> HashMap<String, Vec<MapLine>> {
    let mut maps = HashMap::new();
    let mut current_map_name: Option<String> = None;
    let mut current_map_lines: Vec<MapLine> = Vec::new();
    for line in lines {
        if line.contains("seeds") {
            continue;
        }
        if line.contains("map:") {
            current_map_name = Some(line.replace(" map:", ""));
        } else if line.len() > 0 {
            let columns = line.split_whitespace().collect::<Vec<&str>>();
            let map_line = MapLine {
                destination: columns[0].parse::<i64>().unwrap(),
                source: columns[1].parse::<i64>().unwrap(),
                range: columns[2].parse::<i64>().unwrap(),
            };
            current_map_lines.push(map_line);
        } else if current_map_name.is_some() {
            maps.insert(current_map_name.unwrap(), current_map_lines);
            current_map_name = None;
            current_map_lines = Vec::new();
        }
    }

    // Handle the last line
    if current_map_name.is_some() {
        maps.insert(current_map_name.unwrap(), current_map_lines);
    }

    return maps;
}

struct MapLine {
    destination: i64,
    source: i64,
    range: i64,
}
