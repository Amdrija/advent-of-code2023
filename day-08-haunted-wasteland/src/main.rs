use std::{collections::HashMap, fs, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
struct Instructions {
    list: Vec<Instruction>,
    current: usize,
}

impl Instructions {
    fn new(instructions: Vec<Instruction>) -> Instructions {
        return Instructions {
            list: instructions,
            current: 0,
        };
    }
}

impl Iterator for Instructions {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.list.len() {
            self.current = 0;
        }

        let instruction = self.list[self.current];
        self.current += 1;

        return Some(instruction);
    }
}

#[derive(Debug)]
struct ParseInstructionsError;

impl FromStr for Instructions {
    type Err = ParseInstructionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .chars()
            .map(|ch| {
                if ch == 'R' {
                    return Instruction::Right;
                }

                return Instruction::Left;
            })
            .collect();

        return Ok(Instructions::new(instructions));
    }
}

struct Destination {
    left: String,
    right: String,
}

#[derive(Debug)]
struct ParseDestinationError;

impl FromStr for Destination {
    type Err = ParseDestinationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(", ");

        let left = split
            .next()
            .ok_or(ParseDestinationError)?
            .strip_prefix("(")
            .ok_or(ParseDestinationError)?;
        let right = split
            .next()
            .ok_or(ParseDestinationError)?
            .strip_suffix(")")
            .ok_or(ParseDestinationError)?;

        return Ok(Destination {
            left: left.to_string(),
            right: right.to_string(),
        });
    }
}

struct Map {
    map: HashMap<String, Destination>,
}

#[derive(Debug)]
enum ParseMapError {
    MapError,
    DestinationError(ParseDestinationError),
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, Destination> = HashMap::new();

        for line in s.lines() {
            let mut split = line.split(" = ");
            let from = split.next().ok_or(ParseMapError::MapError)?;
            let to = split
                .next()
                .ok_or(ParseMapError::MapError)?
                .parse::<Destination>()
                .map_err(|e| ParseMapError::DestinationError(e))?;

            map.insert(from.to_string(), to);
        }

        return Ok(Map { map });
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut split = input.split("\n\n");

    let mut instructions = split
        .next()
        .unwrap()
        .parse::<Instructions>()
        .unwrap()
        .into_iter();
    let map = split.next().unwrap().parse::<Map>().unwrap();

    let mut current = "AAA";
    let mut count = 0;
    while current != "ZZZ" {
        let instruction = instructions.next().unwrap();

        match instruction {
            Instruction::Left => current = &map.map[current].left,
            Instruction::Right => current = &map.map[current].right,
        }

        count += 1;
    }

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Instruction, Instructions, Map};

    #[test]
    fn test_parsing_instructions() {
        let instructions = "LLR".parse::<Instructions>();

        assert!(instructions.is_ok());

        assert_eq!(
            instructions.unwrap().list,
            vec![Instruction::Left, Instruction::Left, Instruction::Right]
        );
    }

    #[test]
    fn test_instruction_iterator() {
        let instructions = Instructions::new(vec![
            Instruction::Left,
            Instruction::Left,
            Instruction::Right,
        ])
        .into_iter()
        .zip(vec![0, 1, 2, 3, 4, 5, 6])
        .collect::<Vec<(Instruction, usize)>>();

        assert_eq!(
            instructions,
            vec![
                (Instruction::Left, 0),
                (Instruction::Left, 1),
                (Instruction::Right, 2),
                (Instruction::Left, 3),
                (Instruction::Left, 4),
                (Instruction::Right, 5),
                (Instruction::Left, 6)
            ]
        );
    }

    const MAP: &str = "AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    #[test]
    fn test_parse_map() {
        let map = MAP.parse::<Map>();

        assert!(map.is_ok());

        let map = map.unwrap().map;
        assert_eq!(map.keys().count(), 3);

        assert!(map.contains_key("AAA"));
        assert_eq!(map["AAA"].left, "BBB");
        assert_eq!(map["AAA"].right, "BBB");

        assert!(map.contains_key("BBB"));
        assert_eq!(map["BBB"].left, "AAA");
        assert_eq!(map["BBB"].right, "ZZZ");

        assert!(map.contains_key("ZZZ"));
        assert_eq!(map["ZZZ"].left, "ZZZ");
        assert_eq!(map["ZZZ"].right, "ZZZ");
    }
}
