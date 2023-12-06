use std::fs;

pub fn run() {
    let filename = "./src/day_6/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let race = parse_races(lines);

    let ways_to_win_race = calculate_ways_to_win(race);

    println!("The answer is: {:?}", ways_to_win_race);
}

fn calculate_ways_to_win(race: Race) -> i64 {
    let mut ways_to_win = 0;
    // Ignoring accelleration time of 0 since that can never win a race
    for accelleration_time in 1..(race.time + 1) {
        let race_win = simulate_race(&race, accelleration_time as f64);
        if race_win {
            ways_to_win += 1;
        } else if ways_to_win > 0 {
            break;
        }
    }

    return ways_to_win;
}

fn simulate_race(race: &Race, accelleration_time: f64) -> bool {
    let velocity = accelleration_time;

    let time_for_distance = (race.distance as f64) / velocity;

    return (accelleration_time + time_for_distance) < race.time as f64;
}

fn parse_races(lines: Vec<&str>) -> Race {
    let time = parse_timesheet_numbers(lines[0].replace("Time:", ""));
    let distance = parse_timesheet_numbers(lines[1].replace("Distance:", ""));

    return Race { time, distance };
}

fn parse_timesheet_numbers(line: String) -> i64 {
    return line
        .split_whitespace()
        .collect::<String>()
        .trim()
        .parse()
        .unwrap();
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}
