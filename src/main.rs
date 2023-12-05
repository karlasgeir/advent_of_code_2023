use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    let (day, part) = parse_args();

    match (day.as_str(), part.as_str()) {
        ("1", "1") => day_1::part_1::run(),
        ("1", "2") => day_1::part_2::run(),
        ("2", "1") => day_2::part_1::run(),
        ("2", "2") => day_2::part_2::run(),
        ("3", "1") => day_3::part_1::run(),
        ("3", "2") => day_3::part_2::run(),
        ("4", "1") => day_4::part_1::run(),
        ("4", "2") => day_4::part_2::run(),
        ("5", "1") => day_5::part_1::run(),
        ("5", "2") => day_5::part_2::run(),
        _ => println!("Invalid day or part"),
    }
}

fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <day> <part>");
        std::process::exit(1);
    }

    let day = &args[1];
    let part = &args[2];

    return (day.to_string(), part.to_string());
}
