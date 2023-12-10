use std::{collections::VecDeque, fs, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Cell {
    i: usize,
    j: usize,
}

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
}

impl Pipe {
    fn get_next(&self, i: usize, j: usize) -> (Cell, Cell) {
        return match self {
            Pipe::Vertical => (Cell { i: i + 1, j }, Cell { i: i - 1, j }),
            Pipe::Horizontal => (Cell { i, j: j + 1 }, Cell { i, j: j - 1 }),
            Pipe::NorthEast => (Cell { i: i - 1, j }, Cell { i, j: j + 1 }),
            Pipe::NorthWest => (Cell { i: i - 1, j }, Cell { i, j: j - 1 }),
            Pipe::SouthWest => (Cell { i: i + 1, j }, Cell { i, j: j - 1 }),
            Pipe::SouthEast => (Cell { i: i + 1, j }, Cell { i, j: j + 1 }),
            Pipe::Ground => (Cell { i, j }, Cell { i, j }),
        };
    }
}

fn parse_pipe(ch: char) -> Pipe {
    return match ch {
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        _ => Pipe::Ground,
    };
}

fn get_next_from_start(pipes: &Vec<Vec<Pipe>>, start: &Cell) -> VecDeque<Cell> {
    let mut q = VecDeque::new();
    let diffs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for diff in diffs {
        let i = start.i as i32 + diff.0;
        let j = start.j as i32 + diff.1;
        if i >= 0 && i < pipes.len() as i32 && j >= 0 && j < pipes[0].len() as i32 {
            let i = i as usize;
            let j = j as usize;
            let next = pipes[i][j].get_next(i, j);
            if (next.0.i == start.i && next.0.j == start.j)
                || (next.1.i == start.i && next.1.j == start.j)
            {
                q.push_back(Cell { i, j });
            }
        }
    }

    return q;
}

fn determine_start(start: &Cell, next: &VecDeque<Cell>) -> Pipe {
    if next[0].i.abs_diff(next[1].i) == 2 {
        return Pipe::Vertical;
    }

    if next[0].j.abs_diff(next[1].j) == 2 {
        return Pipe::Horizontal;
    }

    //this means that the pipe must go from north
    if start.i as i32 - next[0].i as i32 == 1 {
        //this means that the pipe must go to west
        if start.j as i32 - next[1].j as i32 == 1 {
            return Pipe::NorthWest;
        } else {
            //because we already tested north->south
            // (vertical pipe) and north->west
            // this is the only one left;
            return Pipe::NorthEast;
        }
    }

    //otherwise, the pipe must go to south
    //this means the pipe must go to west
    if start.j as i32 - next[1].j as i32 == 1 {
        return Pipe::SouthWest;
    }

    return Pipe::SouthEast;
}

fn bfs(pipes: &mut Vec<Vec<Pipe>>, start: &Cell) -> (usize, Vec<Vec<bool>>) {
    let mut level = 1;
    let mut q: VecDeque<Cell> = get_next_from_start(&pipes, start);
    let start_pipe = determine_start(start, &q);
    pipes[start.i][start.j] = start_pipe;
    println!("{:?}", pipes[start.i][start.j]);

    let mut visited = pipes
        .iter()
        .map(|pipeline| pipeline.iter().map(|_| false).collect())
        .collect::<Vec<Vec<bool>>>();
    visited[start.i][start.j] = true;

    while !q.is_empty() {
        let mut next_level = VecDeque::new();
        while !q.is_empty() {
            let current = q.pop_front().unwrap();
            visited[current.i][current.j] = true;
            let next = pipes[current.i][current.j].get_next(current.i, current.j);
            if !visited[next.0.i][next.0.j] {
                next_level.push_back(next.0);
            }
            if !visited[next.1.i][next.1.j] {
                next_level.push_back(next.1);
            }
        }
        q = next_level;
        level += 1;
    }

    return (level, visited);
}

fn get_inside_surfice(pipes: &Vec<Vec<Pipe>>, is_loop: &Vec<Vec<bool>>) -> u32 {
    let mut inside_count = 0;
    let mut is_inside = false;

    for i in 0..pipes.len() {
        for j in 0..pipes[0].len() {
            if is_loop[i][j] {
                match pipes[i][j] {
                    Pipe::Vertical | Pipe::SouthWest | Pipe::SouthEast => is_inside = !is_inside,
                    _ => {}
                };
            } else if is_inside {
                inside_count += 1;
            }
        }
    }

    return inside_count;
}

fn parse_pipes(s: &str) -> (Vec<Vec<Pipe>>, Cell) {
    let mut result = Vec::new();
    let mut start = Cell { i: 0, j: 0 };
    for (i, line) in s.lines().enumerate() {
        result.push(
            line.chars()
                .enumerate()
                .map(|(j, ch)| {
                    if ch == 'S' {
                        start.i = i;
                        start.j = j;
                    }
                    return parse_pipe(ch);
                })
                .collect::<Vec<Pipe>>(),
        );
    }

    return (result, start);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (mut pipes, start) = parse_pipes(&input);

    let (levels, is_loop) = bfs(&mut pipes, &start);
    println!("{}", levels - 1);

    println!("{}", get_inside_surfice(&pipes, &is_loop));
}

#[cfg(test)]
mod tests {
    use crate::{Cell, Pipe};

    #[test]
    fn test_get_next() {
        let pipe = Pipe::Vertical.get_next(2, 3);
        let pipe = [pipe.0, pipe.1];
        assert!(pipe.contains(&Cell { i: 3, j: 3 }));
        assert!(pipe.contains(&Cell { i: 1, j: 3 }));

        let pipe = Pipe::Horizontal.get_next(2, 3);
        let pipe = [pipe.0, pipe.1];
        assert!(pipe.contains(&Cell { i: 2, j: 2 }));
        assert!(pipe.contains(&Cell { i: 2, j: 4 }));

        let pipe = Pipe::NorthEast.get_next(2, 3);
        let pipe = [pipe.0, pipe.1];
        assert!(pipe.contains(&Cell { i: 1, j: 3 }));
        assert!(pipe.contains(&Cell { i: 2, j: 4 }));

        let pipe = Pipe::NorthWest.get_next(2, 3);
        let pipe = [pipe.0, pipe.1];
        assert!(pipe.contains(&Cell { i: 1, j: 3 }));
        assert!(pipe.contains(&Cell { i: 2, j: 2 }));

        let pipe = Pipe::SouthWest.get_next(2, 3);
        let pipe = [pipe.0, pipe.1];
        assert!(pipe.contains(&Cell { i: 3, j: 3 }));
        assert!(pipe.contains(&Cell { i: 2, j: 2 }));

        let pipe = Pipe::SouthEast.get_next(2, 3);
        let pipe = [pipe.0, pipe.1];
        assert!(pipe.contains(&Cell { i: 3, j: 3 }));
        assert!(pipe.contains(&Cell { i: 2, j: 4 }));
    }
}
