use std::fs;

use crate::day_2::game_parsing;

pub fn run() {
    let filename = "./src/day_2/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut successful_games: Vec<game_parsing::Game> = Vec::new();
    for line in lines {
        let parsed_game = game_parsing::parse_game(line);
        let mut failed = false;
        for result in parsed_game.results.iter() {
            if result.blue > max_blue || result.red > max_red || result.green > max_green {
                failed = true;
                break;
            }
        }

        if !failed {
            successful_games.push(parsed_game);
        }
    }

    let successful_game_ids: Vec<_> = successful_games.iter().map(|g| g.id).collect();
    println!(
        "The answer is: {:?}",
        successful_game_ids.iter().sum::<i32>()
    );
}
