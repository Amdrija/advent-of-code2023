use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_next(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Direction::North => (i - 1, j),
            Direction::East => (i, j + 1),
            Direction::South => (i + 1, j),
            Direction::West => (i, j - 1),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

impl Tile {
    fn parse(s: &str) -> Tile {
        match s {
            "#" => Tile::Forest,
            "." => Tile::Path,
            "^" => Tile::Slope(Direction::North),
            ">" => Tile::Slope(Direction::East),
            "v" => Tile::Slope(Direction::South),
            "<" => Tile::Slope(Direction::West),
            _ => panic!("Unknown tile: {}", s),
        }
    }

    fn get_next(&self, i: usize, j: usize, part2: bool) -> Vec<(usize, usize)> {
        match self {
            Tile::Forest => Vec::new(),
            Tile::Path => vec![
                Direction::North.get_next(i, j),
                Direction::South.get_next(i, j),
                Direction::West.get_next(i, j),
                Direction::East.get_next(i, j),
            ],
            Tile::Slope(direction) => {
                if part2 {
                    vec![
                        direction.get_next(i, j),
                        direction.opposite().get_next(i, j),
                    ]
                } else {
                    vec![direction.get_next(i, j)]
                }
            }
        }
    }
}

struct Hike {
    tiles: Vec<Vec<Tile>>,
    visited: Vec<Vec<bool>>,
}

impl Hike {
    fn parse(s: &str) -> Self {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| Tile::parse(&ch.to_string()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            visited: vec![vec![false; tiles[0].len()]; tiles.len()],
            tiles: tiles,
        }
    }

    fn backtrack(&mut self, i: usize, j: usize, part2: bool) -> (usize, bool) {
        self.visited[i][j] = true;
        if i == self.tiles.len() - 1 {
            self.visited[i][j] = false;
            return (0, false);
        }

        let mut max = 0;
        for (next_i, next_j) in self.tiles[i][j].get_next(i, j, part2) {
            if !self.visited[next_i][next_j] && self.tiles[next_i][next_j] != Tile::Forest {
                let (next_steps, dead_end) = self.backtrack(next_i, next_j, part2);
                if !dead_end && max < next_steps + 1 {
                    max = next_steps + 1;
                }
            }
        }

        self.visited[i][j] = false;
        return (max, max == 0);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut hike = Hike::parse(&input);

    let mut j = 0;
    while hike.tiles[1][j] == Tile::Forest {
        j += 1;
    }

    println!("{}", hike.backtrack(1, j, true).0);
}
