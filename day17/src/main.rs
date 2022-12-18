use core::num;
use std::cmp;
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy)]
enum HorizontalDirection {
    Left,
    Right,
}

impl HorizontalDirection {
    fn from(c: char) -> HorizontalDirection {
        match c {
            '<' => HorizontalDirection::Left,
            _ => HorizontalDirection::Right,
        }
    }
}

fn load_data(filename: &str) -> io::Result<Vec<HorizontalDirection>> {
    let lines: Vec<String> = read_lines(filename)?
        .map(Result::<String, Error>::unwrap)
        .take(1)
        .collect();
    Ok(lines[0].chars().map(HorizontalDirection::from).collect())
}

/// Coordinate represents lowest, left-most coordinate
#[derive(Debug, Clone, Copy)]
enum Rock {
    HorizontalLine(usize, usize),
    Star(usize, usize),
    MirroredL(usize, usize),
    VerticalLine(usize, usize),
    Square(usize, usize),
}

impl Rock {
    fn next(&self, init_height: usize) -> Rock {
        match self {
            Rock::HorizontalLine(_, _) => Rock::Star(init_height, 3),
            Rock::Star(_, _) => Rock::MirroredL(init_height, 2),
            Rock::MirroredL(_, _) => Rock::VerticalLine(init_height, 2),
            Rock::VerticalLine(_, _) => Rock::Square(init_height, 2),
            Rock::Square(_, _) => Rock::HorizontalLine(init_height, 2),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Elements {
    Air,
    Rock,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Row(u8);

impl Row {
    fn new() -> Self {
        Row(0)
    }
    fn is_air(&self, idx: usize) -> bool {
        (self.0 & ((1 as u8) << idx)) == 0
    }
    fn is_rock(&self, idx: usize) -> bool {
        (self.0 & ((1 as u8) << idx)) > 0
    }
    fn set_rock(&mut self, idx: usize) {
        self.0 |= (1 as u8) << idx;
    }
}

const TUNNEL_WIDTH: usize = 7;

#[derive(Debug, Clone)]
struct Tunnel {
    t: Vec<Row>,
    jet_instructions: Vec<HorizontalDirection>,
    next_jet_instruction: usize,
    falling_rock: Rock,
    tower_height: usize,
}

impl Tunnel {
    fn new(jet_instructions: Vec<HorizontalDirection>) -> Self {
        Tunnel {
            t: vec![Row::new(); 4],
            jet_instructions,
            next_jet_instruction: 0,
            falling_rock: Rock::HorizontalLine(3, 2),
            tower_height: 0,
        }
    }

    fn can_rock_move_down(&mut self) -> bool {
        match &mut self.falling_rock {
            Rock::HorizontalLine(row, col) => {
                if *row == 0 {
                    // Reached the bottom
                    return false;
                }

                // Check if each position can move one down
                for i in 0..4 {
                    if self.t[(*row) - 1].is_rock(*col + i) {
                        return false;
                    }
                }

                true
            }
            Rock::Star(row, col) => {
                if *row == 0 {
                    // Reached the bottom
                    return false;
                }

                if self.t[(*row) - 1].is_rock(*col)
                    || self.t[*row].is_rock(*col - 1)
                    || self.t[*row].is_rock(*col + 1)
                {
                    return false;
                }

                true
            }
            Rock::MirroredL(row, col) => {
                if *row == 0 {
                    // Reached the bottom
                    return false;
                }

                // Check if each position can move one down
                for i in 0..3 {
                    if self.t[(*row) - 1].is_rock(*col + i) {
                        return false;
                    }
                }

                true
            }
            Rock::VerticalLine(row, col) => {
                // Check if we can move one down
                if *row == 0 || self.t[(*row) - 1].is_rock(*col) {
                    return false;
                }

                true
            }
            Rock::Square(row, col) => {
                if *row == 0 {
                    // Reached the bottom
                    return false;
                }

                // Check if each position can move one down
                for i in 0..2 {
                    if self.t[(*row) - 1].is_rock(*col + i) {
                        return false;
                    }
                }

                true
            }
        }
    }

    fn move_rock_down(&mut self) {
        match &mut self.falling_rock {
            Rock::HorizontalLine(row, col) => *row -= 1,
            Rock::Star(row, col) => *row -= 1,
            Rock::MirroredL(row, col) => *row -= 1,
            Rock::VerticalLine(row, col) => *row -= 1,
            Rock::Square(row, col) => *row -= 1,
        }
    }

    fn move_rock_horizontally_if_possible(&mut self, direction: HorizontalDirection) {
        match &mut self.falling_rock {
            Rock::HorizontalLine(row, col) => match direction {
                HorizontalDirection::Left => {
                    if *col > 0 && self.t[*row].is_air(*col - 1) {
                        *col -= 1;
                    }
                }
                HorizontalDirection::Right => {
                    if (*col + 4) < TUNNEL_WIDTH && self.t[*row].is_air(*col + 4) {
                        *col += 1;
                    }
                }
            },
            Rock::Star(row, col) => {
                match direction {
                    HorizontalDirection::Left => {
                        if *col < 2 {
                            return;
                        }

                        // Check if left side is free
                        if self.t[*row].is_rock(*col - 1)
                            || self.t[*row + 1].is_rock(*col - 2)
                            || self.t[*row + 2].is_rock(*col - 1)
                        {
                            return;
                        }

                        // Left side is free
                        *col -= 1;
                    }
                    HorizontalDirection::Right => {
                        if *col + 2 >= TUNNEL_WIDTH {
                            return;
                        }

                        // Check if right side is free
                        if self.t[*row].is_rock(*col + 1)
                            || self.t[*row + 1].is_rock(*col + 2)
                            || self.t[*row + 2].is_rock(*col + 1)
                        {
                            return;
                        }

                        // Right side is free
                        *col += 1;
                    }
                }
            }
            Rock::MirroredL(row, col) => {
                match direction {
                    HorizontalDirection::Left => {
                        if *col == 0 {
                            return;
                        }

                        // Check if left side is free
                        if self.t[*row].is_rock(*col - 1) {
                            return;
                        }
                        for i in 1..3 {
                            if self.t[*row + i].is_rock(*col + 1) {
                                return;
                            }
                        }

                        // Left side is free
                        *col -= 1;
                    }
                    HorizontalDirection::Right => {
                        if (*col + 3) >= TUNNEL_WIDTH {
                            return;
                        }

                        // Check if right side is free
                        for i in 0..3 {
                            if self.t[*row + i].is_rock(*col + 3) {
                                return;
                            }
                        }

                        // Right side is free
                        *col += 1;
                    }
                }
            }
            Rock::VerticalLine(row, col) => {
                match direction {
                    HorizontalDirection::Left => {
                        if *col == 0 {
                            return;
                        }

                        // Check if the left side is free
                        for i in 0..4 {
                            if self.t[*row + i].is_rock(*col - 1) {
                                return;
                            }
                        }

                        // Left side is free
                        *col -= 1;
                    }
                    HorizontalDirection::Right => {
                        if (*col + 1) >= TUNNEL_WIDTH {
                            return;
                        }

                        // Check if the right side is free
                        for i in 0..4 {
                            if self.t[*row + i].is_rock(*col + 1) {
                                return;
                            }
                        }

                        // Left side is free
                        *col += 1;
                    }
                }
            }
            Rock::Square(row, col) => {
                match direction {
                    HorizontalDirection::Left => {
                        if *col == 0 {
                            return;
                        }

                        // Check if the left side is free
                        for i in 0..2 {
                            if self.t[*row + i].is_rock(*col - 1) {
                                return;
                            }
                        }

                        // Left side is free
                        *col -= 1;
                    }
                    HorizontalDirection::Right => {
                        if (*col + 2) >= TUNNEL_WIDTH {
                            return;
                        }

                        // Check if the left side is free
                        for i in 0..2 {
                            if self.t[*row + i].is_rock(*col + 2) {
                                return;
                            }
                        }

                        // Left side is free
                        *col += 1;
                    }
                }
            }
        }
    }

    fn stop_rock(&mut self) {
        match &self.falling_rock {
            Rock::HorizontalLine(row, col) => {
                for i in 0..4 {
                    self.t[*row].set_rock(*col + i);
                }

                self.tower_height = cmp::max(self.tower_height, *row + 1);
            }
            Rock::Star(row, col) => {
                for i in 0..3 {
                    self.t[*row + i].set_rock(*col);
                }
                self.t[*row + 1].set_rock(*col - 1);
                self.t[*row + 1].set_rock(*col + 1);

                self.tower_height = cmp::max(self.tower_height, *row + 2 + 1);
            }
            Rock::MirroredL(row, col) => {
                for i in 0..3 {
                    self.t[*row].set_rock(*col + i);
                }
                self.t[*row + 1].set_rock(*col + 2);
                self.t[*row + 2].set_rock(*col + 2);

                self.tower_height = cmp::max(self.tower_height, *row + 2 + 1);
            }
            Rock::VerticalLine(row, col) => {
                for i in 0..4 {
                    self.t[*row + i].set_rock(*col);
                }

                self.tower_height = cmp::max(self.tower_height, *row + 3 + 1);
            }
            Rock::Square(row, col) => {
                for i in 0..2 {
                    for j in 0..2 {
                        self.t[*row + i].set_rock(*col + j);
                    }
                }

                self.tower_height = cmp::max(self.tower_height, *row + 1 + 1);
            }
        }

        self.falling_rock = self.falling_rock.next(self.tower_height + 3);
        while self.t.len() < (self.tower_height + 7) {
            self.t.push(Row::new());
        }
    }

    fn simulate_falling_rock(&mut self) {
        loop {
            self.move_rock_horizontally_if_possible(
                self.jet_instructions[self.next_jet_instruction],
            );
            self.next_jet_instruction =
                (self.next_jet_instruction + 1) % self.jet_instructions.len();
            if self.can_rock_move_down() {
                self.move_rock_down();
            } else {
                self.stop_rock();
                break;
            }
        }
    }

    fn print(&self) {
        for row in self.t.iter().rev() {
            for i in 0..TUNNEL_WIDTH {
                if row.is_rock(i) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn simulate(jet_instructions: Vec<HorizontalDirection>, n: usize) -> usize {
    let mut tunnel = Tunnel::new(jet_instructions);

    let mut ht = HashMap::new();

    let mut out = 0;

    let mut num_rocks = 0;
    while num_rocks < n {
        tunnel.simulate_falling_rock();
        num_rocks += 1;

        if tunnel.tower_height >= 40 {
            // Start cycle detection

            // Get pattern
            let pattern: Vec<Row> = tunnel.t[tunnel.tower_height - 40..tunnel.tower_height]
                .iter()
                .map(|x| x.clone())
                .collect();
            let prev_pattern_opt = ht.get(&pattern);
            if prev_pattern_opt.is_none() {
                ht.insert(pattern, (num_rocks, tunnel.tower_height));
                continue;
            }

            // We found an existing pattern!
            let (prev_num_rocks, prev_tower_height) = prev_pattern_opt.unwrap();
            let remaining_rocks = n - num_rocks;

            let cycle_cost = num_rocks - prev_num_rocks;
            let total_cycles = remaining_rocks / cycle_cost;

            if total_cycles == 0 {
                continue;
            }

            let add_on_tower_height = (tunnel.tower_height - prev_tower_height) * total_cycles;

            num_rocks += cycle_cost * total_cycles;
            out += add_on_tower_height;
        }
    }

    out + tunnel.tower_height
}

fn task1(jet_instructions: Vec<HorizontalDirection>) -> usize {
    simulate(jet_instructions, 2022)
}

fn task2(jet_instructions: Vec<HorizontalDirection>) -> usize {
    simulate(jet_instructions, 1_000_000_000_000)
}

fn main() -> io::Result<()> {
    let jet_instructions = load_data("input.txt")?;

    // Task 1
    let out_task1 = task1(jet_instructions.clone());
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(jet_instructions);
    println!("Task 2: {}", out_task2);

    Ok(())
}
