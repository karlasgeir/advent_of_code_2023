use std::{collections::HashSet, fs};

pub fn run() {
    let filename = "./src/day_4/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut cards_won = vec![1; lines.len()];
    for (index, line) in lines.iter().enumerate() {
        let won_cards = count_winning_numbers(line) as usize;
        for card_index in 0..won_cards {
            let game_index = index + card_index;
            if game_index + 1 >= cards_won.len() {
                break;
            }
            cards_won[game_index + 1] += cards_won[index];
        }
    }

    println!("The answer is: {:?}", cards_won.iter().sum::<i32>());
}

fn count_winning_numbers(line: &str) -> i32 {
    let cards_parts = line.split(":").collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>();
    let winning_numbers = cards_parts[0]
        .trim()
        .split_whitespace()
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();
    let game_numbers = cards_parts[1]
        .trim()
        .split_whitespace()
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();

    let intersection = winning_numbers
        .intersection(&game_numbers)
        .collect::<Vec<&i32>>();

    return intersection.len() as i32;
}
