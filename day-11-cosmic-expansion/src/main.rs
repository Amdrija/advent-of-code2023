use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
    io::Empty,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Galaxy,
    Empty,
}

fn expand_rows(space: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut expanded = Vec::new();

    for row in space {
        expanded.push(row.clone());
        if row.iter().all(|tile| *tile == Tile::Empty) {
            expanded.push(row.clone());
        }
    }

    return expanded;
}

fn expand_columns(space: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut expanded = vec![Vec::<Tile>::new(); space.len()];

    for j in 0..space[0].len() {
        let mut to_expand = true;
        for i in 0..space.len() {
            if space[i][j] == Tile::Galaxy {
                to_expand = false;
                break;
            }
        }

        for i in 0..space.len() {
            expanded[i].push(space[i][j]);
            if to_expand {
                expanded[i].push(space[i][j]);
            }
        }
    }

    return expanded;
}

fn find_galaxies(space: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();

    for i in 0..space.len() {
        for j in 0..space[i].len() {
            if space[i][j] == Tile::Galaxy {
                galaxies.push((i, j));
            }
        }
    }

    return galaxies;
}

fn find_distances_sum(galaxies: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    for i in 0..galaxies.len() - 1 {
        result.push(Vec::new());
        for j in i + 1..galaxies.len() {
            let distance =
                galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
            result[i].push(distance);
        }
    }

    return result;
}

fn expand(space: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let expanded_columns = expand_columns(&space);

    return expand_rows(&expanded_columns);
}

fn rows_to_expand(space: &Vec<Vec<Tile>>) -> HashSet<usize> {
    let mut expanded = HashSet::new();

    for (i, row) in space.iter().enumerate() {
        if row.iter().all(|tile| *tile == Tile::Empty) {
            expanded.insert(i);
        }
    }

    return expanded;
}

fn columns_to_expand(space: &Vec<Vec<Tile>>) -> HashSet<usize> {
    let mut expanded = HashSet::new();

    for j in 0..space[0].len() {
        let mut to_expand = true;
        for i in 0..space.len() {
            if space[i][j] == Tile::Galaxy {
                to_expand = false;
                break;
            }
        }

        if to_expand {
            expanded.insert(j);
        }
    }

    return expanded;
}

fn find_distances_sum_sets(
    galaxies: &Vec<(usize, usize)>,
    expanded_rows: &HashSet<usize>,
    expanded_columns: &HashSet<usize>,
    expand_factor: usize,
) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    for i in 0..galaxies.len() - 1 {
        result.push(Vec::new());
        for j in i + 1..galaxies.len() {
            let mut expanded_rows_count = 0;
            for k in min(galaxies[i].0, galaxies[j].0)..max(galaxies[i].0, galaxies[j].0) {
                if expanded_rows.contains(&k) {
                    expanded_rows_count += 1;
                }
            }

            let mut expanded_columns_count = 0;
            for k in min(galaxies[i].1, galaxies[j].1)..max(galaxies[i].1, galaxies[j].1) {
                if expanded_columns.contains(&k) {
                    expanded_columns_count += 1;
                }
            }

            let distance = galaxies[i].0.abs_diff(galaxies[j].0)
                + galaxies[i].1.abs_diff(galaxies[j].1)
                + (expand_factor - 1) * (expanded_columns_count + expanded_rows_count);

            result[i].push(distance);
        }
    }

    return result;
}

fn parse_space(s: &str) -> Vec<Vec<Tile>> {
    return s
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| match tile {
                    '#' => Tile::Galaxy,
                    _ => Tile::Empty,
                })
                .collect()
        })
        .collect();
}

fn get_sum(distances: &Vec<Vec<usize>>) -> usize {
    return distances
        .iter()
        .map(|dist_from_galaxy| dist_from_galaxy.iter().sum::<usize>())
        .sum();
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let space = parse_space(&input);
    let expanded_space = expand(&space);
    let galaxies = find_galaxies(&expanded_space);

    println!("{}", get_sum(&find_distances_sum(&galaxies)));

    let expanded_rows = rows_to_expand(&space);
    let expanded_columns = columns_to_expand(&space);
    let galaxies_non_expanded = find_galaxies(&space);

    println!(
        "{}",
        get_sum(&find_distances_sum_sets(
            &galaxies_non_expanded,
            &expanded_rows,
            &expanded_columns,
            1000000
        ))
    );
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use crate::{
        columns_to_expand, expand, find_distances_sum, find_distances_sum_sets, find_galaxies,
        get_sum, parse_space, rows_to_expand, Tile,
    };

    #[test]
    fn test_parse() {
        let input = fs::read_to_string("test").unwrap();
        let space = parse_space(&input);
        let galaxySet = HashSet::from([
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4),
        ]);

        for i in 0..space.len() {
            for j in 0..space[i].len() {
                if galaxySet.contains(&(i, j)) {
                    assert_eq!(space[i][j], Tile::Galaxy);
                } else {
                    assert_eq!(space[i][j], Tile::Empty);
                }
            }
        }
    }

    #[test]
    fn test_expand() {
        let input = fs::read_to_string("test").unwrap();
        let space = parse_space(&input);
        let expanded_space = expand(&space);

        let expanded_input = fs::read_to_string("test_expanded").unwrap();
        let expected_expanded = parse_space(&expanded_input);

        assert_eq!(expanded_space, expected_expanded);
    }

    #[test]
    fn test_find_galaxies() {
        let input = fs::read_to_string("test").unwrap();
        let space = parse_space(&input);
        let expected_galaxies = vec![
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4),
        ];

        assert_eq!(find_galaxies(&space), expected_galaxies);
    }

    #[test]
    fn test_find_distances() {
        let input = fs::read_to_string("test").unwrap();
        let space = parse_space(&input);
        let expanded_space = expand(&space);
        let galaxies = find_galaxies(&expanded_space);

        assert_eq!(get_sum(&find_distances_sum(&galaxies)), 374);
    }

    #[test]
    fn test_rows_to_expand() {
        let input = fs::read_to_string("test").unwrap();
        let space = parse_space(&input);
        let expanded_rows = rows_to_expand(&space);

        assert_eq!(expanded_rows, HashSet::from([3, 7]));
    }

    #[test]
    fn test_columns_to_expand() {
        let input = fs::read_to_string("test").unwrap();
        let space = parse_space(&input);
        let expanded_rows = columns_to_expand(&space);

        assert_eq!(expanded_rows, HashSet::from([2, 5, 8]));
    }

    #[test]
    fn test_find_distances_sum_set() {
        let input = fs::read_to_string("test").unwrap();
        let space = parse_space(&input);
        let galaxies = find_galaxies(&space);
        let expanded_rows = rows_to_expand(&space);
        let expanded_columns = columns_to_expand(&space);

        let sum_sets = find_distances_sum_sets(&galaxies, &expanded_rows, &expanded_columns, 2);
        assert_eq!(get_sum(&sum_sets), 374);
    }
}
