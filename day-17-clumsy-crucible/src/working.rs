use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

fn min_heat_loss(map: &Vec<Vec<u8>>, min_step: i64, max_step: i64) -> u64 {
    let N = map.len();
    let M = map[0].len();

    let mut pq: BinaryHeap<(i64, (usize, usize), (i64, i64))> = BinaryHeap::new();
    let mut best: HashMap<(usize, usize, (i64, i64)), i64> = HashMap::new();

    pq.push((0, (0, 0), (0, 0)));

    while let Some((cost, (i, j), direction)) = pq.pop() {
        let cost = -cost;
        if i == N - 1 && j == M - 1 {
            return cost as u64;
        }

        let key = (i, j, direction);
        if best.contains_key(&key) && best[&key] < cost {
            continue;
        }

        for (diff_i, diff_j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if (diff_i, diff_j) == direction || (-diff_i, -diff_j) == direction {
                continue;
            }

            let mut new_cost = cost;
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

                let key = (next_i, next_j, (diff_i, diff_j));
                if !best.contains_key(&key) || new_cost < best[&key] {
                    best.insert(key, new_cost);
                    pq.push((-new_cost, (next_i, next_j), (diff_i, diff_j)));
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
