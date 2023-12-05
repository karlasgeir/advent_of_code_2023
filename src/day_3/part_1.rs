use std::fs;

pub fn run() {
    let filename = "./src/day_3/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let symbols_map = map_symbols(&lines);

    let mut valid_numbers: Vec<i32> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        valid_numbers.append(&mut valid_numbers_in_line(line, index, &symbols_map));
    }

    println!("The answer is: {:?}", valid_numbers.iter().sum::<i32>());
}

fn has_adjacent_symbol_in_row(
    row_index: usize,
    column_index: usize,
    symbols_map: &Vec<Vec<bool>>,
) -> bool {
    let row = &symbols_map[row_index];
    if column_index > 0 && row[column_index - 1] {
        return true;
    }

    if column_index < row.len() - 1 && row[column_index + 1] {
        return true;
    }

    return row[column_index];
}

fn has_adjacent_symbol_in_adjacent_rows(
    row_index: usize,
    column_index: usize,
    symbols_map: &Vec<Vec<bool>>,
) -> bool {
    if row_index > 0 && has_adjacent_symbol_in_row(row_index - 1, column_index, symbols_map) {
        return true;
    }

    if row_index < symbols_map.len() - 1
        && has_adjacent_symbol_in_row(row_index + 1, column_index, symbols_map)
    {
        return true;
    }

    return has_adjacent_symbol_in_row(row_index, column_index, symbols_map);
}

fn valid_numbers_in_line(line: &str, row: usize, symbols_map: &Vec<Vec<bool>>) -> Vec<i32> {
    let mut valid_numbers: Vec<i32> = Vec::new();

    let mut current_number = String::from("");
    let mut has_adjacent_symbol = false;
    for (index, character) in line.chars().enumerate() {
        if character.is_numeric() {
            current_number.push(character);

            if !has_adjacent_symbol {
                has_adjacent_symbol = has_adjacent_symbol_in_adjacent_rows(row, index, symbols_map);
            }
        } else {
            if has_adjacent_symbol && current_number.len() > 0 {
                valid_numbers.push(current_number.parse::<i32>().unwrap());
            }
            has_adjacent_symbol = false;
            current_number.clear();
        }
    }

    // If the last number is a digit, we need to check if it's valid
    if has_adjacent_symbol && current_number.len() > 0 {
        valid_numbers.push(current_number.parse::<i32>().unwrap());
    }

    return valid_numbers;
}

fn is_symbol(symbol: char) -> bool {
    return !symbol.is_numeric() && !symbol.is_whitespace() && symbol != '.';
}

fn map_symbols(lines: &Vec<&str>) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        let mut row: Vec<bool> = Vec::new();
        for character in line.chars() {
            row.push(is_symbol(character));
        }
        map.push(row);
    }

    return map;
}
