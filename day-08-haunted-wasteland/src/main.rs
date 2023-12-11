use std::{collections::HashMap, fs, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone)]
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

impl Map {
    fn traverse(
        &self,
        instructions: Instructions,
        starting_node: &str,
        is_end: fn(&str) -> bool,
    ) -> usize {
        let mut current = starting_node;
        let mut count = 0;

        for instruction in instructions {
            if is_end(current) {
                break;
            }

            match instruction {
                Instruction::Left => current = &self.map[current].left,
                Instruction::Right => current = &self.map[current].right,
            }

            count += 1;
        }

        return count;
    }

    #[allow(dead_code)]
    fn traverse_parallel(&self, instructions: Instructions) -> usize {
        let mut current_nodes = self
            .map
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect::<Vec<&String>>();
        let mut count = 0;

        for instruction in instructions {
            if current_nodes.iter().all(|node| node.ends_with("Z")) {
                break;
            }

            current_nodes = match instruction {
                Instruction::Left => current_nodes
                    .iter()
                    .map(|node| &self.map[*node].left)
                    .collect(),
                Instruction::Right => current_nodes
                    .iter()
                    .map(|node| &self.map[*node].right)
                    .collect(),
            };

            count += 1;
        }

        return count;
    }
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

    let instructions = split.next().unwrap().parse::<Instructions>().unwrap();
    let map = split.next().unwrap().parse::<Map>().unwrap();

    // println!("{}", map.traverse_parallel(instructions.clone()));

    let current_nodes = map
        .map
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<&String>>();

    println!("{:?}", current_nodes);

    let counts = current_nodes
        .iter()
        .map(|start| map.traverse(instructions.clone(), &start, |node| node.ends_with("Z")))
        .collect::<Vec<usize>>();

    println!("{:?}", counts);
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_map_traverse() {
        let map = MAP.parse::<Map>();
        assert!(map.is_ok());

        let instructions = "LR".parse::<Instructions>();
        assert!(instructions.is_ok());

        assert_eq!(
            map.unwrap()
                .traverse(instructions.unwrap(), &"AAA", |node| node == "ZZZ"),
            2
        );
    }

    const MAP_PARALLEL: &str = "11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    #[test]
    fn test_map_traverse_parallel() {
        let map: Result<Map, crate::ParseMapError> = MAP_PARALLEL.parse::<Map>();
        assert!(map.is_ok());

        let instructions = "LR".parse::<Instructions>();
        assert!(instructions.is_ok());

        assert_eq!(map.unwrap().traverse_parallel(instructions.unwrap()), 6);
    }
}
