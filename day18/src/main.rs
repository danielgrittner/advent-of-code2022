use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::HashSet;
use std::collections::VecDeque;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

fn load_data(filename: &str) -> io::Result<Vec<Cube>> {
    let regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+),(?P<z>\d+)").unwrap();
    Ok(read_lines(filename)?
        .map(|l| {
            let line_str = l.unwrap();
            let captures = regex.captures(&line_str).unwrap();

            Cube {
                x: captures["x"].parse::<i32>().unwrap(),
                y: captures["y"].parse::<i32>().unwrap(),
                z: captures["z"].parse::<i32>().unwrap(),
            }
        })
        .collect())
}

fn build_ht(cubes: &Vec<Cube>) -> HashSet<Cube> {
    HashSet::from_iter(cubes.iter().cloned())
}

fn task1(cubes: &Vec<Cube>, ht: &HashSet<Cube>) -> usize {
    let mut total_cube_sides = 6 * cubes.len();

    for cube in cubes.iter() {
        for (i, j, k) in [(-1,0,0), (1,0,0), (0,-1,0), (0,1,0), (0,0,-1), (0,0,1)] {
            let mut query_cube = cube.clone();
            query_cube.x = query_cube.x + i;
            query_cube.y = query_cube.y + j;
            query_cube.z = query_cube.z + k;

            if ht.contains(&query_cube) {
                total_cube_sides -= 1;
            }
        }
    }
    
    total_cube_sides
}

fn is_in_bound(i: i32) -> bool {
    return -1 <= i && i < 21;
}

fn flood_fill(start: Cube, lave_ht: &HashSet<Cube>) -> HashSet<Cube> {
    let mut air_outside = HashSet::new();
    air_outside.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let cur_cube = queue.pop_front().unwrap();

        for (i, j, k) in [(-1,0,0), (1,0,0), (0,-1,0), (0,1,0), (0,0,-1), (0,0,1)] {
            let mut next_cube = cur_cube.clone();
            next_cube.x = next_cube.x + i;
            next_cube.y = next_cube.y + j;
            next_cube.z = next_cube.z + k;

            if lave_ht.contains(&next_cube) || air_outside.contains(&next_cube) || !is_in_bound(next_cube.x) || !is_in_bound(next_cube.y) || !is_in_bound(next_cube.z) {
                continue;
            }

            air_outside.insert(next_cube);
            queue.push_back(next_cube);
        }
    }

    air_outside
}

fn task2(cubes: &mut Vec<Cube>, ht: &mut HashSet<Cube>) -> usize {
    let lower_bound = -1;
    let upper_bound = 21;

    let start = Cube { x: -1, y: -1, z: -1 };
    let outside_air = flood_fill(start, &ht);

    let mut inside_air_cubes = Vec::new();
    let mut inside_air_ht = HashSet::new();
    for x in lower_bound..upper_bound {
        for y in lower_bound..upper_bound {
            for z in lower_bound..upper_bound {
                let cube = Cube { x, y, z };
                if !ht.contains(&cube) && !outside_air.contains(&cube) {
                    inside_air_cubes.push(cube);
                    inside_air_ht.insert(cube);
                }
            }
        }
    }
    
    task1(&cubes, &ht) - task1(&inside_air_cubes, &inside_air_ht)
}

fn main() -> io::Result<()> {
    let mut cubes = load_data("input.txt")?;
    let mut ht = build_ht(&cubes);

    // Task 1
    let out_task1 = task1(&cubes, &ht);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(&mut cubes, &mut ht);
    println!("Task 2: {}", out_task2);

    Ok(())
}