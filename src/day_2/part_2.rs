use std::fs;

use crate::day_2::game_parsing;

pub fn run() {
    let filename = "./src/day_2/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut powers: Vec<i32> = Vec::new();
    for line in lines {
        let parsed_game = game_parsing::parse_game(line);
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for result in parsed_game.results.iter() {
            if result.red > min_red {
                min_red = result.red;
            }

            if result.green > min_green {
                min_green = result.green;
            }

            if result.blue > min_blue {
                min_blue = result.blue;
            }
        }

        powers.push(min_red * min_green * min_blue);
    }

    println!("The answer is: {:?}", powers.iter().sum::<i32>());
}
