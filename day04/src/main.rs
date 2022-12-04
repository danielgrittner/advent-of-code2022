use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Range(i32, i32);

struct RangePair(Range, Range);

fn load_data(path: &str) -> io::Result<Vec<RangePair>> {
    Ok(read_lines(path)?
        .map(|l| {
            let line_str = l.unwrap();
            let left_and_right: Vec<&str> = line_str.split(",").collect();

            let left_range_split: Vec<&str> = left_and_right[0].split("-").collect();
            let left_range = Range(
                left_range_split[0].parse::<i32>().unwrap(),
                left_range_split[1].parse::<i32>().unwrap(),
            );

            let right_range_split: Vec<&str> = left_and_right[1].split("-").collect();
            let right_range = Range(
                right_range_split[0].parse::<i32>().unwrap(),
                right_range_split[1].parse::<i32>().unwrap(),
            );

            RangePair(left_range, right_range)
        })
        .collect())
}

fn task1(data: &Vec<RangePair>) -> u32 {
    data.iter()
        .map(|range_pair| {
            let r1 = &range_pair.0;
            let r2 = &range_pair.1;

            if (r1.0 <= r2.0 && r2.1 <= r1.1) || (r2.0 <= r1.0 && r1.1 <= r2.1) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn task2(data: &Vec<RangePair>) -> u32 {
    data.iter()
        .map(|range_pair| {
            let r1 = &range_pair.0;
            let r2 = &range_pair.1;

            if r1.1 < r2.0 || r2.1 < r1.0 {
                0
            } else {
                1
            }
        })
        .sum()
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
