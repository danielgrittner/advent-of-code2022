use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Elements {
    Air,
    Rock,
    Sand,
}

fn load_data(filename: &str) -> io::Result<(Vec<Vec<Elements>>, usize)> {
    let mut data = vec![vec![Elements::Air; 1_000]; 170];

    let lines: Vec<String> = read_lines(filename)?
        .map(Result::<String, Error>::unwrap)
        .collect();

    let coordinate_regex = Regex::new(r"\d+,\d+").unwrap();
    let coord_extract_regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();

    let mut max_y = 0;

    for line in lines.iter() {
        // (y,x)-pairs where y represents the row and x the column
        let coordinates: Vec<(usize, usize)> = coordinate_regex
            .find_iter(&line)
            .map(|coord_str| {
                let capture = coord_extract_regex.captures(coord_str.as_str()).unwrap();
                (
                    capture["y"].parse::<usize>().unwrap(),
                    capture["x"].parse::<usize>().unwrap(),
                )
            })
            .collect();

        if coordinates.is_empty() {
            continue;
        }

        max_y = cmp::max(max_y, coordinates[0].0);

        for i in 0..(coordinates.len() - 1) {
            let (y1, x1) = coordinates[i];
            let (y2, x2) = coordinates[i + 1];
            max_y = cmp::max(max_y, y2);

            if y1 == y2 {
                for i in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                    data[y1][i] = Elements::Rock;
                }
            } else {
                for i in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                    data[i][x1] = Elements::Rock;
                }
            }
        }
    }

    Ok((data, max_y + 2))
}

fn task1(mut data: Vec<Vec<Elements>>) -> usize {
    let start_point = (0, 500); // (y, x)

    let mut units = 0;
    loop {
        // Let a sand unit drop
        let mut cur_pos = start_point;
        let mut is_stuck = false;
        while !is_stuck && (cur_pos.0 + 1) < data.len() {
            if data[cur_pos.0 + 1][cur_pos.1] == Elements::Air {
                cur_pos.0 += 1;
            } else if data[cur_pos.0 + 1][cur_pos.1 - 1] == Elements::Air {
                cur_pos.0 += 1;
                cur_pos.1 -= 1;
            } else if data[cur_pos.0 + 1][cur_pos.1 + 1] == Elements::Air {
                cur_pos.0 += 1;
                cur_pos.1 += 1;
            } else {
                is_stuck = true;
                units += 1;
                data[cur_pos.0][cur_pos.1] = Elements::Sand;
            }
        }

        if (cur_pos.0 + 1) >= data.len() {
            break;
        }
    }
    units
}

fn task2(mut data: Vec<Vec<Elements>>, floor: usize) -> usize {
    let start_point = (0, 500); // (y, x)

    let mut units = 0;
    while data[0][500] != Elements::Sand {
        // Let a sand unit drop
        let mut cur_pos = start_point;
        let mut is_stuck = false;
        while !is_stuck && (cur_pos.0 + 1) < floor {
            if data[cur_pos.0 + 1][cur_pos.1] == Elements::Air {
                cur_pos.0 += 1;
            } else if data[cur_pos.0 + 1][cur_pos.1 - 1] == Elements::Air {
                cur_pos.0 += 1;
                cur_pos.1 -= 1;
            } else if data[cur_pos.0 + 1][cur_pos.1 + 1] == Elements::Air {
                cur_pos.0 += 1;
                cur_pos.1 += 1;
            } else {
                is_stuck = true;
            }
        }

        data[cur_pos.0][cur_pos.1] = Elements::Sand;
        units += 1;
    }

    units
}

fn main() -> io::Result<()> {
    let (data, floor) = load_data("input.txt")?;

    // Task 1
    let out_task1 = task1(data.clone());
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(data, floor);
    println!("Task 2: {}", out_task2);

    Ok(())
}
