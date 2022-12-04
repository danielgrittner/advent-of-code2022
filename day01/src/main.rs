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

fn load_calories(path: &str) -> Result<Vec<i32>, Error> {
    let mut vec = Vec::new();

    let mut cur_calories = 0;
    for line_res in read_lines(path)? {
        let line = &(line_res?);

        if line.is_empty() {
            vec.push(cur_calories);
            cur_calories = 0;
        } else {
            cur_calories += line.parse::<i32>().unwrap();
        }
    }

    vec.push(cur_calories);

    Ok(vec)
}

fn main() -> Result<(), Error> {
    let mut calories = load_calories("input1.txt")?;
    calories.sort();

    // Task 1
    let out_task1 = calories.last().unwrap();
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2: i32 = calories.iter().rev().take(3).sum();
    println!("Task 2: {}", out_task2);

    Ok(())
}
