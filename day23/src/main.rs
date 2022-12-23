use core::num;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, Error};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn load_data(filename: &str) -> io::Result<HashSet<Coordinate>> {
    let field: Vec<Vec<char>> = read_lines(filename)?
        .map(|l| {
            let l_str = l.unwrap();
            l_str.chars().collect()
        })
        .collect();

    let mut elves_pos = HashSet::new();
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == '#' {
                elves_pos.insert(Coordinate {
                    x: row as i32,
                    y: col as i32,
                });
            }
        }
    }

    Ok(elves_pos)
}

fn simulate(mut elves_pos: HashSet<Coordinate>, n: usize) -> i32 {
    let mut dir_queue = VecDeque::new();
    dir_queue.push_back(Direction::North);
    dir_queue.push_back(Direction::South);
    dir_queue.push_back(Direction::West);
    dir_queue.push_back(Direction::East);

    for _ in 0..n {
        let mut planned_move: HashMap<Coordinate, Option<Direction>> = HashMap::new();
        let mut next_pos_count: HashMap<Coordinate, usize> = HashMap::new();

        // First half: determine new position
        for pos in elves_pos.iter() {
            let mut next_pos = *pos;
            let mut target_dir = None;

            // Empty around?
            let mut is_surrounded = false;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let key = Coordinate {
                        x: next_pos.x + i,
                        y: next_pos.y + j,
                    };
                    if elves_pos.contains(&key) {
                        is_surrounded = true;
                        break;
                    }
                }
            }

            if !is_surrounded {
                // Don't move because no other elve is around!
                planned_move.insert(*pos, target_dir);
                continue;
            }

            for next_dir in dir_queue.iter() {
                match next_dir {
                    Direction::North => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x - 1,
                                y: next_pos.y + i,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.x -= 1;
                            target_dir = Some(Direction::North);
                            break;
                        }
                    }
                    Direction::South => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x + 1,
                                y: next_pos.y + i,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.x += 1;
                            target_dir = Some(Direction::South);
                            break;
                        }
                    }
                    Direction::West => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x + i,
                                y: next_pos.y - 1,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.y -= 1;
                            target_dir = Some(Direction::West);
                            break;
                        }
                    }
                    Direction::East => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x + i,
                                y: next_pos.y + 1,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.y += 1;
                            target_dir = Some(Direction::East);
                            break;
                        }
                    }
                }
            }

            if target_dir.is_some() {
                next_pos_count
                    .entry(next_pos.clone())
                    .and_modify(|cnt| *cnt += 1)
                    .or_insert(1);
            }
            planned_move.insert(*pos, target_dir);
        }

        // Second half: Move if possible
        let mut new_elves_pos = HashSet::new();

        for pos in elves_pos.iter() {
            let planned_dir = *planned_move.get(pos).unwrap();

            if planned_dir.is_none() {
                // No move was possible
                new_elves_pos.insert(*pos);
                continue;
            }

            let mut planned_pos = pos.clone();
            match planned_dir.unwrap() {
                Direction::North => planned_pos.x -= 1,
                Direction::South => planned_pos.x += 1,
                Direction::East => planned_pos.y += 1,
                Direction::West => planned_pos.y -= 1,
            }

            if *next_pos_count.get(&planned_pos).unwrap() > 1 {
                // Multiple elves want to move to the same position! Don't move!
                new_elves_pos.insert(*pos);
            } else {
                // Only the current elve wants to move to the planned pos! Move!
                new_elves_pos.insert(planned_pos);
            }
        }

        let front_dir = dir_queue.pop_front().unwrap();
        dir_queue.push_back(front_dir);
        elves_pos = new_elves_pos;
    }

    // Get empty spots
    let mut min_y = 10_000_000;
    let mut max_y = -10_000_000;
    let mut min_x = 10_000_000;
    let mut max_x = -10_000_000;
    for coord in elves_pos.iter() {
        min_y = cmp::min(min_y, coord.y);
        max_y = cmp::max(max_y, coord.y);
        min_x = cmp::min(min_x, coord.x);
        max_x = cmp::max(max_x, coord.x);
    }

    let mut out = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let key = Coordinate { x, y };
            if !elves_pos.contains(&key) {
                out += 1;
            }
        }
    }

    out
}

fn simulate2(mut elves_pos: HashSet<Coordinate>) -> i32 {
    let mut dir_queue = VecDeque::new();
    dir_queue.push_back(Direction::North);
    dir_queue.push_back(Direction::South);
    dir_queue.push_back(Direction::West);
    dir_queue.push_back(Direction::East);

    let mut loop_cnt = 0;

    loop {
        loop_cnt += 1;

        let mut planned_move: HashMap<Coordinate, Option<Direction>> = HashMap::new();
        let mut next_pos_count: HashMap<Coordinate, usize> = HashMap::new();

        // First half: determine new position
        for pos in elves_pos.iter() {
            let mut next_pos = *pos;
            let mut target_dir = None;

            // Empty around?
            let mut is_surrounded = false;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let key = Coordinate {
                        x: next_pos.x + i,
                        y: next_pos.y + j,
                    };
                    if elves_pos.contains(&key) {
                        is_surrounded = true;
                        break;
                    }
                }
            }

            if !is_surrounded {
                // Don't move because no other elve is around!
                planned_move.insert(*pos, target_dir);
                continue;
            }

            for next_dir in dir_queue.iter() {
                match next_dir {
                    Direction::North => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x - 1,
                                y: next_pos.y + i,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.x -= 1;
                            target_dir = Some(Direction::North);
                            break;
                        }
                    }
                    Direction::South => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x + 1,
                                y: next_pos.y + i,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.x += 1;
                            target_dir = Some(Direction::South);
                            break;
                        }
                    }
                    Direction::West => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x + i,
                                y: next_pos.y - 1,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.y -= 1;
                            target_dir = Some(Direction::West);
                            break;
                        }
                    }
                    Direction::East => {
                        let mut found_elve = false;
                        for i in -1..=1 {
                            let key = Coordinate {
                                x: next_pos.x + i,
                                y: next_pos.y + 1,
                            };
                            if elves_pos.contains(&key) {
                                found_elve = true;
                                break;
                            }
                        }
                        if !found_elve {
                            next_pos.y += 1;
                            target_dir = Some(Direction::East);
                            break;
                        }
                    }
                }
            }

            if target_dir.is_some() {
                next_pos_count
                    .entry(next_pos.clone())
                    .and_modify(|cnt| *cnt += 1)
                    .or_insert(1);
            }
            planned_move.insert(*pos, target_dir);
        }

        // Second half: Move if possible
        let mut new_elves_pos = HashSet::new();

        let mut num_elves_not_moved = 0;

        for pos in elves_pos.iter() {
            let planned_dir = *planned_move.get(pos).unwrap();

            if planned_dir.is_none() {
                // No move was possible
                new_elves_pos.insert(*pos);
                num_elves_not_moved += 1;
                continue;
            }

            let mut planned_pos = pos.clone();
            match planned_dir.unwrap() {
                Direction::North => planned_pos.x -= 1,
                Direction::South => planned_pos.x += 1,
                Direction::East => planned_pos.y += 1,
                Direction::West => planned_pos.y -= 1,
            }

            if *next_pos_count.get(&planned_pos).unwrap() > 1 {
                // Multiple elves want to move to the same position! Don't move!
                new_elves_pos.insert(*pos);
                num_elves_not_moved += 1;
            } else {
                // Only the current elve wants to move to the planned pos! Move!
                new_elves_pos.insert(planned_pos);
            }
        }

        if num_elves_not_moved == elves_pos.len() {
            break;
        }

        let front_dir = dir_queue.pop_front().unwrap();
        dir_queue.push_back(front_dir);
        elves_pos = new_elves_pos;
    }

    loop_cnt
}

fn main() -> io::Result<()> {
    let elves_pos = load_data("input.txt")?;

    // Task 1
    let out_task1 = simulate(elves_pos.clone(), 10);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = simulate2(elves_pos);
    println!("Task 2: {}", out_task2);

    Ok(())
}
