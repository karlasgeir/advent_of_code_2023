use std::fs;

pub fn run() {
    let filename = "./src/day_9/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let scores = lines
        .iter()
        .map(|l| find_prev_history_value(l))
        .collect::<Vec<i32>>();

    println!("The answer is: {:?}", scores.iter().sum::<i32>());
}

fn find_prev_history_value(line: &str) -> i32 {
    let mut next_history: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect();

    // Initialize the history vector
    let mut history: Vec<Vec<i32>> = Vec::new();
    history.push(next_history.clone());

    loop {
        next_history = get_next_history_line(next_history.clone());
        history.push(next_history.clone());
        if next_history.is_empty() || next_history.iter().all(|&n| n == 0) {
            break;
        }
    }

    return extrapolate(history);
}

fn get_next_history_line(last_history: Vec<i32>) -> Vec<i32> {
    let mut next_history: Vec<i32> = Vec::new();
    for (index, value) in last_history.iter().enumerate() {
        if index == last_history.len() - 1 {
            continue;
        }

        next_history.push(last_history[index + 1] - value);
    }

    return next_history;
}

fn extrapolate(history: Vec<Vec<i32>>) -> i32 {
    let mut history = history.clone();
    history.reverse();
    let mut last_extrapolated = 0;

    for (index, _) in history.iter().enumerate() {
        if index == history.len() - 1 {
            break;
        }

        let last_history_line = history[index + 1].clone();
        let last_prev_item = last_history_line[0];

        last_extrapolated = last_prev_item - last_extrapolated;
    }

    return last_extrapolated;
}
