use std::{fmt::Display, fs};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn parse(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Unknown direction: {}", s),
        }
    }

    fn parse_2(ch: &char) -> Direction {
        match ch {
            '3' => Direction::Up,
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            _ => panic!("Unknown direction: {}", ch),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "U",
                Direction::Right => "R",
                Direction::Down => "D",
                Direction::Left => "L",
            }
        )
    }
}

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn get_next(&self, direction: &Direction, length: i64) -> Point {
        match direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y - length,
            },
            Direction::Right => Point {
                x: self.x + length,
                y: self.y,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + length,
            },
            Direction::Left => Point {
                x: self.x - length,
                y: self.y,
            },
        }
    }
}

struct Step {
    direction: Direction,
    length: i64,
}

impl Step {
    fn parse(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<_>>();

        Self {
            direction: Direction::parse(split[0]),
            length: split[1].parse::<i64>().unwrap(),
        }
    }

    fn parse_2(s: &str) -> Self {
        let hex = s.split(" ").last().unwrap();
        let hex = hex.strip_prefix("(#").unwrap().strip_suffix(")").unwrap();

        let last_char = hex.chars().last().unwrap();
        Self {
            direction: Direction::parse_2(&last_char),
            length: i64::from_str_radix(hex.strip_suffix(last_char).unwrap(), 16).unwrap(),
        }
    }
}

fn get_area(steps: &Vec<Step>) -> u64 {
    let mut area = 0;
    let mut p1 = Point { x: 0, y: 0 };

    for step in steps {
        let p2 = p1.get_next(&step.direction, step.length);

        // we have to add length
        // because the area is up to the point,
        // so we have to include the edge as well
        area += p1.x * p2.y - p1.y * p2.x + step.length;

        p1 = p2;
    }

    //accounting for the 0,0 block
    //that is why it's + 1
    return 1 + (area.abs() / 2) as u64;
}

fn parse_steps(s: &str, parse_step: fn(&str) -> Step) -> Vec<Step> {
    return s.lines().map(|line| parse_step(line)).collect();
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let steps = parse_steps(&input, Step::parse);

    println!("{}", get_area(&steps));

    let steps = parse_steps(&input, Step::parse_2);
    println!("{}", get_area(&steps));
}
