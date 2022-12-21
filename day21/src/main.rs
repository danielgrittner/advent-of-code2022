use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Operation {
    Plus,
    Minus,
    Mul,
    Div,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "+" => Ok(Operation::Plus),
            "-" => Ok(Operation::Minus),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            _ => Err(Error::new(ErrorKind::Other, "Parsing error")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum JobType {
    Num(i64),
    Ops(String, Operation, String),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Job {
    id: String,
    job: JobType,
}

fn load_data(filename: &str) -> io::Result<Vec<Job>> {
    let regex = Regex::new(
        r"(?P<id>[a-z]+): ((?P<num>\d+)|(?P<lhs>[a-z]+) (?P<op>[+\-*/]) (?P<rhs>[a-z]+))",
    )
    .unwrap();

    Ok(read_lines(filename)?
        .map(|l| {
            let line_str = l.unwrap();

            let captures = regex.captures(&line_str).unwrap();

            let id = &captures["id"];
            if !captures.name("num").is_none() {
                Job {
                    id: id.to_string(),
                    job: JobType::Num(captures["num"].parse::<i64>().unwrap()),
                }
            } else {
                Job {
                    id: id.to_string(),
                    job: JobType::Ops(
                        captures["lhs"].to_string(),
                        captures["op"].parse::<Operation>().unwrap(),
                        captures["rhs"].to_string(),
                    ),
                }
            }
        })
        .collect())
}

fn build_ht(data: &Vec<Job>) -> HashMap<String, (JobType, bool)> {
    let mut ht = HashMap::new();
    data.iter().for_each(|j| {
        ht.insert(j.id.clone(), (j.job.clone(), false));
    });
    ht
}

fn evaluate(current: &str, ht: &mut HashMap<String, (JobType, bool)>) -> i64 {
    let (job_type, _) = ht.get(current).unwrap().clone();
    match job_type {
        JobType::Num(x) => x,
        JobType::Ops(lhs, op, rhs) => {
            let lhs_val = evaluate(&lhs, ht);
            let rhs_val = evaluate(&rhs, ht);

            let result = match op {
                Operation::Plus => lhs_val + rhs_val,
                Operation::Minus => lhs_val - rhs_val,
                Operation::Mul => lhs_val * rhs_val,
                Operation::Div => lhs_val / rhs_val,
            };

            ht.insert(current.to_string(), (JobType::Num(result), false));

            result
        }
    }
}

fn annotate_contains_humn(current: &str, ht: &mut HashMap<String, (JobType, bool)>) -> bool {
    let res = match ht.get(current).unwrap().clone().0 {
        JobType::Num(x) => current.starts_with("humn"),
        JobType::Ops(lhs, _, rhs) => {
            annotate_contains_humn(&lhs, ht) || annotate_contains_humn(&rhs, ht)
        }
    };
    if res {
        ht.entry(current.to_string()).and_modify(|e| e.1 = true);
    }
    res
}

fn task1(data: &Vec<Job>) -> i64 {
    let mut ht = build_ht(&data);
    evaluate("root", &mut ht)
}

fn solver(current: &str, ht: &mut HashMap<String, (JobType, bool)>, target: i64) -> i64 {
    if current.starts_with("humn") {
        return target;
    }

    let (job_type, _) = ht.get(current).unwrap().clone();
    if let JobType::Ops(lhs, op, rhs) = job_type {
        if ht.get(&lhs).unwrap().1 {
            // lhs contains humn!
            let rhs_result = evaluate(&rhs, ht);
            let new_target = match op {
                Operation::Plus => target - rhs_result,
                Operation::Minus => target + rhs_result,
                Operation::Mul => target / rhs_result,
                Operation::Div => target * rhs_result,
            };

            solver(&lhs, ht, new_target)
        } else {
            // rhs contains humn!
            let lhs_result = evaluate(&lhs, ht);
            let new_target = match op {
                Operation::Plus => target - lhs_result,
                Operation::Minus => (target - lhs_result) * -1,
                Operation::Mul => target / lhs_result,
                Operation::Div => (lhs_result / target),
            };

            solver(&rhs, ht, new_target)
        }
    } else {
        // Should not happen!
        assert!(false);
        -1
    }
}

fn task2(data: &Vec<Job>) -> i64 {
    let mut ht = build_ht(&data);

    ht.insert("humn".to_string(), (JobType::Num(0), true));
    annotate_contains_humn("root", &mut ht);

    if let JobType::Ops(lhs, _, rhs) = ht.get("root").unwrap().clone().0 {
        if ht.get(&lhs).unwrap().1 {
            // Lhs contains humn
            let target = evaluate(&rhs, &mut ht);
            solver(&lhs, &mut ht, target)
        } else {
            // Rhs contains humn
            let target = evaluate(&lhs, &mut ht);
            solver(&rhs, &mut ht, target)
        }
    } else {
        -1
    }
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
