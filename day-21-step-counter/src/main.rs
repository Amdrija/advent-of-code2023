use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

fn parse(s: &str) -> (Vec<Vec<Tile>>, usize, usize) {
    let mut start_i = 0;
    let mut start_j = 0;

    let garden = s
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    '#' => Tile::Rock,
                    '.' => Tile::Plot,
                    'S' => {
                        start_i = i;
                        start_j = j;
                        Tile::Plot
                    }
                    _ => panic!("Unknown tile {}", ch),
                })
                .collect()
        })
        .collect();

    (garden, start_i, start_j)
}

fn part1(garden: &Vec<Vec<Tile>>, i: usize, j: usize, steps: u64) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((i, j));

    for _ in 0..steps {
        let mut next_q: HashSet<(usize, usize)> = HashSet::new();

        while let Some((i, j)) = q.pop_front() {
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_i = i as i64 + di;
                let new_j = j as i64 + dj;
                if new_i < 0
                    || new_j < 0
                    || new_i >= garden.len() as i64
                    || new_j >= garden[0].len() as i64
                    || garden[new_i as usize][new_j as usize] == Tile::Rock
                {
                    continue;
                }

                next_q.insert((new_i as usize, new_j as usize));
            }
        }

        q = next_q.into_iter().collect::<VecDeque<_>>();
    }

    return q.len();
}

fn part2(garden: &Vec<Vec<Tile>>, i: i64, j: i64, steps: u64) -> usize {
    let mut q: VecDeque<(i64, i64)> = VecDeque::new();
    q.push_back((i, j));

    for _ in 0..steps {
        let mut next_q: HashSet<(i64, i64)> = HashSet::new();

        while let Some((i, j)) = q.pop_front() {
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_i = i + di;
                let new_j = j + dj;
                if garden[new_i.rem_euclid(garden.len() as i64) as usize]
                    [new_j.rem_euclid(garden[0].len() as i64) as usize]
                    == Tile::Rock
                {
                    continue;
                }

                next_q.insert((new_i, new_j));
            }
        }

        q = next_q.into_iter().collect::<VecDeque<_>>();
    }

    return q.len();
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (garden, start_i, start_j) = parse(&input);

    println!("{}", part1(&garden, start_i, start_j, 6));
    println!(
        "{}",
        part2(&garden, start_i as i64, start_j as i64, 26501365)
    );

    //For part 2, again had to do it by hand, because of the propoerties of the input
}
