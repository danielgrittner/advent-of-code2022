use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_score(c: char) -> u32 {
    c as u32 - (if 'a' <= c && c <= 'z' { ('a' as u32) - 1 } else { ('A' as u32) - 27 })
}

fn intersect(sets: Vec<&HashSet<char>>) -> Vec<char> {
    sets[0]
        .iter()
        .map(|c| {
            for i in 1..sets.len() {
                if !sets[i].contains(c) {
                    return None;
                }
            }
            Some(*c)
        })
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect()
}

fn task1(path: &str) -> io::Result<u32> {
    Ok(
        read_lines(path)?
        .map(|line| -> u32 {
            let rucksack_content = line.unwrap();

            let rucksack_left = &rucksack_content[..rucksack_content.len()/2];
            let left_map = rucksack_left
                .chars()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                });

            let rucksack_right = &rucksack_content[rucksack_content.len()/2..];
            let right_map = rucksack_right
                .chars()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                });

            intersect(vec![&left_map, &right_map])
                .iter()
                .map(|c| get_score(*c))
                .sum()
        })
        .sum()
    )
}

fn task2(path: &str) -> io::Result<u32> {
    Ok(
        read_lines(path)?
        .map(|l| {
            l.unwrap()
                .chars()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                })
        })
        .tuples::<(_,_,_)>()
        .map(|group| -> u32 {
            intersect(vec![&group.0, &group.1, &group.2])
                .iter()
                .map(|c| get_score(*c))
                .sum()
        })
        .sum()
    )
}

fn main() -> io::Result<()> {
    // Task 1
    let out_task1: u32 = task1("input.txt")?;
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2: u32 = task2("input.txt")?;
    println!("Task 2: {}", out_task2);

    Ok(())
}
