use std::fs;

pub fn run() {
    let filename = "./src/day_6/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let races = parse_races(lines);

    let ways_to_win_races = simulate_races(races);

    println!(
        "The answer is: {:?}",
        ways_to_win_races.iter().product::<i32>()
    );
}

fn simulate_races(races: Vec<Race>) -> Vec<i32> {
    let mut ways_to_win_races: Vec<i32> = Vec::new();
    for race in races {
        let ways_to_win_race = calculate_ways_to_win(race);
        ways_to_win_races.push(ways_to_win_race);
    }

    return ways_to_win_races;
}

fn calculate_ways_to_win(race: Race) -> i32 {
    let mut ways_to_win = 0;
    // Ignoring accelleration time of 0 since that can never win a race
    for accelleration_time in 1..race.time {
        let race_win = simulate_race(&race, accelleration_time as f32);
        if race_win {
            ways_to_win += 1;
        } else if ways_to_win > 0 {
            break;
        }
    }

    return ways_to_win;
}

fn simulate_race(race: &Race, accelleration_time: f32) -> bool {
    let velocity: f32 = accelleration_time;

    let time_for_distance = (race.distance as f32) / velocity;

    return (accelleration_time + time_for_distance) < race.time as f32;
}

fn parse_races(lines: Vec<&str>) -> Vec<Race> {
    let timesheet = parse_timesheet_numbers(lines[0].replace("Time:", ""));
    let distancesheet = parse_timesheet_numbers(lines[1].replace("Distance:", ""));

    let mut races: Vec<Race> = Vec::new();
    for (index, time) in timesheet.iter().enumerate() {
        races.push(Race {
            time: *time,
            distance: distancesheet[index],
        })
    }

    return races;
}

fn parse_timesheet_numbers(line: String) -> Vec<i32> {
    return line
        .split_whitespace()
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}
