use std::fs;

pub fn run() {
    let filename = "./src/day_10/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let start_position = find_start_position(lines.clone());
    let pipes = parse_pipes(lines);

    let loop_pipes = find_loop(pipes.clone(), start_position, Orientation::East).unwrap();

    println!("The answer is: {:?}", loop_pipes.len() / 2 + 1);
}

fn find_loop(
    pipes: Vec<Vec<Option<Pipe>>>,
    start_position: (usize, usize),
    start_orientation: Orientation,
) -> Option<Vec<Pipe>> {
    let mut loop_pipes: Vec<Pipe> = Vec::new();
    let mut current_orientation = start_orientation;
    let mut current_position = start_position;
    loop {
        current_position = match current_orientation {
            Orientation::North => (current_position.0 - 1, current_position.1),
            Orientation::East => (current_position.0, current_position.1 + 1),
            Orientation::South => (current_position.0 + 1, current_position.1),
            Orientation::West => (current_position.0, current_position.1 - 1),
        };

        let next_pipe = match current_orientation {
            Orientation::North => pipes[current_position.0][current_position.1],
            Orientation::East => pipes[current_position.0][current_position.1],
            Orientation::South => pipes[current_position.0][current_position.1],
            Orientation::West => pipes[current_position.0][current_position.1],
        };

        if next_pipe.is_none() {
            // Successfully found the loop
            if current_position == start_position {
                return Some(loop_pipes);
            }

            // No loop here, break
            break;
        }

        let next_pipe = next_pipe.unwrap();
        loop_pipes.push(next_pipe);

        if next_pipe.start_orientation.opposite() == current_orientation {
            current_orientation = next_pipe.end_orientation;
        } else if next_pipe.end_orientation.opposite() == current_orientation {
            current_orientation = next_pipe.start_orientation;
        } else {
            println!("No loop here, break");
            break;
        }
    }

    let next_start_orientation = match current_orientation {
        Orientation::North => Orientation::East,
        Orientation::East => Orientation::South,
        Orientation::South => Orientation::West,
        Orientation::West => Orientation::North,
    };

    // We have checked all orientations from the starting position
    if next_start_orientation == Orientation::East {
        return None;
    }

    return find_loop(pipes.clone(), start_position, next_start_orientation);
}

fn find_start_position(lines: Vec<&str>) -> (usize, usize) {
    for (row_index, line) in lines.iter().enumerate() {
        for (column_index, character) in line.chars().enumerate() {
            if character == 'S' {
                return (row_index, column_index);
            }
        }
    }

    return (0, 0);
}

fn parse_pipes(lines: Vec<&str>) -> Vec<Vec<Option<Pipe>>> {
    let mut pipes: Vec<Vec<Option<Pipe>>> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<Option<Pipe>> = Vec::new();
        for (x, character) in line.chars().enumerate() {
            match character {
                '-' => row.push(Some(Pipe {
                    start_orientation: Orientation::West,
                    end_orientation: Orientation::East,
                })),
                '|' => row.push(Some(Pipe {
                    start_orientation: Orientation::North,
                    end_orientation: Orientation::South,
                })),
                'L' => row.push(Some(Pipe {
                    start_orientation: Orientation::North,
                    end_orientation: Orientation::East,
                })),
                'J' => row.push(Some(Pipe {
                    start_orientation: Orientation::North,
                    end_orientation: Orientation::West,
                })),
                '7' => row.push(Some(Pipe {
                    start_orientation: Orientation::West,
                    end_orientation: Orientation::South,
                })),
                'F' => row.push(Some(Pipe {
                    start_orientation: Orientation::East,
                    end_orientation: Orientation::South,
                })),
                _ => row.push(None),
            }
        }
        pipes.push(row);
    }

    return pipes;
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    start_orientation: Orientation,
    end_orientation: Orientation,
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn opposite(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::South,
            Orientation::East => Orientation::West,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
        }
    }
}

impl PartialEq for Orientation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Orientation::North, Orientation::North) => true,
            (Orientation::East, Orientation::East) => true,
            (Orientation::South, Orientation::South) => true,
            (Orientation::West, Orientation::West) => true,
            _ => false,
        }
    }
}
