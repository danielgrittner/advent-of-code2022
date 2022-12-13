use std::cmp::Ordering;
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

#[derive(Debug, Clone)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        Self::cmp_packets(self, other)
    }

    fn cmp_packets(left: &Packet, right: &Packet) -> Ordering {
        match left {
            Packet::Num(lhs_num) => match right {
                Packet::Num(rhs_num) => lhs_num.cmp(rhs_num),
                Packet::List(_) => {
                    let lhs_packed = Packet::List(vec![Packet::Num(*lhs_num)]);
                    Self::cmp_packets(&lhs_packed, &right)
                }
            },
            Packet::List(lhs_vec) => match right {
                Packet::Num(rhs_num) => {
                    let rhs_packed = Packet::List(vec![Packet::Num(*rhs_num)]);
                    Self::cmp_packets(&left, &rhs_packed)
                }
                Packet::List(rhs_vec) => {
                    let mut i = 0;
                    while i < lhs_vec.len() && i < rhs_vec.len() {
                        match Self::cmp_packets(&lhs_vec[i], &rhs_vec[i]) {
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => i += 1,
                        }
                    }

                    if i == lhs_vec.len() && i == rhs_vec.len() {
                        Ordering::Equal
                    } else if i >= lhs_vec.len() && i < rhs_vec.len() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn new(left: Packet, right: Packet) -> Self {
        PacketPair { left, right }
    }

    fn is_in_right_order(&self) -> bool {
        self.left.cmp(&self.right) != Ordering::Greater
    }
}

fn parse_packet(packet_chars: Vec<char>) -> Packet {
    let mut packet_stack: Vec<Option<Packet>> = Vec::new();

    let mut i = 0;
    while i < packet_chars.len() {
        if packet_chars[i] == '[' {
            packet_stack.push(None);
            i += 1;
        } else if packet_chars[i] == ']' {
            let mut packet_list = Vec::new();
            while !packet_stack.is_empty() && packet_stack.last().unwrap().is_some() {
                packet_list.push(packet_stack.pop().unwrap().unwrap());
            }
            packet_list.reverse();
            if !packet_stack.is_empty() {
                // Remove None (representing '[')
                packet_stack.pop();
            }
            packet_stack.push(Some(Packet::List(packet_list)));
            i += 1;
            if i < packet_chars.len() && packet_chars[i] == ',' {
                i += 1;
            }
        } else {
            // We have a number
            let mut num_str = String::new();
            while i < packet_chars.len() && packet_chars[i].is_digit(10) {
                num_str.push(packet_chars[i]);
                i += 1;
            }
            let num = num_str.parse::<u32>().unwrap();
            packet_stack.push(Some(Packet::Num(num)));
            if packet_chars[i] != ']' {
                i += 1;
            }
        }
    }

    assert!(packet_stack.len() == 1);
    packet_stack.pop().unwrap().unwrap()
}

fn load_data(filename: &str) -> io::Result<Vec<PacketPair>> {
    let mut out = Vec::new();

    let lines: Vec<String> = read_lines(filename)?
        .map(Result::<String, Error>::unwrap)
        .collect();

    let mut idx = 0;
    while idx < lines.len() {
        let first_packet_str: Vec<char> = lines[idx].chars().collect();
        let first_packet = parse_packet(first_packet_str);

        let snd_packet_str: Vec<char> = lines[idx + 1].chars().collect();
        let snd_packet = parse_packet(snd_packet_str);

        out.push(PacketPair::new(first_packet, snd_packet));

        idx += 3;
    }

    Ok(out)
}

fn task1(data: &Vec<PacketPair>) -> usize {
    data.iter()
        .enumerate()
        .map(|(i, p)| if p.is_in_right_order() { i + 1 } else { 0 })
        .sum()
}

fn task2(data: Vec<PacketPair>) -> usize {
    let mut data2 = Vec::new();
    for it in data.into_iter() {
        data2.push((it.left, false));
        data2.push((it.right, false));
    }

    // Insert dividers
    data2.push((Packet::List(vec![Packet::List(vec![Packet::Num(2)])]), true));
    data2.push((Packet::List(vec![Packet::List(vec![Packet::Num(6)])]), true));

    // Sort packets
    data2.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    // Find the dividers again
    let mut out: usize = 1;
    for i in 0..data2.len() {
        if data2[i].1 {
            out *= i + 1;
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
    let out_task2 = task2(data);
    println!("Task 2: {}", out_task2);

    Ok(())
}
