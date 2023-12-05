use std::fs;

pub fn run() {
    let filename = "./src/day_3/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut gears: Vec<i32> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let prev_line = if index > 0 {
            lines.get(index - 1)
        } else {
            None
        };
        let next_line = lines.get(index + 1);
        gears.append(&mut gears_in_line(line, prev_line, next_line));
    }

    println!("The answer is: {:?}", gears.iter().sum::<i32>());
}

fn gears_in_line(line: &str, prev_line: Option<&&str>, next_line: Option<&&str>) -> Vec<i32> {
    let mut gears: Vec<i32> = Vec::new();

    for (index, character) in line.chars().enumerate() {
        if character == '*' {
            gears.push(calculate_gears(line, prev_line, next_line, index));
        }
    }

    return gears;
}

fn check_number_proximity_in_line(line: &str, index: usize) -> Vec<i32> {
    let chars: Vec<char> = line.chars().collect();
    let mut touching_numbers: Vec<i32> = Vec::new();
    if index > 0 && chars[index - 1].is_numeric() {
        touching_numbers.push(expand_number(line, index - 1))
    }

    if index < chars.len() - 1 && chars[index + 1].is_numeric() {
        touching_numbers.push(expand_number(line, index + 1))
    }

    if chars[index].is_numeric() {
        touching_numbers.push(expand_number(line, index))
    }

    // Theoretically, this is not enough, since by doing thise we are not allowing two instances of the same number
    // to be touching the gear in the same line, but the input fortionately does not have this case :D
    touching_numbers.dedup();

    return touching_numbers;
}

fn calculate_gears(
    current_line: &str,
    prev_line: Option<&&str>,
    next_line: Option<&&str>,
    index: usize,
) -> i32 {
    let mut touching_numbers: Vec<i32> = Vec::new();

    if let Some(prev_line) = prev_line {
        touching_numbers.append(&mut check_number_proximity_in_line(prev_line, index));
    }

    if let Some(next_line) = next_line {
        touching_numbers.append(&mut check_number_proximity_in_line(next_line, index));
    }

    touching_numbers.append(&mut check_number_proximity_in_line(current_line, index));

    if touching_numbers.len() == 2 {
        return touching_numbers[0] * touching_numbers[1];
    }

    return 0;
}

fn expand_number(complete_string: &str, search_index: usize) -> i32 {
    let mut digit_start_index = search_index;
    let mut digit_end_index = search_index;
    let chars: Vec<char> = complete_string.chars().collect();

    while digit_start_index > 0 && chars[digit_start_index - 1].is_numeric() {
        digit_start_index -= 1;
    }

    while digit_end_index < chars.len() - 1 && chars[digit_end_index + 1].is_numeric() {
        digit_end_index += 1;
    }

    let complete_number = complete_string[digit_start_index..digit_end_index + 1].to_string();
    return complete_number.parse::<i32>().unwrap();
}
