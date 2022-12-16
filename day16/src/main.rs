use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
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

#[derive(Debug, Clone)]
struct Node {
    edges: HashSet<usize>,
    rate: usize
}

#[derive(Debug, Clone)]
struct Graph {
    adj_list: Vec<Node>,
    start_node: usize
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { adj_list: vec![Node { edges: HashSet::new(), rate: 0 }; n], start_node: 0 }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adj_list[from].edges.insert(to);
    }

    fn set_rate(&mut self, node: usize, rate: usize) {
        self.adj_list[node].rate = rate;
    }
}

fn load_data(filename: &str) -> io::Result<Graph> {
    let lines: Vec<String> = read_lines(filename)?.map(io::Result::<String>::unwrap).collect();
    let mut graph = Graph::new(lines.len());

    let mut node_to_id = HashMap::new();
    for line in lines.iter() {
        let node: String = line.chars().skip(6).take(2).collect();
        let next_id = node_to_id.len();
        if node.starts_with("AA") {
            graph.start_node = next_id;
        }
        node_to_id.insert(node, next_id);
    }
    
    let source_node_regex = Regex::new(r"Valve (?P<source>[A-Z]{2}) has flow rate=(?P<rate>\d+)").unwrap();
    let target_capture_regex = Regex::new(r"[A-Z]{2}").unwrap();

    for line in lines.iter() {
        let split_line: Vec<&str> = line.split(";").collect();

        // Parse source node information
        let source_capture = source_node_regex.captures(split_line[0]).unwrap();
        let source_node = *node_to_id.get(&source_capture["source"]).unwrap();
        let rate = source_capture["rate"].parse::<usize>().unwrap();

        graph.set_rate(source_node, rate);
        
        // Parse edges
        target_capture_regex
            .find_iter(split_line[1])
            .map(|target_node| *node_to_id.get(target_node.as_str()).unwrap())
            .for_each(|target_node_id| graph.add_edge(source_node, target_node_id));
    }

    Ok(graph)
}

fn floyd_warshall(g: &Graph) -> Vec<Vec<usize>> {
    let inf = (1 as usize) << 32;
    let n = g.adj_list.len();

    let mut dp = vec![vec![inf; n]; n];
    for node_id in 0..n {
        dp[node_id][node_id] = 0;
        for target in g.adj_list[node_id].edges.iter() {
            dp[node_id][*target] = 1;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = cmp::min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    dp
}

fn find_best_path(remaining_time: i32, cur_node: usize, rates: &mut Vec<usize>, apsp: &Vec<Vec<usize>>) -> usize {
    if remaining_time <= 0 {
        return 0;
    }

    let mut max_result = 0;
    for node_id in 0..rates.len() {
        if rates[node_id] == 0 || (remaining_time as usize) < (apsp[cur_node][node_id] + 1) {
            continue;
        }

        let gain = (remaining_time as usize - (apsp[cur_node][node_id] + 1)) * rates[node_id];

        let rate = rates[node_id];
        rates[node_id] = 0;
        
        max_result = cmp::max(max_result, 
            gain + find_best_path(remaining_time - (apsp[cur_node][node_id] + 1) as i32, node_id, rates, apsp));
        
        rates[node_id] = rate;
    }

    max_result
}

fn task1(start_node: usize, mut rates: Vec<usize>, apsp: &Vec<Vec<usize>>) -> usize {
    find_best_path(30, start_node, &mut rates, apsp)
}

fn find_best_path_parallel<'a>(remaining_time: i32,
                          me_cur_node: usize,
                          me_time_to_pressure: usize,
                          mut me_rate: usize,
                          elephant_cur_node: usize,
                          elephant_time_to_pressure: usize,
                          mut elephant_rate: usize,
                          rates: &mut Vec<usize>,
                          apsp: &Vec<Vec<usize>>) -> usize {
    let mut gain = 0;
    
    if me_time_to_pressure == 0 {
        gain += me_rate * remaining_time as usize;
        me_rate = 0;
    }
    if elephant_time_to_pressure == 0 {
        gain += elephant_rate * remaining_time as usize;
        elephant_rate = 0;
    }

    if remaining_time <= 0 {
        return gain;
    }

    if me_time_to_pressure > 0 && elephant_time_to_pressure > 0 {
        let step = cmp::min(me_time_to_pressure, elephant_time_to_pressure);
        return find_best_path_parallel(remaining_time - (step as i32),
                                        me_cur_node,
                                        me_time_to_pressure - step,
                                        me_rate,
                                        elephant_cur_node,
                                        elephant_time_to_pressure - step,
                                        elephant_rate,
                                        rates,
                                        apsp);
    }

    let mut max_result = 0;
    
    if me_time_to_pressure == 0 {

        // ME
        
        for node_id in 0..rates.len() {
            if rates[node_id] == 0 || (remaining_time as usize) < (apsp[me_cur_node][node_id] + 1) {
                continue;                
            }

            let rate = rates[node_id];
            rates[node_id] = 0;

            let res = find_best_path_parallel(remaining_time,
                                                    node_id,
                                                    apsp[me_cur_node][node_id] + 1,
                                                    rate,
                                                    elephant_cur_node,
                                                    elephant_time_to_pressure,
                                                    elephant_rate,
                                                    rates,
                                                    apsp);
            max_result = cmp::max(max_result, res);
            
            rates[node_id] = rate;
        }

        if max_result == 0 && elephant_time_to_pressure > 0 {
            max_result = find_best_path_parallel(remaining_time - (elephant_time_to_pressure as i32), me_cur_node, 0, 0, elephant_cur_node, 0, elephant_rate, rates, apsp);
        }
    } else {

        // ELEPHANT
        
        for node_id in 0..rates.len() {
            if rates[node_id] == 0 || (remaining_time as usize) < (apsp[elephant_cur_node][node_id] + 1) {
                continue;                
            }

            let rate = rates[node_id];
            rates[node_id] = 0;

            let res = find_best_path_parallel(remaining_time,
                                                    me_cur_node,
                                                    me_time_to_pressure,
                                                    me_rate,
                                                    node_id,
                                                    apsp[elephant_cur_node][node_id] + 1,
                                                    rate,
                                                    rates,
                                                    apsp);
            max_result = cmp::max(max_result, res);

            rates[node_id] = rate;
        }

        if max_result == 0 && me_time_to_pressure > 0 {
            max_result = find_best_path_parallel(remaining_time - (me_time_to_pressure as i32), me_cur_node, 0, me_rate, elephant_cur_node, 0, 0, rates, apsp);
        }
    }

    gain + max_result
}

fn task2(start_node: usize, mut rates: Vec<usize>, apsp: &Vec<Vec<usize>>) -> usize {
    find_best_path_parallel(26, start_node, 0, 0, start_node, 0, 0, &mut rates, apsp)
}

fn main() -> io::Result<()> {
    let graph = load_data("input.txt")?;
    let rates: Vec<usize> = graph.adj_list.iter().map(|node| node.rate).collect();
    let apsp = floyd_warshall(&graph);

    // Task 1
    let out_task1 = task1(graph.start_node, rates.clone(), &apsp);
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(graph.start_node, rates, &apsp);
    println!("Task 2: {}", out_task2);
    
    Ok(())
}
