use std::cmp;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type Ore = u32;
type Clay = u32;
type Obsidian = u32;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore_robot_costs: Ore,
    clay_robot_costs: Ore,
    obsidian_robot_costs: (Ore, Clay),
    geode_robot_costs: (Ore, Obsidian),
}

fn load_data(filename: &str) -> io::Result<Vec<Blueprint>> {
    let blueprint_regex = Regex::new(r"Blueprint \d+: Each ore robot costs (?P<ore_robot_costs>\d+) ore. Each clay robot costs (?P<clay_robot_costs>\d+) ore. Each obsidian robot costs (?P<obsidian_robot_costs1>\d+) ore and (?P<obsidian_robot_costs2>\d+) clay. Each geode robot costs (?P<geode_robot_costs1>\d+) ore and (?P<geode_robot_costs2>\d+) obsidian.").unwrap();

    Ok(read_lines(filename)?
        .map(|l| {
            let line_str = l.unwrap();

            let captures = blueprint_regex.captures(&line_str).unwrap();

            Blueprint {
                ore_robot_costs: captures["ore_robot_costs"].parse::<Ore>().unwrap(),
                clay_robot_costs: captures["clay_robot_costs"].parse::<Ore>().unwrap(),
                obsidian_robot_costs: (
                    captures["obsidian_robot_costs1"].parse::<Ore>().unwrap(),
                    captures["obsidian_robot_costs2"].parse::<Clay>().unwrap(),
                ),
                geode_robot_costs: (
                    captures["geode_robot_costs1"].parse::<Ore>().unwrap(),
                    captures["geode_robot_costs2"].parse::<Obsidian>().unwrap(),
                ),
            }
        })
        .collect())
}

#[derive(Debug, Clone, Copy)]
struct Production {
    t: usize,
    time_limit: usize,
    blueprint: Blueprint,
    num_ore_robots: u32,
    num_clay_robots: u32,
    num_obsidian_robots: u32,
    num_geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    max_ore_costs: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Robot {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot
}

impl Production {
    fn new(blueprint: Blueprint, time_limit: usize) -> Self {
        let max_ore_costs = cmp::max(blueprint.clay_robot_costs, cmp::max(blueprint.obsidian_robot_costs.0, blueprint.geode_robot_costs.0));
        Production {
            t: 1,
            time_limit,
            blueprint,
            num_ore_robots: 1,
            num_clay_robots: 0,
            num_obsidian_robots: 0,
            num_geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            max_ore_costs
        }
    }

    fn reached_timelimit(&self) -> bool { self.t == self.time_limit }

    fn produce(&mut self) {
        self.ore += self.num_ore_robots;
        self.clay += self.num_clay_robots;
        self.obsidian += self.num_obsidian_robots;
        self.geode += self.num_geode_robots;
    }

    fn can_build_ore_robot(&self) -> bool { self.ore >= self.blueprint.ore_robot_costs }

    fn can_build_clay_robot(&self) -> bool { self.ore >= self.blueprint.clay_robot_costs }

    fn can_build_obsidian_robot(&self) -> bool { self.ore >= self.blueprint.obsidian_robot_costs.0 && self.clay >= self.blueprint.obsidian_robot_costs.1 }

    fn can_build_geode_robot(&self) -> bool { self.ore >= self.blueprint.geode_robot_costs.0 && self.obsidian >= self.blueprint.geode_robot_costs.1 }

    fn build_and_produce(&mut self, requested_robot: Robot) {
        match requested_robot {
            Robot::OreRobot => {
                self.ore -= self.blueprint.ore_robot_costs;
            },
            Robot::ClayRobot => {
                self.ore -= self.blueprint.clay_robot_costs;
            },
            Robot::ObsidianRobot => {
                self.ore -= self.blueprint.obsidian_robot_costs.0;
                self.clay -= self.blueprint.obsidian_robot_costs.1;
            },
            Robot::GeodeRobot => {
                self.ore -= self.blueprint.geode_robot_costs.0;
                self.obsidian -= self.blueprint.geode_robot_costs.1;
            }
        }

        self.produce();
        self.t += 1;

        match requested_robot {
            Robot::OreRobot => self.num_ore_robots += 1,
            Robot::ClayRobot => self.num_clay_robots += 1,
            Robot::ObsidianRobot => self.num_obsidian_robots += 1,
            Robot::GeodeRobot => self.num_geode_robots += 1
        }
    }
}

fn search(production: Production) -> u32 {
    let mut max_geode = 0;
    
    let mut queue = VecDeque::new();
    queue.push_back(production);

    while !queue.is_empty() {
        let mut cur_production = queue.pop_front().unwrap();
        
        if cur_production.reached_timelimit() {
            cur_production.produce();
            max_geode = cmp::max(max_geode, cur_production.geode);
            continue;
        }

        for next_robot in [Robot::GeodeRobot, Robot::ObsidianRobot, Robot::ClayRobot, Robot::OreRobot] {
            if next_robot == Robot::OreRobot && cur_production.num_ore_robots == cur_production.max_ore_costs {
                continue;
            }
            if next_robot == Robot::ClayRobot && cur_production.num_clay_robots == cur_production.blueprint.obsidian_robot_costs.1 {
                continue;
            }
            if next_robot == Robot::ObsidianRobot && cur_production.num_obsidian_robots == cur_production.blueprint.geode_robot_costs.1 {
                continue;
            }

            if next_robot == Robot::ObsidianRobot && cur_production.num_clay_robots == 0 {
                continue;
            }
            if next_robot == Robot::GeodeRobot && cur_production.num_obsidian_robots == 0 {
                continue;
            }

            let mut new_production = cur_production.clone();

            while (next_robot == Robot::GeodeRobot && !new_production.can_build_geode_robot()) ||
                (next_robot == Robot::ObsidianRobot && !new_production.can_build_obsidian_robot()) ||
                (next_robot == Robot::ClayRobot && !new_production.can_build_clay_robot()) ||
                (next_robot == Robot::OreRobot && !new_production.can_build_ore_robot()) {
                new_production.produce();
                if new_production.reached_timelimit() {
                    break;
                }
                new_production.t += 1;
            }

            if new_production.reached_timelimit() {
                max_geode = cmp::max(max_geode, new_production.geode);
                continue;
            }
            
            new_production.build_and_produce(next_robot);
            max_geode = cmp::max(max_geode, new_production.geode);
            queue.push_back(new_production);
        }
    }

    max_geode
}

fn task1(blueprints: Vec<Blueprint>) -> u32 {
    let mut out = 0;

    for i in 0..blueprints.len() {
        out += (i as u32 + 1) * search(Production::new(blueprints[i], 24));
    }

    out
}

fn task2(blueprints: Vec<Blueprint>) -> u32 {
    let mut out = 1;

    for i in 0..3 {
        out *= search(Production::new(blueprints[i], 32));
    }

    out
}

fn main() -> io::Result<()> {
    let blueprints = load_data("input.txt")?;

    // Task 1
    let out_task1 = task1(blueprints.clone());
    println!("Task 1: {}", out_task1);

    // Task 2
    let out_task2 = task2(blueprints);
    println!("Task 2: {}", out_task2);

    Ok(())
}
