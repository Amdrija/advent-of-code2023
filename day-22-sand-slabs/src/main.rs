use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

struct Point {
    x: u64,
    y: u64,
    z: u64,
}

struct Brick {
    start: Point,
    end: Point,
    id: usize,
}

impl Brick {
    fn parse(s: &str, id: usize) -> Self {
        let mut split = s.split("~");

        let start = split
            .next()
            .unwrap()
            .split(",")
            .map(|coord| coord.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let end = split
            .next()
            .unwrap()
            .split(",")
            .map(|coord| coord.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Self {
            start: Point {
                x: start[0],
                y: start[1],
                z: start[2],
            },
            end: Point {
                x: end[0],
                y: end[1],
                z: end[2],
            },
            id,
        }
    }

    fn get_area(&self) -> Vec<Point> {
        if self.start.x != self.end.x {
            return (self.start.x..=self.end.x)
                .map(|x| Point {
                    x,
                    y: self.start.y,
                    z: 1,
                })
                .collect();
        }

        if self.start.y != self.end.y {
            return (self.start.y..=self.end.y)
                .map(|y| Point {
                    x: self.start.x,
                    y,
                    z: 1,
                })
                .collect();
        }

        return vec![Point {
            x: self.start.x,
            y: self.start.y,
            z: self.end.z - self.start.z + 1,
        }];
    }
}

struct Jenga {
    grid: HashMap<(u64, u64), (u64, usize)>,
    supported_by: HashMap<usize, HashSet<usize>>,
    supports: HashMap<usize, Vec<usize>>,
}

impl Jenga {
    fn fall(mut bricks: Vec<Brick>) -> Self {
        let mut grid = HashMap::new();
        let mut supported_by = HashMap::new();
        let mut supports = HashMap::new();

        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));

        for brick in bricks {
            let area = brick.get_area();
            supports.insert(brick.id, Vec::new());

            let mut max = 0;
            let mut brick_supported_by: HashSet<usize> = HashSet::new();
            for point in &area {
                if let Some((height, id)) = grid.get(&(point.x, point.y)) {
                    if max < *height {
                        max = *height;
                        brick_supported_by = HashSet::new();
                        brick_supported_by.insert(*id);
                    } else if max == *height {
                        brick_supported_by.insert(*id);
                    }
                }
            }

            for supported in &brick_supported_by {
                supports.get_mut(&supported).unwrap().push(brick.id);
            }
            supported_by.insert(brick.id, brick_supported_by);

            for point in &area {
                grid.insert((point.x, point.y), (max + point.z, brick.id));
            }
        }

        return Self {
            grid,
            supported_by,
            supports,
        };
    }

    fn dissolvable(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for (brick, supports) in &self.supports {
            if supports
                .iter()
                .all(|supported| self.supported_by[supported].len() > 1)
            {
                result.push(*brick);
            }
        }

        return result;
    }

    fn total_destruction(&mut self) -> usize {
        let mut result = 0;

        for (brick, _) in &self.supports {
            result += self.fall_brick(*brick);
        }

        result
    }

    fn fall_brick(&self, brick: usize) -> usize {
        let mut q = VecDeque::new();
        q.push_back(brick);
        let mut fallen = HashSet::new();
        fallen.insert(&brick);

        while let Some(current) = q.pop_front() {
            for supported in &self.supports[&current] {
                if self.supported_by[supported]
                    .iter()
                    .all(|b| fallen.contains(b))
                {
                    fallen.insert(supported);
                    q.push_back(*supported);
                }
            }
        }

        fallen.len() - 1
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let bricks = input
        .lines()
        .enumerate()
        .map(|(i, line)| Brick::parse(&line, i))
        .collect();

    let mut jenga = Jenga::fall(bricks);

    let dissolvable = jenga.dissolvable();
    println!("{:?}", jenga.supports);
    println!("{}", dissolvable.len());
    println!("{}", jenga.total_destruction());
}
