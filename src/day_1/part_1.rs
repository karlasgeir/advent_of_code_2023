use std::fs;

pub fn run() {
    let filename = "./src/day_1/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut calibration_values: Vec<i32> = Vec::new();
    for line in lines {
        calibration_values.push( extract_calibration_values(line));
    }

    println!("The answer is: {:?}", calibration_values.iter().sum::<i32>());
}

fn extract_calibration_values(
    line: &str,
) -> i32 {
    let number_chars: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
    
    if number_chars.len() < 1 {
        return 0;
    }

    let first_number = number_chars[0];
    let second_number = number_chars[number_chars.len() - 1];

    let number = format!("{}{}", first_number, second_number);

    return number.parse::<i32>().unwrap();
}