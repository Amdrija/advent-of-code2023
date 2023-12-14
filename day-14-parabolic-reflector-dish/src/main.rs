use std::{collections::HashMap, fs};

fn calculate_load_1(platform: &str) -> usize {
    let mut load = 0;
    let rows = platform.lines().collect::<Vec<_>>();
    let mut previous_tilt = vec![-1; rows[0].len()];

    for (i, row) in rows.iter().enumerate() {
        for (j, tile) in row.chars().enumerate() {
            match tile {
                'O' => {
                    previous_tilt[j] += 1;
                    load += rows.len() - previous_tilt[j] as usize;
                }
                '#' => {
                    previous_tilt[j] = i as i32;
                }
                _ => (),
            };
        }

        // println!("{:?}", previous_tilt);
    }

    return load;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Round,
    Square,
    Empty,
}

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn transpose(platform: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let m = platform[0].len();
    let n = platform.len();
    let mut transposed = vec![vec![Tile::Empty; n]; m];

    for i in 0..n {
        for j in 0..m {
            transposed[j][i] = platform[i][j];
        }
    }

    return transposed;
}

fn mirror(platform: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let m = platform[0].len();
    let n = platform.len();
    let mut mirrored = vec![vec![Tile::Empty; m]; n];

    for i in 0..n {
        for j in 0..m {
            mirrored[n - 1 - i][m - 1 - j] = platform[i][j];
        }
    }

    return mirrored;
}

fn transform_to_north(platform: Vec<Vec<Tile>>, direction: &Direction) -> Vec<Vec<Tile>> {
    return match direction {
        Direction::North => platform,
        Direction::West => transpose(platform),
        Direction::South => mirror(platform),
        Direction::East => transpose(mirror(platform)),
    };
}

fn tilt_platform(platform: Vec<Vec<Tile>>, direction: &Direction) -> Vec<Vec<Tile>> {
    let platform = transform_to_north(platform, direction);

    let n = platform.len();
    let m = platform[0].len();
    let mut new_position = vec![vec![-1; m]; n];

    for j in 0..m {
        match platform[0][j] {
            Tile::Round => new_position[0][j] = 0,
            Tile::Square => new_position[0][j] = 0,
            Tile::Empty => (),
        }
    }

    for i in 1..n {
        for j in 0..m {
            new_position[i][j] = match platform[i][j] {
                Tile::Round => new_position[i - 1][j] + 1,
                Tile::Square => i as i32,
                Tile::Empty => new_position[i - 1][j],
            };
        }
    }

    let mut result = vec![vec![Tile::Empty; m]; n];
    for i in 0..n {
        for j in 0..m {
            match platform[i][j] {
                Tile::Round => result[new_position[i][j] as usize][j] = Tile::Round,
                Tile::Square => result[new_position[i][j] as usize][j] = Tile::Square,
                Tile::Empty => (),
            }
        }
    }

    return transform_to_north(result, direction);
}

fn cycle_platform(mut platform: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let directions = vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    for direction in directions {
        platform = tilt_platform(platform, &direction);
    }

    return platform;
}

fn to_string(platform: &Vec<Vec<Tile>>) -> String {
    let mut result = String::new();

    for row in platform {
        for tile in row {
            result += match tile {
                Tile::Round => "O",
                Tile::Square => "#",
                Tile::Empty => ".",
            }
        }

        result += "\n";
    }

    return result;
}

fn parse(s: &str) -> Vec<Vec<Tile>> {
    return s
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'O' => Tile::Round,
                    '#' => Tile::Square,
                    '.' => Tile::Empty,
                    _ => panic!("Unknown character! {}", ch),
                })
                .collect()
        })
        .collect();
}

fn calculate_load(platform: &Vec<Vec<Tile>>) -> usize {
    let mut load = 0;

    for i in 0..platform.len() {
        for tile in &platform[i] {
            if *tile == Tile::Round {
                load += platform.len() - i;
            }
        }
    }

    return load;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut platform = parse(&input);
    let tilted = tilt_platform(platform.clone(), &Direction::North);

    println!("{}", calculate_load(&tilted));

    let mut platforms: HashMap<String, usize> = HashMap::new();
    platforms.insert(to_string(&platform), 0);

    let mut end = platform.clone();
    let mut period = 0;
    let mut start = 0;
    for i in 1..1000000001 {
        if i % 100 == 0 {
            println!("{i}");
        }

        let cycled = cycle_platform(platform);
        let key = to_string(&cycled);
        if let Some(last_i) = platforms.get(&key) {
            period = i - last_i;
            start = *last_i;
            break;
        } else {
            platforms.insert(key, i);
        }

        platform = cycled;
    }

    let remain = (1000000000 - start) % period;
    for i in 0..remain + start {
        end = cycle_platform(end);
    }

    println!("{} {} {}", period, start, calculate_load(&end));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{cycle_platform, parse, tilt_platform, transform_to_north, Direction};

    #[test]
    fn test_transform_to_north() {
        let input = fs::read_to_string("test.txt");

        assert!(input.is_ok());

        let platform = parse(&input.unwrap());
        let directions = vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];

        for direction in directions {
            let transformed = transform_to_north(platform.clone(), &direction);
            let original = transform_to_north(transformed.clone(), &direction);
            assert_eq!(platform, original);
        }
    }

    #[test]
    fn test_tilt_platform() {
        let input = fs::read_to_string("test.txt");
        assert!(input.is_ok());

        let expected = fs::read_to_string("test_tilted.txt");
        assert!(expected.is_ok());

        let platform = parse(&input.unwrap());
        let expected_tilted = parse(&expected.unwrap());

        let tilted = tilt_platform(platform.clone(), &Direction::North);

        assert_eq!(tilted, expected_tilted);
    }

    #[test]
    fn test_cycle_platform() {
        let input = fs::read_to_string("test.txt");
        assert!(input.is_ok());

        let expected = fs::read_to_string("test_cycled.txt");
        assert!(expected.is_ok());

        let platform = parse(&input.unwrap());
        let expected = parse(&expected.unwrap());

        let cycled = cycle_platform(platform);
        assert_eq!(cycled, expected);

        let expected = fs::read_to_string("test_cycled_2.txt");
        assert!(expected.is_ok());

        let cycled2 = cycle_platform(cycled);
        let expected = parse(&expected.unwrap());
        assert_eq!(cycled2, expected);

        let expected = fs::read_to_string("test_cycled_3.txt");
        assert!(expected.is_ok());

        let cycled3 = cycle_platform(cycled2);
        let expected = parse(&expected.unwrap());
        assert_eq!(cycled3, expected);
    }
}
