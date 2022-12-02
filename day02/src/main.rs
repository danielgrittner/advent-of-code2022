use std::str::FromStr;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum GameResult {
    Victory,
    Draw,
    Defeat
}

impl GameResult {
    fn score(&self) -> i32 {
        match self {
            GameResult::Victory => 6,
            GameResult::Draw => 3,
            GameResult::Defeat => 0
        }
    }
}

impl FromStr for GameResult {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<GameResult, Self::Err> {
        match s {
            "X" => Ok(GameResult::Defeat),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Victory),
            _ => Err(Error::new(ErrorKind::Other, "Parsing error"))
        }
    }
}

#[derive(PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }

    fn play(&self, other: &Shape) -> GameResult {
        match self {
            Shape::Rock => {
                match other {
                    Shape::Rock => GameResult::Draw,
                    Shape::Paper => GameResult::Defeat,
                    Shape::Scissors => GameResult::Victory
                }
            },
            Shape::Paper => {
                match other {
                    Shape::Rock => GameResult::Victory,
                    Shape::Paper => GameResult::Draw,
                    Shape::Scissors => GameResult::Defeat
                }
            },
            Shape::Scissors => {
                match other {
                    Shape::Rock => GameResult::Defeat,
                    Shape::Paper => GameResult::Victory,
                    Shape::Scissors => GameResult::Draw
                }
            }
        }
    }
}

// https://www.reddit.com/r/rust/comments/2vqama/parse_string_as_enum_value/
/// Usage: string.parse::<Shape>()
impl FromStr for Shape {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Shape, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(Error::new(ErrorKind::Other, "Parsing error"))
        }
    }
}

impl GameResult {
    fn get_other_shape(&self, other_shape: &Shape) -> Shape {
        match self {
            GameResult::Defeat => {
                match other_shape {
                    Shape::Paper => Shape::Rock,
                    Shape::Rock => Shape::Scissors,
                    Shape::Scissors => Shape::Paper
                }
            },
            GameResult::Draw => {
                match other_shape {
                    Shape::Paper => Shape::Paper,
                    Shape::Rock => Shape::Rock,
                    Shape::Scissors => Shape::Scissors
                }
            },
            GameResult::Victory => {
                match other_shape {
                    Shape::Paper => Shape::Scissors,
                    Shape::Rock => Shape::Paper,
                    Shape::Scissors => Shape::Rock
                }
            }
        }
    }
}

struct ShapePair(Shape, Shape);

fn load_data(path: &str) -> Result<Vec<ShapePair>, Error> {
    let mut vec = Vec::new();

    for line in read_lines(path)? {
        let line_str = line?;
        let pair: Vec<&str> = line_str.split(" ").collect();
        vec.push(
            ShapePair(pair[0].parse::<Shape>()?, pair[1].parse::<Shape>()?)
        );
    }

    Ok(vec)
}

struct ShapeGameResultPair(Shape, GameResult);

fn load_data2(path: &str) -> Result<Vec<ShapeGameResultPair>, Error> {
    let mut vec = Vec::new();

    for line in read_lines(path)? {
        let line_str = line?;
        let pair: Vec<&str> = line_str.split(" ").collect();
        vec.push(
            ShapeGameResultPair(pair[0].parse::<Shape>()?, pair[1].parse::<GameResult>()?)
        );
    }

    Ok(vec)
}

fn main() -> Result<(), Error> {
    // Task 1:
    let data = load_data("input.txt")?;
    let out_task1: i32 = data.iter().map(|p| p.1.score() + p.1.play(&p.0).score()).sum();
    println!("Task 1: {}", out_task1);
    
    // Task 2:
    let data2 = load_data2("input.txt")?;
    let out_task2: i32 = data2.iter().map(|p| p.1.get_other_shape(&p.0).score() + p.1.score()).sum();
    println!("Task 2: {}",  out_task2);
    
    Ok(())
}
