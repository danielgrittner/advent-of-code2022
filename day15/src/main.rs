use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coordinate(i32, i32);

#[derive(Debug)]
struct Sensor {
    location: Coordinate,
    closest_beacon_location: Coordinate,
}

impl Sensor {
    fn calc_distance_to_closest_beacon(&self) -> usize {
        // Manhattan Distance
        ((self.location.0 - self.closest_beacon_location.0).abs()
            + (self.location.1 - self.closest_beacon_location.1).abs()) as usize
    }
}

fn load_data(filename: &str) -> io::Result<Vec<Sensor>> {
    let sensor_regex = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();

    Ok(read_lines(filename)?
        .map(|l| {
            let line_str = l.unwrap();
            let captures = sensor_regex.captures(&line_str).unwrap();
            Sensor {
                location: Coordinate(
                    captures["sensor_x"].parse::<i32>().unwrap(),
                    captures["sensor_y"].parse::<i32>().unwrap(),
                ),
                closest_beacon_location: Coordinate(
                    captures["beacon_x"].parse::<i32>().unwrap(),
                    captures["beacon_y"].parse::<i32>().unwrap(),
                ),
            }
        })
        .collect())
}

fn task1(data: &Vec<Sensor>) -> usize {
    let target_y = 2_000_000;
    // let target_y = 10;
    let mut beacon_empty_locations = HashSet::new();

    for sensor in data.iter() {
        let beacon_dist = sensor.calc_distance_to_closest_beacon();

        let y_dist_to_target = (target_y - sensor.location.1).abs() as usize;
        if y_dist_to_target > beacon_dist {
            // Doesn't reach the target y
            continue;
        }

        let x_left_most = sensor.location.0 - (beacon_dist - y_dist_to_target) as i32;
        let x_right_most = sensor.location.0 + (beacon_dist - y_dist_to_target) as i32;
        for x in x_left_most..=x_right_most {
            let pos = Coordinate(x, target_y);
            if pos != sensor.closest_beacon_location {
                beacon_empty_locations.insert(pos);
            }
        }
    }

    beacon_empty_locations.len()
}

fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort();

    let mut out = Vec::new();

    let mut left = 0;
    let mut right = intervals[0].1;

    for i in 1..intervals.len() {
        if (right + 1) < intervals[i].0 {
            out.push((left, right));
            left = intervals[i].0;
            right = intervals[i].1;
        } else {
            right = cmp::max(right, intervals[i].1);
        }
    }

    out.push((left, right));

    out
}

fn task2(data: &Vec<Sensor>) -> usize {
    let lb: i32 = 0;
    // let ub: i32 = 20;
    let ub: i32 = 4_000_000;

    let mut row_intervals = vec![Vec::new(); (ub + 1) as usize];

    // For each sensor compute its row intervals
    for sensor in data.iter() {
        let beacon_dist = sensor.calc_distance_to_closest_beacon();

        let x_left_most = cmp::max(lb, sensor.location.0 - beacon_dist as i32);
        let x_right_most = cmp::min(ub, sensor.location.0 + beacon_dist as i32);
        for x in x_left_most..=x_right_most {
            let remaining_dist = (beacon_dist - (x - sensor.location.0).abs() as usize) as i32;

            let y_interval_left = cmp::max(lb, sensor.location.1 - remaining_dist);
            let y_interval_right = cmp::min(ub, sensor.location.1 + remaining_dist);

            row_intervals[x as usize].push((y_interval_left, y_interval_right));
        }
    }

    // Merge intervals and search for a row where we have an empty spot
    let mut target_x = 0;
    let mut target_y = 0;
    for i in 0..row_intervals.len() {
        let merged_intervals = merge_intervals(row_intervals[i].clone());

        if merged_intervals.len() > 1 {
            // We found it!
            target_x = i;

            assert!(merged_intervals.len() == 2);
            target_y = (merged_intervals[0].1 + 1) as usize;

            break;
        }
    }

    target_x * 4_000_000 + target_y
}

fn main() -> io::Result<()> {
    let data = load_data("input.txt")?;

    // Task 1
    let out_task1 = task1(&data);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(&data);
    println!("Task 2: {}", out_task2);

    Ok(())
}
