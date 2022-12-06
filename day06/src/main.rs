use std::collections::HashMap;
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

fn load_data(filename: &str) -> io::Result<Vec<char>> {
    Ok(read_lines(filename)?.next().unwrap()?.chars().collect())
}

fn detect_start(data: &Vec<char>, message_length: usize) -> usize {
    let mut window = HashMap::new();
    for i in 0..message_length {
        window.entry(&data[i]).and_modify(|v| *v += 1).or_insert(1);
    }
    if window.len() == message_length {
        return message_length as usize;
    }

    for i in message_length..data.len() {
        // Remove i - 4
        let value = window.get_mut(&data[i - message_length]).unwrap();
        *value -= 1;
        if *value == 0 {
            window.remove(&data[i - message_length]);
        }

        window.entry(&data[i]).and_modify(|v| *v += 1).or_insert(1);

        if window.len() == message_length {
            return i + 1;
        }
    }

    0
}

fn task1(data: &Vec<char>) -> usize {
    detect_start(data, 4)
}

fn task2(data: &Vec<char>) -> usize {
    detect_start(data, 14)
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
