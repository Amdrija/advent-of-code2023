use std::{collections::VecDeque, fs};

fn hash(s: &str) -> u8 {
    let mut hash = 0;

    for ch in s.chars() {
        hash += ch as u32;
        hash *= 17;
        hash %= 256;
    }

    return hash as u8;
}

struct Lens {
    label: String,
    focal_length: u8,
}

struct Box {
    lenses: VecDeque<Lens>,
}

impl Box {
    fn new() -> Self {
        return Self {
            lenses: VecDeque::new(),
        };
    }

    fn add_lens(&mut self, lens: Lens) {
        for l in &mut self.lenses {
            if l.label == lens.label {
                l.focal_length = lens.focal_length;
                return;
            }
        }

        self.lenses.push_back(lens);
    }

    fn remove_lens(&mut self, label: &str) {
        for (i, l) in &mut self.lenses.iter().enumerate() {
            if l.label == label {
                self.lenses.remove(i);
                return;
            }
        }
    }
}

struct Boxes {
    boxes: Vec<Box>,
}

enum Operation {
    Put(Lens),
    Remove(String),
}

impl Operation {
    fn new(s: &str) -> Operation {
        let split = s.split("=").collect::<Vec<_>>();

        if split.len() == 1 {
            return Operation::Remove(split[0].strip_suffix("-").unwrap().to_string());
        }

        return Operation::Put(Lens {
            label: split[0].to_string(),
            focal_length: split[1].parse().unwrap(),
        });
    }
}

impl Boxes {
    fn new() -> Self {
        let mut result = Boxes { boxes: Vec::new() };

        for _ in 0..256 {
            result.boxes.push(Box::new());
        }

        return result;
    }

    fn operation(&mut self, operation: Operation) {
        match operation {
            Operation::Put(lens) => {
                let label_hash = hash(&lens.label);

                self.boxes[label_hash as usize].add_lens(lens);
            }
            Operation::Remove(label) => {
                let label_hash = hash(&label);

                self.boxes[label_hash as usize].remove_lens(&label);
            }
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut power = 0;
        for (i, bx) in self.boxes.iter().enumerate() {
            for (j, lens) in bx.lenses.iter().enumerate() {
                power += (i + 1) * (j + 1) * lens.focal_length as usize;
            }
        }

        return power as u64;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!(
        "{}",
        input.split(",").map(|part| hash(part) as u64).sum::<u64>()
    );

    let mut boxes = Boxes::new();

    for op in input.split(",") {
        let operation = Operation::new(op);
        boxes.operation(operation);
    }

    println!("{}", boxes.focusing_power());
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{hash, Boxes, Operation};

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
    }

    #[test]
    fn test_focusing_power() {
        let input = fs::read_to_string("test.txt");

        assert!(input.is_ok());

        let mut boxes = Boxes::new();

        for op in input.unwrap().split(",") {
            let operation = Operation::new(op);
            boxes.operation(operation);
        }

        assert_eq!(boxes.focusing_power(), 145);
    }
}
