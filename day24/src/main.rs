use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
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
enum Blizzard {
    North,
    South,
    East,
    West,
}

impl FromStr for Blizzard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Blizzard, Self::Err> {
        match s {
            "^" => Ok(Blizzard::North),
            ">" => Ok(Blizzard::East),
            "<" => Ok(Blizzard::West),
            "v" => Ok(Blizzard::South),
            _ => Err(Error::new(ErrorKind::Other, "Parsing error")),
        }
    }
}

fn load_data(filename: &str) -> io::Result<Vec<Vec<Vec<Blizzard>>>> {
    Ok(read_lines(filename)?
        .map(|l| {
            let row = l.unwrap();

            let mut new_row = Vec::new();
            for i in 0..row.len() {
                new_row.push(Vec::new());
                let c = &row[i..i + 1];
                if !c.starts_with(".") && !c.starts_with("#") {
                    new_row[i].push(c.parse::<Blizzard>().unwrap());
                }
            }

            new_row
        })
        .collect())
}

fn move_blizzards(field: &Vec<Vec<Vec<Blizzard>>>) -> Vec<Vec<Vec<Blizzard>>> {
    let mut new_field = Vec::new();
    for i in 0..field.len() {
        new_field.push(Vec::new());
        for j in 0..field[i].len() {
            new_field[i].push(Vec::new());
        }
    }

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j].is_empty() {
                continue;
            }

            for blizzard_dir in field[i][j].iter() {
                match blizzard_dir {
                    Blizzard::North => {
                        let next_row = if i == 1 { field.len() - 2 } else { i - 1 };
                        new_field[next_row][j].push(Blizzard::North);
                    }
                    Blizzard::South => {
                        let next_row = if i == (field.len() - 2) { 1 } else { i + 1 };
                        new_field[next_row][j].push(Blizzard::South);
                    }
                    Blizzard::West => {
                        let next_col = if j == 1 { field[i].len() - 2 } else { j - 1 };
                        new_field[i][next_col].push(Blizzard::West);
                    }
                    Blizzard::East => {
                        let next_col = if j == (field[i].len() - 2) { 1 } else { j + 1 };
                        new_field[i][next_col].push(Blizzard::East);
                    }
                }
            }
        }
    }

    new_field
}

fn print_blizzards(field: &Vec<Vec<Vec<Blizzard>>>) {
    for i in 1..(field.len() - 1) {
        for j in 1..(field[i].len() - 1) {
            if field[i][j].is_empty() {
                print!(".");
            } else if field[i][j].len() == 1 {
                match field[i][j][0] {
                    Blizzard::North => print!("^"),
                    Blizzard::South => print!("v"),
                    Blizzard::East => print!("<"),
                    Blizzard::West => print!(">"),
                }
            } else {
                print!("{}", field[i][j].len());
            }
        }
        println!("");
    }
}

fn shortest_path(
    mut field: Vec<Vec<Vec<Blizzard>>>,
    start: (usize, usize),
    target: (usize, usize),
) -> (usize, Vec<Vec<Vec<Blizzard>>>) {
    let mut first_time_visited = vec![vec![None; field[0].len()]; field.len()];

    let mut queue = HashSet::new();
    queue.insert(start);

    let n: i64 = field.len() as i64 - 1;
    let m: i64 = field[0].len() as i64 - 1;

    let mut stop = false;
    let mut minute = 0;
    while !stop {
        minute += 1;
        field = move_blizzards(&field);

        let mut next_queue = HashSet::new();

        for (x, y) in queue.into_iter() {
            if (x + 1, y) == target || (x - 1, y) == target {
                stop = true;
                break;
            }

            if first_time_visited[x][y].is_none() {
                first_time_visited[x][y] = Some(minute);
            }

            // 1. Option: wait
            if field[x][y].is_empty() {
                next_queue.insert((x, y));
            }

            // 2. Option: try different positions
            for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_x = x as i64 + i;
                let new_y = y as i64 + j;

                if 0 < new_x
                    && new_x < n
                    && 0 < new_y
                    && new_y < m
                    && field[new_x as usize][new_y as usize].is_empty()
                    && (first_time_visited[new_x as usize][new_y as usize].is_none()
                        || (minute - first_time_visited[new_x as usize][new_y as usize].unwrap()
                            < field.len()))
                {
                    next_queue.insert((new_x as usize, new_y as usize));
                }
            }
        }

        queue = next_queue;
    }

    (minute, field)
}

fn main() -> io::Result<()> {
    let data = load_data("input.txt")?;

    let start = (0, 1);
    let target = (data.len() - 1, data[0].len() - 2);

    // Task 1
    let (out_task1, field) = shortest_path(data, start, target);
    println!("Task 1: {}", out_task1);

    // Task 2
    let (out_task2_p2, field2) = shortest_path(field, target, start);
    let (out_task2_p3, _) = shortest_path(field2, start, target);
    let out_task2 = out_task1 + out_task2_p2 + out_task2_p3;
    println!("Task 2: {}", out_task2);

    Ok(())
}
