use std::cmp::Reverse;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, self};
use priority_queue::PriorityQueue;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_data(filename: &str) -> io::Result<Vec<Vec<char>>> {
    Ok(
        read_lines(filename)?
            .map(|l| l.unwrap().chars().collect())
            .collect()
    )
}

fn find_start_and_target(data: &Vec<Vec<char>>) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut start = Option::None;
    let mut target = Option::None;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'S' {
                start = Some((i, j));
            } else if data[i][j] == 'E' {
                target = Some((i, j));
            }
        }
    }
    
    (start, target)
}

fn task1(data: &Vec<Vec<char>>, start_pos: (usize, usize), target_pos: (usize, usize)) -> usize {
    let end1 = data.len() as i32;
    let end2 = data[0].len() as i32;

    let mut visited = vec![vec![false; data[0].len()]; data.len()];
    let mut min_pq: PriorityQueue<(usize, usize), Reverse<usize>> = PriorityQueue::new();

    visited[start_pos.0][start_pos.1] = true;
    min_pq.push(start_pos, Reverse(0));

    while !min_pq.is_empty() {
        let (pos, path_length) = min_pq.pop().unwrap();
        if pos == target_pos {
            return path_length.0;
        }

        let new_priority = Reverse(path_length.0 + 1);
        let c = data[pos.0][pos.1];

        // Check surrounding
        let upperbound = std::char::from_u32((c as u32) + 1).unwrap();

        for o1 in -1..=1 {
            let i2 = (pos.0 as i32) + o1;     
            if 0 <= i2 && i2 < end1 && !visited[i2 as usize][pos.1] && data[i2 as usize][pos.1] <= upperbound {
                visited[i2 as usize][pos.1] = true;
                min_pq.push((i2 as usize, pos.1), new_priority.clone());
            }
        }

        for o2 in -1..=1 {
            let j2 = (pos.1 as i32) + o2;
            if 0 <= j2 && j2 < end2 && !visited[pos.0][j2 as usize] && data[pos.0][j2 as usize] <= upperbound {
                visited[pos.0][j2 as usize] = true;
                min_pq.push((pos.0, j2 as usize), new_priority.clone());
            }
        }
    }
    
    0
}

fn task2(data: &Vec<Vec<char>>, target_pos: (usize, usize)) -> usize {
    let mut min_path = data.len() * data[0].len();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'a' {
                let res = task1(data, (i, j), target_pos);
                if res > 0 {
                    min_path = std::cmp::min(min_path, res);
                }
            }
        }
    }
    
    min_path
}

fn main() -> io::Result<()> {
    let mut data = load_data("input.txt")?;
    let (start_opt, target_opt) = find_start_and_target(&data);

    let start = start_opt.unwrap();
    let target = target_opt.unwrap();

    data[start.0][start.1] = 'a';
    data[target.0][target.1] = 'z';
    
    // Task 1
    let out_task1 = task1(&data, start, target);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(&data, target);
    println!("Task 2: {}", out_task2);
    
    Ok(())
}
