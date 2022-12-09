use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
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
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            _ => Err(Error::new(ErrorKind::Other, "Parsing error")),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    n: u32,
}

fn load_data(filename: &str) -> io::Result<Vec<Instruction>> {
    let instruction_regex = Regex::new(r"(?P<direction>[RLUD]) (?P<num>\d+)").unwrap();

    Ok(read_lines(filename)?
        .map(|l| {
            let s = l.unwrap();
            let caps = instruction_regex.captures(&s).unwrap();

            Instruction {
                direction: caps["direction"].parse::<Direction>().unwrap(),
                n: caps["num"].parse::<u32>().unwrap(),
            }
        })
        .collect())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new() -> Self {
        Position { row: 0, col: 0 }
    }

    fn is_touching(&self, other: &Position) -> bool {
        (self.row - 1 <= other.row)
            && (other.row <= self.row + 1)
            && (self.col - 1 <= other.col)
            && (other.col <= self.col + 1)
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn new(n: usize) -> Self {
        Rope {
            knots: vec![Position::new(); n],
        }
    }

    fn follow_knot(&mut self, knot_idx: usize) {
        if knot_idx >= self.knots.len()
            || self.knots[knot_idx - 1].is_touching(&self.knots[knot_idx])
        {
            return;
        }

        if self.knots[knot_idx - 1].row == self.knots[knot_idx].row {
            if self.knots[knot_idx - 1].col < self.knots[knot_idx].col {
                // Go left
                self.knots[knot_idx].col -= 1;
            } else {
                // Go right
                self.knots[knot_idx].col += 1;
            }
        } else if self.knots[knot_idx - 1].col == self.knots[knot_idx].col {
            if self.knots[knot_idx - 1].row < self.knots[knot_idx].row {
                // Go up
                self.knots[knot_idx].row -= 1;
            } else {
                // Go down
                self.knots[knot_idx].row += 1;
            }
        } else {
            if self.knots[knot_idx - 1].col < self.knots[knot_idx].col
                && self.knots[knot_idx - 1].row < self.knots[knot_idx].row
            {
                // Go left-up
                self.knots[knot_idx].row -= 1;
                self.knots[knot_idx].col -= 1;
            } else if self.knots[knot_idx - 1].col > self.knots[knot_idx].col
                && self.knots[knot_idx - 1].row < self.knots[knot_idx].row
            {
                // Go right-up
                self.knots[knot_idx].row -= 1;
                self.knots[knot_idx].col += 1;
            } else if self.knots[knot_idx - 1].col > self.knots[knot_idx].col
                && self.knots[knot_idx - 1].row > self.knots[knot_idx].row
            {
                // Go right-down
                self.knots[knot_idx].row += 1;
                self.knots[knot_idx].col += 1;
            } else {
                // Go left-down
                self.knots[knot_idx].row += 1;
                self.knots[knot_idx].col -= 1;
            }
        }

        self.follow_knot(knot_idx + 1);
    }

    fn apply_direction(&mut self, d: Direction) {
        match d {
            Direction::Left => self.knots[0].col -= 1,
            Direction::Up => self.knots[0].row -= 1,
            Direction::Right => self.knots[0].col += 1,
            Direction::Down => self.knots[0].row += 1,
        }
        self.follow_knot(1);
    }
}

fn simulate(instructions: &Vec<Instruction>, n: usize) -> usize {
    let mut tail_visited_pos: HashSet<Position> = HashSet::new();

    let mut rope = Rope::new(n);
    instructions.iter().for_each(|instruction| {
        for _ in 0..instruction.n {
            rope.apply_direction(instruction.direction);
            tail_visited_pos.insert(*rope.knots.last().unwrap());
        }
    });

    tail_visited_pos.len()
}

fn task1(instructions: &Vec<Instruction>) -> usize {
    simulate(instructions, 2)
}

fn task2(instructions: &Vec<Instruction>) -> usize {
    simulate(instructions, 10)
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
