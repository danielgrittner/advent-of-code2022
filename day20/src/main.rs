extern crate multimap;

use multimap::MultiMap;
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

fn load_data(filename: &str) -> io::Result<Vec<i64>> {
    Ok(read_lines(filename)?
        .map(Result::<String, Error>::unwrap)
        .map(|l| l.parse::<i64>().unwrap())
        .collect())
}

#[derive(Debug)]
struct Node {
    val: i64,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<usize>,
    tail: Option<usize>,
    nodes: Vec<Node>,
    index: MultiMap<i64, usize>,
    length: usize,
}

impl LinkedList {
    fn build(vec: &Vec<i64>) -> Self {
        let mut ll = LinkedList {
            head: Some(0),
            tail: Some(vec.len() - 1),
            nodes: Vec::new(),
            index: MultiMap::new(),
            length: vec.len(),
        };

        let mut prev = None;
        for i in 0..vec.len() {
            let new_node = Node {
                val: vec[i],
                next: None,
                prev: prev,
            };
            ll.nodes.push(new_node);
            ll.index.insert(vec[i], i);
            if prev.is_some() {
                ll.nodes[prev.unwrap()].next = Some(i);
            }
            prev = Some(i);
        }

        ll
    }

    fn move_element_by_element_positions(&mut self, element: i64, index: usize) {
        let cur = self
            .index
            .get_vec(&element)
            .unwrap()
            .iter()
            .filter(|&idx| *idx == index)
            .map(|x| *x)
            .collect::<Vec<usize>>()[0];

        let prev = self.nodes[cur].prev.take();
        let next = self.nodes[cur].next.take();

        // Determine starting node
        let mut running_node = 0;
        if element < 0 {
            if prev.is_none() {
                running_node = self.tail.unwrap();
            } else {
                running_node = prev.unwrap();
            }
        } else {
            if next.is_none() {
                running_node = self.head.unwrap();
            } else {
                running_node = next.unwrap();
            }
        }

        // Move current node out
        if prev.is_none() {
            // current is head
            self.nodes[next.unwrap()].prev = None;
            self.head = next.clone();
        } else if next.is_none() {
            // current is tail
            self.nodes[prev.unwrap()].next = None;
            self.tail = prev.clone();
        } else {
            self.nodes[prev.unwrap()].next = next.clone();
            self.nodes[next.unwrap()].prev = prev.clone();
        }

        if element < 0 {
            // Decreasing order
            for _ in 0..(element.abs() as usize % (self.length - 1)) {
                if self.nodes[running_node].prev.is_none() {
                    running_node = self.tail.unwrap();
                } else {
                    running_node = self.nodes[running_node].prev.unwrap();
                }
            }

            // Insert right from running node
            if self.nodes[running_node].next.is_none() {
                // Running node is tail
                self.nodes[running_node].next = Some(cur);
                self.nodes[cur].prev = Some(running_node);
                self.tail = Some(cur);
            } else {
                let cur_next = self.nodes[running_node].next;
                self.nodes[cur].next = cur_next;
                self.nodes[cur_next.unwrap()].prev = Some(cur);

                self.nodes[running_node].next = Some(cur);
                self.nodes[cur].prev = Some(running_node);
            }
        } else {
            // Increasing order
            for _ in 0..(element.abs() as usize % (self.length - 1)) {
                if self.nodes[running_node].next.is_none() {
                    running_node = self.head.unwrap();
                } else {
                    running_node = self.nodes[running_node].next.unwrap();
                }
            }

            // Insert left from running node
            if self.nodes[running_node].prev.is_none() {
                // Running node is head
                self.nodes[running_node].prev = Some(cur);
                self.nodes[cur].next = Some(running_node);
                self.head = Some(cur);
            } else {
                let cur_prev = self.nodes[running_node].prev;
                self.nodes[cur].prev = cur_prev;
                self.nodes[cur_prev.unwrap()].next = Some(cur);

                self.nodes[running_node].prev = Some(cur);
                self.nodes[cur].next = Some(running_node);
            }
        }
    }

    fn get_coordinate(&self) -> i64 {
        let target = 0;
        let mut cur = Some(*self.index.get(&target).unwrap());

        let mut out = 0;

        for _ in 0..3 {
            for _ in 0..1000 {
                cur = self.nodes[cur.unwrap()].next.clone();
                if cur.is_none() {
                    cur = self.head.clone();
                }
            }

            out += self.nodes[cur.unwrap()].val;
        }

        out
    }

    fn print(&self) {
        let start = 0;
        let mut cur = Some(*self.index.get(&start).unwrap());

        for _ in 0..self.length {
            print!("{}, ", self.nodes[cur.unwrap()].val);
            cur = self.nodes[cur.unwrap()].next.clone();
            if cur.is_none() {
                cur = self.head.clone();
            }
        }
        println!("");
    }
}

fn task1(nums: &Vec<i64>) -> i64 {
    let mut linkedlist = LinkedList::build(&nums);

    for (idx, num) in nums.iter().enumerate() {
        linkedlist.move_element_by_element_positions(*num, idx);
    }

    linkedlist.get_coordinate()
}

const DECRIPTION_KEY: i64 = 811589153;

fn task2(nums: Vec<i64>) -> i64 {
    let new_nums = nums.iter().map(|x| x * DECRIPTION_KEY).collect();
    let mut linkedlist = LinkedList::build(&new_nums);

    for _ in 0..10 {
        for (idx, num) in new_nums.iter().enumerate() {
            linkedlist.move_element_by_element_positions(*num, idx);
        }
    }

    linkedlist.get_coordinate()
}

fn main() -> io::Result<()> {
    let nums = load_data("input.txt")?;

    // Task 1
    let out_task1 = task1(&nums);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(nums);
    println!("Task 2: {}", out_task2);

    Ok(())
}
