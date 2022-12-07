use regex::Regex;
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

fn load_data(filename: &str) -> io::Result<Vec<String>> {
    Ok(read_lines(filename)?.map(|l| l.unwrap()).collect())
}

/**
 * Instructions:
 *
 * cd <dir> ==> add 0 to stack
 * cd .. ==> pop from stack and add to previous
 * ls ==> update last element in stack
 */

fn collect_sorted_dir_sizes(data: &Vec<String>) -> Vec<u64> {
    let mut dir_sizes = Vec::new();

    let filesize_regex = Regex::new(r"(?P<filesize>\d+) *").unwrap();

    let mut path = Vec::new();
    let mut line_idx = 0;
    while line_idx < data.len() {
        let line = &data[line_idx];
        if line.starts_with("$ cd ..") {
            // Move one directory level up
            let child_size = path.pop().unwrap();
            dir_sizes.push(child_size);
            *path.last_mut().unwrap() += child_size;
            line_idx += 1;
        } else if line.starts_with("$ ls") {
            // Update directory size
            line_idx += 1;
            // Iterate as long as we have ls output
            while line_idx < data.len() && !data[line_idx].starts_with("$") {
                if !data[line_idx].starts_with("dir") {
                    // We have a line with filesize!
                    let caps = filesize_regex.captures(&data[line_idx]).unwrap();
                    *path.last_mut().unwrap() += caps["filesize"].parse::<u64>().unwrap();
                }
                line_idx += 1;
            }
        } else {
            // Must be "$ cd <dir>"
            path.push(0);
            line_idx += 1;
        }
    }

    // Clean up stack
    while !path.is_empty() {
        let child_size = path.pop().unwrap();
        dir_sizes.push(child_size);
        if !path.is_empty() {
            *path.last_mut().unwrap() += child_size;
        }
    }

    // Sort for efficient querying
    dir_sizes.sort();
    dir_sizes
}

fn task1(sorted_dir_sizes: &Vec<u64>) -> u64 {
    let limit = 100_000;
    let i = sorted_dir_sizes.partition_point(|&x| x < limit);
    sorted_dir_sizes[..i].iter().sum()
}

fn task2(sorted_dir_sizes: &Vec<u64>) -> u64 {
    let limit =
        30000000 - (70000000 as i64 - i64::try_from(*sorted_dir_sizes.last().unwrap()).unwrap());
    if limit <= 0 {
        return 0;
    }
    let i = sorted_dir_sizes.partition_point(|&x| x < limit as u64);
    sorted_dir_sizes[i]
}

fn main() -> io::Result<()> {
    let data = load_data("input.txt")?;
    let sorted_dir_sizes = collect_sorted_dir_sizes(&data);

    // Task 1
    let out_task1 = task1(&sorted_dir_sizes);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(&sorted_dir_sizes);
    println!("Task 2: {}", out_task2);

    Ok(())
}
