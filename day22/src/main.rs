use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(usize),
    RotateLeft,
    RotateRight,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        match s {
            "L" => Ok(Instruction::RotateLeft),
            "R" => Ok(Instruction::RotateRight),
            _ => Ok(Instruction::Move(s.parse::<usize>().unwrap())),
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    start: usize,
    end: usize,
    walls: HashSet<usize>,
}

#[derive(Debug, Clone)]
struct Map {
    rows: Vec<Row>,
    start_and_end_per_column: Vec<(usize, usize)>,
}

fn load_data(filename: &str) -> io::Result<(Map, Vec<Instruction>)> {
    let lines = read_lines(filename)?
        .map(Result::<String, Error>::unwrap)
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut rows = Vec::new();
    let mut start_and_end_per_column = vec![(100000, 0); 161];

    let mut idx = 0;
    while !lines[idx].is_empty() {
        // Find starting pos
        let mut row_start_idx = 0;
        while row_start_idx < lines[idx].len() && lines[idx][row_start_idx] == ' ' {
            row_start_idx += 1;
        }

        let mut walls = HashSet::new();
        for i in row_start_idx..lines[idx].len() {
            start_and_end_per_column[i].0 = cmp::min(start_and_end_per_column[i].0, idx); // row start
            start_and_end_per_column[i].1 = cmp::max(start_and_end_per_column[i].1, idx + 1); // row end

            if lines[idx][i] == '#' {
                walls.insert(i);
            }
        }

        rows.push(Row {
            start: row_start_idx,
            end: lines[idx].len(),
            walls,
        });

        idx += 1;
    }

    idx += 1;

    let instrut_regex = Regex::new(r"(\d+|L|R)").unwrap();
    let instructions_string = String::from_iter(lines[idx].iter());
    let instructions = instrut_regex
        .find_iter(&instructions_string)
        .map(|x| x.as_str().parse::<Instruction>().unwrap())
        .collect();

    Ok((
        Map {
            rows,
            start_and_end_per_column,
        },
        instructions,
    ))
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn task1(map: &Map, instructions: &Vec<Instruction>) -> usize {
    let mut pos = (0, map.rows[0].start);
    let mut direction = Direction::Right;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::RotateLeft => {
                direction = match direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                };
            }
            Instruction::RotateRight => {
                direction = match direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
            }
            Instruction::Move(n) => {
                let num_steps = *n;
                match direction {
                    Direction::Right => {
                        for _ in 0..num_steps {
                            let next_y = if (pos.1 + 1) == map.rows[pos.0].end {
                                map.rows[pos.0].start
                            } else {
                                pos.1 + 1
                            };
                            if map.rows[pos.0].walls.contains(&next_y) {
                                // Hit a wall!
                                break;
                            } else {
                                pos.1 = next_y;
                            }
                        }
                    }
                    Direction::Down => {
                        for _ in 0..num_steps {
                            let next_x = if (pos.0 + 1) == map.start_and_end_per_column[pos.1].1 {
                                map.start_and_end_per_column[pos.1].0
                            } else {
                                pos.0 + 1
                            };
                            if map.rows[next_x].walls.contains(&pos.1) {
                                // Hit a wall!
                                break;
                            } else {
                                pos.0 = next_x;
                            }
                        }
                    }
                    Direction::Left => {
                        for _ in 0..num_steps {
                            let next_y = if pos.1 == map.rows[pos.0].start {
                                map.rows[pos.0].end - 1
                            } else {
                                pos.1 - 1
                            };
                            if map.rows[pos.0].walls.contains(&next_y) {
                                // Hit a wall!
                                break;
                            } else {
                                pos.1 = next_y;
                            }
                        }
                    }
                    Direction::Up => {
                        for _ in 0..num_steps {
                            let next_x = if pos.0 == map.start_and_end_per_column[pos.1].0 {
                                map.start_and_end_per_column[pos.1].1 - 1
                            } else {
                                pos.0 - 1
                            };
                            if map.rows[next_x].walls.contains(&pos.1) {
                                // Hit a wall!
                                break;
                            } else {
                                pos.0 = next_x;
                            }
                        }
                    }
                }
            }
        }
    }

    let direction_score = match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + direction_score
}

const CUBE_SIDE_LENGTH: usize = 50;

fn task2(map: &Map, instructions: &Vec<Instruction>) -> usize {
    // Manually mapped input!
    let mut right_wrap_around_mapping = HashMap::new();
    let mut down_wrap_around_mapping = HashMap::new();
    let mut left_wrap_around_mapping = HashMap::new();
    let mut up_wrap_around_mapping = HashMap::new();

    for i in 0..CUBE_SIDE_LENGTH {
        // 1)
        down_wrap_around_mapping.insert((49, 100 + i), (50 + i, 99, Direction::Left));
        right_wrap_around_mapping.insert((50 + i, 99), (49, 100 + i, Direction::Up));

        // 2)
        right_wrap_around_mapping.insert((i, 149), (149 - i, 99, Direction::Left));
        right_wrap_around_mapping.insert((149 - i, 99), (i, 149, Direction::Left));

        // 3)
        down_wrap_around_mapping.insert((149, 50 + i), (150 + i, 49, Direction::Left));
        right_wrap_around_mapping.insert((150 + i, 49), (149, 50 + i, Direction::Up));

        // 4)
        up_wrap_around_mapping.insert((0, 100 + i), (199, i, Direction::Up));
        down_wrap_around_mapping.insert((199, i), (0, 100 + i, Direction::Down));

        // 5)
        left_wrap_around_mapping.insert((50 + i, 50), (100, i, Direction::Down));
        up_wrap_around_mapping.insert((100, i), (50 + i, 50, Direction::Right));

        // 6)
        left_wrap_around_mapping.insert((i, 50), (149 - i, 0, Direction::Right));
        left_wrap_around_mapping.insert((149 - i, 0), (i, 50, Direction::Right));

        // 7)
        up_wrap_around_mapping.insert((0, 50 + i), (150 + i, 0, Direction::Right));
        left_wrap_around_mapping.insert((150 + i, 0), (0, 50 + i, Direction::Down));
    }

    let mut pos = (0, map.rows[0].start);
    let mut direction = Direction::Right;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::RotateLeft => {
                direction = match direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                };
            }
            Instruction::RotateRight => {
                direction = match direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
            }
            Instruction::Move(n) => {
                let num_steps = *n;
                for _ in 0..num_steps {
                    let (next_x, next_y, next_dir) = match direction {
                        Direction::Right => {
                            if (pos.1 + 1) == map.rows[pos.0].end {
                                *right_wrap_around_mapping.get(&pos).unwrap()
                            } else {
                                (pos.0, pos.1 + 1, direction)
                            }
                        }
                        Direction::Down => {
                            if (pos.0 + 1) == map.start_and_end_per_column[pos.1].1 {
                                *down_wrap_around_mapping.get(&pos).unwrap()
                            } else {
                                (pos.0 + 1, pos.1, direction)
                            }
                        }
                        Direction::Left => {
                            if pos.1 == map.rows[pos.0].start {
                                *left_wrap_around_mapping.get(&pos).unwrap()
                            } else {
                                (pos.0, pos.1 - 1, direction)
                            }
                        }
                        Direction::Up => {
                            if pos.0 == map.start_and_end_per_column[pos.1].0 {
                                *up_wrap_around_mapping.get(&pos).unwrap()
                            } else {
                                (pos.0 - 1, pos.1, direction)
                            }
                        }
                    };

                    if map.rows[next_x].walls.contains(&next_y) {
                        // Hit a wall!
                        break;
                    } else {
                        pos = (next_x, next_y);
                        direction = next_dir;
                    }
                }
            }
        }
    }

    let direction_score = match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + direction_score
}

fn main() -> io::Result<()> {
    let (map, instructions) = load_data("input.txt")?;

    // Task 1
    let out_task1 = task1(&map, &instructions);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(&map, &instructions);
    println!("Task 2: {}", out_task2);

    Ok(())
}
