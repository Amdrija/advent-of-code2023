use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_diff(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn is_opposite(&self, other: &Direction) -> bool {
        match self {
            Direction::Up => other == &Direction::Down,
            Direction::Right => other == &Direction::Left,
            Direction::Down => other == &Direction::Up,
            Direction::Left => other == &Direction::Right,
        }
    }
}

fn min_heat_loss(map: &Vec<Vec<u8>>, min_step: i64, max_step: i64) -> u64 {
    let N = map.len();
    let M = map[0].len();

    // our unique "vertex" is based on the position of the cell and the direction
    // from which we entered the cell.
    let mut pq: BinaryHeap<(i64, (usize, usize), Direction)> = BinaryHeap::new();
    let mut best: HashMap<(usize, usize, Direction), i64> = HashMap::new();

    pq.push((0, (0, 0), Direction::Right));
    pq.push((0, (0, 0), Direction::Down));

    while let Some((cost, (i, j), direction)) = pq.pop() {
        let cost = -cost;
        if i == N - 1 && j == M - 1 {
            return cost as u64;
        }

        let key = (i, j, direction.clone());
        if best.contains_key(&key) && best[&key] < cost {
            continue;
        }

        for next_direction in vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            //The main idea is when we change the direction to add as much steps
            // as we can to the priority queue. To be precise we add the cells
            // which are between min_steps and max_steps away from the current
            // cell, but in the new direction
            if next_direction == direction || direction.is_opposite(&next_direction) {
                continue;
            }

            let mut new_cost = cost;
            let (diff_i, diff_j) = next_direction.get_diff();
            for distance in 1..=max_step {
                let next_i = i as i64 + diff_i * distance;
                let next_j = j as i64 + diff_j * distance;

                if next_i < 0 || next_i >= N as i64 || next_j < 0 || next_j >= M as i64 {
                    break;
                }

                let next_i = next_i as usize;
                let next_j = next_j as usize;

                new_cost += map[next_i][next_j] as i64;
                if distance < min_step {
                    continue;
                }

                let key = (next_i, next_j, next_direction.clone());
                if !best.contains_key(&key) || new_cost < best[&key] {
                    best.insert(key, new_cost);
                    pq.push((-new_cost, (next_i, next_j), next_direction.clone()));
                }
            }
        }
    }

    return u64::MAX;
}

fn parse(s: &str) -> Vec<Vec<u8>> {
    let mut map = Vec::new();

    for (i, line) in s.lines().enumerate() {
        map.push(Vec::new());
        for (_, ch) in line.chars().enumerate() {
            map[i].push(ch.to_digit(10).unwrap() as u8);
        }
    }

    return map;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map = parse(&input);
    println!("{}", min_heat_loss(&map, 1, 3));
    println!("{}", min_heat_loss(&map, 4, 10));
}
