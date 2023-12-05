use std::{collections::HashMap, fs};

pub fn run() {
    let filename = "./src/day_5/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let input_seeds = parse_seeds(lines[0]);

    let maps = parse_maps(lines);

    let mut min_location = std::i64::MAX;
    for seed in input_seeds {
        let soil = traverse_map(seed, maps.get("seed-to-soil").unwrap());
        let fertiliser = traverse_map(soil, maps.get("soil-to-fertilizer").unwrap());
        let water = traverse_map(fertiliser, maps.get("fertilizer-to-water").unwrap());
        let light = traverse_map(water, maps.get("water-to-light").unwrap());
        let temperature = traverse_map(light, maps.get("light-to-temperature").unwrap());
        let humidity = traverse_map(temperature, maps.get("temperature-to-humidity").unwrap());
        let location = traverse_map(humidity, maps.get("humidity-to-location").unwrap());
        if location < min_location {
            min_location = location;
        }
    }

    println!("The answer is: {:?}", min_location);
}

fn traverse_map(source: i64, map_lines: &Vec<MapLine>) -> i64 {
    for line in map_lines {
        if (source >= line.source) && (source <= line.source + line.range) {
            return line.destination + (source - line.source);
        }
    }

    return source;
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let seeds_string = line.replace("seeds: ", "");
    let seeds = seeds_string.split_whitespace().collect::<Vec<&str>>();
    return seeds
        .iter()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
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
