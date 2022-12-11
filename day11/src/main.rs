use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Square,
    Add(i64),
    Mul(i64)
}

impl Operation {
    fn apply(&self, x: i64) -> i64 {
        match self {
            Self::Square => x * x,
            Self::Add(y) => x + y,
            Self::Mul(y) => x * y
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op: Operation,
    divisibility_test: i64,
    true_div_test_target: usize,
    false_div_test_target: usize,
    inspected_items: usize
}

struct InspectionResult {
    item: i64,
    monkey_target: usize
}

impl Monkey {
    fn new(items: VecDeque<i64>,
           op: Operation,
           divisibility_test: i64,
           true_div_test_target: usize,
           false_div_test_target: usize) -> Self {
        Monkey {
            items,
            op,
            divisibility_test,
            true_div_test_target,
            false_div_test_target,
            inspected_items: 0
        }
    }

    fn has_items_left(&self) -> bool { self.items.is_empty() }

    fn inspect_item(&mut self, regularizer: i64, use_mod: bool) -> InspectionResult {
        self.inspected_items += 1;
        let mut item = self.items.pop_front().unwrap();
        item = self.op.apply(item);
        if use_mod {
            item %= regularizer;
        } else {
            item /= regularizer;
        }
        InspectionResult { 
            item,
            monkey_target: if item % self.divisibility_test == 0 {
                self.true_div_test_target
            } else {
                self.false_div_test_target
            }
        }
    }

    fn add_item(&mut self, item: i64) {
        self.items.push_back(item);
    }

    fn get_inspected_items(&self) -> usize { self.inspected_items }
}

fn load_data(filename: &str) -> io::Result<Vec<Monkey>> {
    let lines: Vec<String> = read_lines(filename)?
        .map(|l| l.unwrap())
        .collect();

    let item_regex = Regex::new(r"\d+").unwrap();
    let operation_regex = Regex::new(r"  Operation: new = old (?P<op>[+*]) (?P<num>(-?\d+|old))").unwrap();
    let div_by_regex = Regex::new(r"  Test: divisible by (?P<num>\d+)").unwrap();
    let if_true_regex = Regex::new(r"    If true: throw to monkey (?P<num>\d+)").unwrap();
    let if_false_regex = Regex::new(r"    If false: throw to monkey (?P<num>\d+)").unwrap();
    
    let mut monkeys = Vec::new();

    let mut start = 0;
    while start < lines.len() {
        // Items
        let items: VecDeque<i64> = item_regex
            .find_iter(&lines[start+1])
            .map(|item| item.as_str().parse::<i64>().unwrap())
            .collect();
        
        // Operation
        let caps_op = operation_regex.captures(&lines[start+2]).unwrap();
        let op = match &caps_op["op"] {
            "+" => Operation::Add(caps_op["num"].parse::<i64>().unwrap()),
            _ => {
                match &caps_op["num"] {
                    "old" => Operation::Square,
                    _ => Operation::Mul(caps_op["num"].parse::<i64>().unwrap())
                }
            }
        };

        // Test
        let div_by_cap = div_by_regex.captures(&lines[start+3]).unwrap();
        let div_by = div_by_cap["num"].parse::<i64>().unwrap();

        let if_true_cap = if_true_regex.captures(&lines[start+4]).unwrap();
        let if_true = if_true_cap["num"].parse::<usize>().unwrap();

        let if_false_cap = if_false_regex.captures(&lines[start+5]).unwrap();
        let if_false = if_false_cap["num"].parse::<usize>().unwrap();

        monkeys.push(Monkey::new(items, op, div_by, if_true, if_false));

        start += 7;
    }
    
    Ok(monkeys)
}

fn simulate_monkeys(mut monkeys: Vec<Monkey>, rounds: usize, regularizer: i64, use_mod: bool) -> Vec<Monkey> {
    for _ in 0..rounds {
        for monkey_id in 0..monkeys.len() {
            while !monkeys[monkey_id].has_items_left() {
                let res = monkeys[monkey_id].inspect_item(regularizer, use_mod);
                monkeys[res.monkey_target].add_item(res.item);
            }
        }
    }

    monkeys
}

fn run(mut monkeys: Vec<Monkey>, rounds: usize, regularizer: i64, use_mod: bool) -> usize {
    monkeys = simulate_monkeys(monkeys, rounds, regularizer, use_mod);

    let mut max_inspected = 0;
    let mut snd_max_inspected = 0;
    for monkey in monkeys.iter() {
        if monkey.get_inspected_items() > max_inspected {
            snd_max_inspected = max_inspected;
            max_inspected = monkey.get_inspected_items();
        } else if monkey.get_inspected_items() > snd_max_inspected {
            snd_max_inspected = monkey.get_inspected_items();
        }
    }

    max_inspected * snd_max_inspected
}

fn task1(monkeys: Vec<Monkey>) -> usize {
    run(monkeys, 20, 3, false)
}

fn task2(monkeys: Vec<Monkey>) -> usize {
    let regularizer: i64 = monkeys.iter().map(|m| m.divisibility_test).product();
    run(monkeys, 10_000, regularizer, true)
}

fn main() -> io::Result<()> {
    let monkeys = load_data("input.txt")?;
    
    // Task 1
    let out_task1 = task1(monkeys.clone());
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(monkeys);
    println!("Task 2: {}", out_task2);

    Ok(())
}