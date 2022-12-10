use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum CpuInstruction {
    Addx(i32),
    Noop
}

fn load_program(filename: &str) -> io::Result<Vec<CpuInstruction>> {
    let addx_regex = Regex::new(r"addx (?P<num>-?\d+)").unwrap();
    
    Ok(
        read_lines(filename)?
            .map(|l| {
                let line = l.unwrap();
                if line.starts_with("noop") {
                    CpuInstruction::Noop
                } else {
                    let cps = addx_regex.captures(&line).unwrap();
                    CpuInstruction::Addx(cps["num"].parse::<i32>().unwrap())
                }
            })
            .collect()
    )
}

fn task1(program: &Vec<CpuInstruction>) -> i32 {
    let mut signal_strength = 0;
    let mut cycles = 0;
    let mut register = 1;
    for instruction in program.iter() {
        cycles += 1;
        if cycles == 20 || (cycles > 20 && (cycles - 20) % 40 == 0) {
            signal_strength += cycles * register;
        }
        
        match instruction {
            CpuInstruction::Addx(x) => {
                cycles += 1;
                if cycles == 20 || (cycles > 20 && (cycles - 20) % 40 == 0) {
                    signal_strength += cycles * register;
                }
                register += x;
            },
            CpuInstruction::Noop => {}
        }
    }
    
    signal_strength
}

fn task2(program: &Vec<CpuInstruction>) {
    let mut screen = vec![vec!["."; 40]; 6];
    let mut register: i32 = 1;
    let mut cycle = 0;
    for instruction in program.iter() {
        cycle += 1;
        let row = (cycle-1) / 40;
        let col = (cycle-1) % 40;
        if (register-1) <= col && col <= (register+1) {
            screen[row as usize][col as usize] = "#";
        }

        match instruction {
            CpuInstruction::Addx(x) => {
                cycle += 1;
                let row2 = (cycle-1) / 40;
                let col2 = (cycle-1) % 40;
                if (register-1) <= col2 && col2 <= (register+1) {
                    screen[row2 as usize][col2 as usize] = "#";
                }
                register += x;
            },
            CpuInstruction::Noop => {}
        }
    }

    // Display screen
    for row in 0..screen.len() {
        println!("{:?}", screen[row].join(""));
    }
}

fn main() -> io::Result<()> {
    let program = load_program("input.txt")?;

    // Task 1
    let out_task1 = task1(&program);
    println!("Task 1: {}", out_task1);

    // Task 2
    task2(&program);
    
    Ok(())
}
