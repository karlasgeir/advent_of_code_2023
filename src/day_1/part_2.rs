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
    let mut digits_map: Vec<Option<String>> = vec![None; line.len()];
    let string_digits = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    for (word, number) in string_digits.iter() {
        for (index, _) in line.match_indices(word) {
            digits_map[index] = Some(number.to_string());
        }
    }

    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            digits_map[index] = Some(char.to_string());
        }
    }

    let number_chars: Vec<_> = digits_map.into_iter().filter(|c| c.is_some()).collect();

    let first_number = number_chars[0].clone().unwrap();
    let second_number = number_chars[number_chars.len() - 1].clone().unwrap();

    let number = format!("{}{}", first_number, second_number);

    return number.parse::<i32>().unwrap();
}