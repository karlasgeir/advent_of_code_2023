use std::{collections::HashSet, fs};

pub fn run() {
    let filename = "./src/day_4/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let card_scores = lines
        .iter()
        .map(|l| get_card_score(l))
        .collect::<Vec<i32>>();

    println!("The answer is: {:?}", card_scores.iter().sum::<i32>());
}

fn get_card_score(line: &str) -> i32 {
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

    let mut score = 0;
    for i in 0..intersection.len() {
        if i == 0 {
            score = 1;
        } else {
            score *= 2;
        }
    }

    return score;
}
