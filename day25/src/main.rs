/*

SNAFU (Special Numeral-Analogue Fuel Units)

starting on the right, normal numbers have a ones place, a tens place, a hundreds place, and so on
==> SNAFU 5 instead of 10 ==> 4 digit number = x3 * 5^3 + x2 * 5^2 + x1 * 5 + x0 * 5^0

Digits => 2, 1, 0, minus (-1), double-minus (-2)

*/

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

fn load_data(filename: &str) -> io::Result<Vec<String>> {
    Ok(read_lines(filename)?
        .map(Result::<String, Error>::unwrap)
        .collect())
}

fn snafu_to_decimal(num: &String) -> i64 {
    let mut out = 0;
    let mut base = 1;

    for c in num.chars().rev() {
        out += base
            * match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                _ => -2,
            };
        base *= 5;
    }

    out
}

fn decimal_to_snafu(mut num: i64) -> String {
    let snafu_digits = ['=', '-', '0', '1', '2'];

    let mut out_rev = String::new();
    while num > 0 {
        num += 2;
        let res = num % 5;
        out_rev.push(snafu_digits[res as usize]);
        num /= 5;
    }

    out_rev.chars().rev().collect()
}

fn main() -> io::Result<()> {
    let snafu_nums = load_data("input.txt")?;

    // Task 1
    let out_task1 = decimal_to_snafu(snafu_nums.iter().map(snafu_to_decimal).sum::<i64>());
    println!("Task 1: {}", out_task1);

    Ok(())
}
