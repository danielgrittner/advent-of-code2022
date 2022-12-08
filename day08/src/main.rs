use std::cmp;
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

fn load_data(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    Ok(read_lines(filename)?
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| (c as i32) - ('0' as i32))
                .collect()
        })
        .collect())
}

fn task1(data: &Vec<Vec<i32>>) -> usize {
    let end_row = data.len();
    let end_col = data[0].len();

    let mut max_height_down_right = vec![vec![(0, 0); end_col]; end_row]; // (down, right)
    for row in (1..end_row - 1).rev() {
        for col in (1..end_col - 1).rev() {
            // Check row downards
            max_height_down_right[row][col].0 =
                cmp::max(data[row + 1][col], max_height_down_right[row + 1][col].0);

            // Check col rightwards
            max_height_down_right[row][col].1 =
                cmp::max(data[row][col + 1], max_height_down_right[row][col + 1].1);
        }
    }

    let mut out = 2 * (end_row + end_col) - 4;

    let mut max_height_up_left = vec![vec![(0, 0); end_col]; end_row]; // (up, left)
    for row in 1..end_row-1 {
        for col in 1..end_col-1 {
            // Check row upwards
            max_height_up_left[row][col].0 =
                cmp::max(data[row - 1][col], max_height_up_left[row - 1][col].0);

            // Check col leftwards
            max_height_up_left[row][col].1 =
                cmp::max(data[row][col - 1], max_height_up_left[row][col - 1].1);

            let tree_height = data[row][col];
            if tree_height > max_height_up_left[row][col].0
                || tree_height > max_height_up_left[row][col].1
                || tree_height > max_height_down_right[row][col].0
                || tree_height > max_height_down_right[row][col].1
            {
                out += 1;
            }
        }
    }

    out
}

fn task2(data: &Vec<Vec<i32>>) -> usize {
    let mut out = 0;

    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            // Top
            let mut top = i - 1;
            while top > 0 && data[top][j] < data[i][j] {
                top -= 1;
            }

            // Down
            let mut down = i + 1;
            while down + 1 < data.len() && data[down][j] < data[i][j] {
                down += 1;
            }

            // Left
            let mut left = j - 1;
            while left > 0 && data[i][left] < data[i][j] {
                left -= 1;
            }

            // Right
            let mut right = j + 1;
            while right + 1 < data[i].len() && data[i][right] < data[i][j] {
                right += 1;
            }

            let score = (i - top) * (down - i) * (j - left) * (right - j);
            out = cmp::max(out, score);
        }
    }

    out
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
