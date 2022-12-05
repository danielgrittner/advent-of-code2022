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

#[derive(Debug, Clone, Copy)]
struct Crate(char);

#[derive(Debug, Clone, Copy)]
struct Instruction {
    num: u32,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Pair(Vec<Vec<Crate>>, Vec<Instruction>);

fn load_data(filename: &str) -> io::Result<Pair> {
    let data: Vec<String> = read_lines(filename)?.map(|l| l.unwrap()).collect();

    // Parse crates
    let num_of_stacks = (data[0].len() + 3) / 4;
    let mut crate_stacks: Vec<Vec<Crate>> = Vec::with_capacity(num_of_stacks);
    for _ in 0..num_of_stacks {
        crate_stacks.push(Vec::new());
    }

    let mut start_idx = 0;
    while !data[start_idx].starts_with(" 1") {
        start_idx += 1;
    }

    for idx in (0..start_idx).rev() {
        let row = &data[idx];
        let mut iter = row.chars();

        let mut stack_idx = 0;
        loop {
            // '['
            if iter.next().is_none() {
                break;
            }

            // Crate or empty
            let crate_id_opt = iter.next();
            if crate_id_opt.is_some() {
                let cid = crate_id_opt.unwrap();
                if cid != ' ' {
                    crate_stacks[stack_idx].push(Crate(cid));
                }
            }

            stack_idx += 1;

            // ']'
            iter.next();
            // Empty space or line break
            iter.next();
        }
    }

    // Skip empty line and go to start of instructions
    start_idx += 2;

    // Parse instructions
    let mut instructions: Vec<Instruction> = Vec::new();
    let instruction_regex =
        Regex::new(r"move (?P<num>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    for idx in start_idx..data.len() {
        let row = &data[idx];
        let caps = instruction_regex.captures(row).unwrap();
        instructions.push(Instruction {
            num: caps["num"].parse::<u32>().unwrap(),
            from: caps["from"].parse::<usize>().unwrap() - 1,
            to: caps["to"].parse::<usize>().unwrap() - 1,
        });
    }

    Ok(Pair(crate_stacks, instructions))
}

fn task1(mut data: Pair) -> String {
    let mut crate_stacks = &mut data.0;
    for instruction in data.1 {
        let mut ops = instruction.num;
        for _ in 0..ops {
            match crate_stacks[instruction.from].pop() {
                None => {}
                Some(x) => crate_stacks[instruction.to].push(x),
            }
        }
    }

    let mut out = String::new();
    for i in 0..crate_stacks.len() {
        match crate_stacks[i].last() {
            Some(c) => out.push(c.0),
            None => {}
        }
    }

    out
}

fn task2(mut data: Pair) -> String {
    let mut crate_stacks = &mut data.0;
    for instruction in data.1 {
        let mut ops = instruction.num;

        let start_idx = crate_stacks[instruction.from].len() - (ops as usize);
        for i in start_idx..crate_stacks[instruction.from].len() {
            let c = crate_stacks[instruction.from][i].clone();
            crate_stacks[instruction.to].push(c);
        }

        // Remove the last ops elements in from stack
        let new_length = crate_stacks[instruction.from]
            .len()
            .saturating_sub(ops as usize);
        crate_stacks[instruction.from].truncate(new_length);
    }

    let mut out = String::new();
    for i in 0..crate_stacks.len() {
        match crate_stacks[i].last() {
            Some(c) => out.push(c.0),
            None => {}
        }
    }

    out
}

fn main() -> io::Result<()> {
    let data = load_data("input.txt")?;

    // Task 1
    let out_task1 = task1(data.clone());
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(data);
    println!("Task 2: {}", out_task2);

    Ok(())
}
