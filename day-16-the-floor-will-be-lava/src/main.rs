use std::{fs, vec};

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    BackMirror,
    ForwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_coord(&self, i: i64, j: i64) -> (i64, i64) {
        return match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        };
    }

    fn next_direction(&self, tile_type: &TileType) -> Vec<Direction> {
        return match (tile_type, self) {
            (TileType::BackMirror, Direction::Up) => vec![Direction::Right],
            (TileType::BackMirror, Direction::Down) => vec![Direction::Left],
            (TileType::BackMirror, Direction::Right) => vec![Direction::Up],
            (TileType::BackMirror, Direction::Left) => vec![Direction::Down],
            (TileType::ForwardMirror, Direction::Up) => vec![Direction::Left],
            (TileType::ForwardMirror, Direction::Down) => vec![Direction::Right],
            (TileType::ForwardMirror, Direction::Right) => vec![Direction::Down],
            (TileType::ForwardMirror, Direction::Left) => vec![Direction::Up],
            (TileType::HorizontalSplitter, Direction::Up | Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }
            (TileType::VerticalSplitter, Direction::Left | Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }
            _ => vec![self.clone()],
        };
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        return match self {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
        }
        .to_string();
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        return match value {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            n => panic!("Unkown enum direction value: {n}"),
        };
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        return self as usize;
    }
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    visited: [bool; 4],
}

impl Tile {
    fn new(tile_type: TileType) -> Tile {
        return Tile {
            tile_type,
            visited: [false; 4],
        };
    }

    fn parse(s: &str) -> Vec<Vec<Tile>> {
        return s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Tile::new(TileType::Empty),
                        '/' => Tile::new(TileType::BackMirror),
                        '\\' => Tile::new(TileType::ForwardMirror),
                        '|' => Tile::new(TileType::VerticalSplitter),
                        '-' => Tile::new(TileType::HorizontalSplitter),
                        _ => panic!("Unknown character {}", ch),
                    })
                    .collect()
            })
            .collect();
    }

    fn tiles_to_string(tiles: &Vec<Vec<Tile>>) -> String {
        return tiles
            .iter()
            .map(|row| {
                row.iter()
                    .fold(String::new(), |res, tile| res + &tile.to_string())
            })
            .fold(String::new(), |res, row| res + &row + "\n");
    }

    fn is_visited(&self) -> bool {
        return self.visited.iter().any(|d| *d);
    }

    fn to_string(&self) -> String {
        let mut result = String::from(".");

        for (i, direction) in self.visited.iter().enumerate() {
            if *direction && result == "." {
                result = Direction::from(i).to_string();
            } else if *direction && result.len() == 1 {
                result = String::from("2");
            } else if *direction {
                result = result.parse::<usize>().unwrap().to_string();
            }
        }

        return result;
    }
}

fn beam(tiles: &mut Vec<Vec<Tile>>, i: i64, j: i64, direction: Direction) {
    if i < 0 || i as usize >= tiles.len() || j < 0 || j as usize >= tiles[0].len() {
        return;
    }

    let dir: usize = direction.clone().into();

    let i = i as usize;
    let j = j as usize;
    if tiles[i][j].visited[dir] {
        return;
    }

    tiles[i][j].visited[dir] = true;

    let new_directions = direction.next_direction(&tiles[i][j].tile_type);
    for new_direction in new_directions {
        let (new_i, new_j) = new_direction.next_coord(i as i64, j as i64);
        beam(tiles, new_i, new_j, new_direction);
    }
}

fn energized_count(tiles: &Vec<Vec<Tile>>) -> usize {
    return tiles
        .iter()
        .map(|row| row.iter().filter(|tile| tile.is_visited()).count())
        .sum();
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let tiles = Tile::parse(&input);

    let mut max = 0;
    for i in 0..tiles.len() {
        let mut left_start = tiles.clone();
        beam(&mut left_start, i as i64, 0, Direction::Right);
        let energized = energized_count(&left_start);

        if energized > max {
            max = energized;
        }

        let mut right_start = tiles.clone();
        beam(
            &mut right_start,
            i as i64,
            (tiles[0].len() - 1) as i64,
            Direction::Left,
        );
        let energized = energized_count(&right_start);

        if energized > max {
            max = energized;
        }
    }

    for j in 0..tiles[0].len() {
        let mut up_start = tiles.clone();
        beam(&mut up_start, 0, j as i64, Direction::Down);
        let energized = energized_count(&up_start);

        if energized > max {
            max = energized;
        }

        let mut down_start = tiles.clone();
        beam(
            &mut up_start,
            (tiles.len() - 1) as i64,
            j as i64,
            Direction::Up,
        );
        let energized = energized_count(&down_start);

        if energized > max {
            max = energized;
        }
    }

    // println!("{}", Tile::tiles_to_string(&tiles));
    println!("{}", max);
}
