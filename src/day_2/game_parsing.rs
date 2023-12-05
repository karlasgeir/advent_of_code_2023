pub fn parse_game(line: &str) -> Game {
    let game_split: Vec<_> = line.split(": ").collect();
    let game_id = game_split[0].replace("Game ", "").parse::<i32>().unwrap();
    let game_content = game_split[1];

    let mut game = Game {
        id: game_id,
        results: Vec::new(),
    };

    for game_string in game_content.split(";") {
        game.results.push(parse_sub_game(game_string));
    }

    return game;
}

pub fn parse_sub_game(line: &str) -> GameResult {
    let mut game_result = GameResult {
        blue: 0,
        red: 0,
        green: 0,
    };
    for part in line.split(", ") {
        let result_split: Vec<_> = part.trim().split(" ").collect();
        let value = result_split[0].parse::<i32>().unwrap();
        let color = result_split[1];

        match color {
            "blue" => game_result.blue = value,
            "red" => game_result.red = value,
            "green" => game_result.green = value,
            _ => panic!("Unknown color: {}", color),
        }
    }

    return game_result;
}

pub struct Game {
    pub id: i32,
    pub results: Vec<GameResult>,
}

pub struct GameResult {
    pub blue: i32,
    pub red: i32,
    pub green: i32,
}
