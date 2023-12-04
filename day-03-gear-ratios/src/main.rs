use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

fn get_adjacent_indexes(i: usize, j: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let delta: [i32; 3] = [-1, 0, 1];
    let delta = delta
        .iter()
        .map(|di| delta.map(|dj| (di, dj)))
        .flatten()
        .filter(|(di, dj)| **di != 0 || *dj != 0);

    return delta
        .map(|(di, dj)| {
            (
                i32::try_from(i).unwrap() + di,
                i32::try_from(j).unwrap() + dj,
            )
        })
        .filter(|(ni, nj)| {
            *ni >= 0
                && *ni < i32::try_from(n).unwrap()
                && *nj >= 0
                && *nj < i32::try_from(m).unwrap()
        })
        .map(|(ni, nj)| (ni as usize, nj as usize))
        .collect::<Vec<(usize, usize)>>();
}

fn find_nums_adjacent_symbols(schematic: &Vec<Vec<char>>) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    for i in 0..schematic.len() {
        let mut num = 0;
        let mut adjacent = false;
        for j in 0..schematic[i].len() {
            if schematic[i][j].is_numeric() {
                num = num * 10 + schematic[i][j].to_digit(10).unwrap();

                for (ni, nj) in get_adjacent_indexes(i, j, schematic.len(), schematic[i].len()) {
                    if schematic[ni][nj] != '.' && !schematic[ni][nj].is_alphanumeric() {
                        adjacent = true;
                    }
                }
            } else {
                if adjacent {
                    nums.push(num);
                }
                adjacent = false;
                num = 0;
            }
        }

        if adjacent {
            nums.push(num);
        }
    }

    return nums;
}

fn find_nums_gear_ratios(schematic: &Vec<Vec<char>>) -> Vec<u32> {
    let mut potential_gears: HashMap<i32, Vec<u32>> = HashMap::new();

    for i in 0..schematic.len() {
        let mut num = 0;
        let mut adjacent: i32 = -1;
        for j in 0..schematic[i].len() {
            if schematic[i][j].is_numeric() {
                num = num * 10 + schematic[i][j].to_digit(10).unwrap();

                for (ni, nj) in get_adjacent_indexes(i, j, schematic.len(), schematic[i].len()) {
                    if schematic[ni][nj] == '*' {
                        adjacent = i32::try_from(ni * schematic.len() + nj).unwrap();
                    }
                }
            } else {
                if adjacent >= 0 {
                    potential_gears
                        .entry(adjacent)
                        .or_insert(Vec::new())
                        .push(num);
                }
                adjacent = -1;
                num = 0;
            }
        }

        if adjacent >= 0 {
            potential_gears
                .entry(adjacent)
                .or_insert(Vec::new())
                .push(num);
        }
    }

    return potential_gears
        .into_iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums[0] * nums[1])
        .collect();
}

fn parse_schematic(buf: impl BufRead) -> Vec<Vec<char>> {
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in buf.lines() {
        schematic.push(line.unwrap().chars().collect());
    }

    return schematic;
}
fn main() {
    let file = File::open("input").unwrap();

    let schematic = parse_schematic(BufReader::new(file));

    let nums = find_nums_adjacent_symbols(&schematic);
    println!("{}", nums.iter().sum::<u32>());

    let gear_ratios = find_nums_gear_ratios(&schematic);
    println!("{:?}", gear_ratios.iter().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::{find_nums_adjacent_symbols, find_nums_gear_ratios, parse_schematic};

    const SCHEMATIC: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
3.....7556
123$.*..12
.664.598..
";
    #[test]
    fn test_schematic_calculation() {
        let schematic = parse_schematic(BufReader::new(SCHEMATIC.as_bytes()));

        let mut nums = find_nums_adjacent_symbols(&schematic);
        println!("{:?}", nums);
        let mut expected_nums = [467, 35, 633, 617, 592, 7556, 123, 664, 598];

        assert_eq!(nums.len(), expected_nums.len());
        nums.sort();
        expected_nums.sort();
        assert_eq!(nums, expected_nums);
    }

    #[test]
    fn test_gear_ratios() {
        let schematic = parse_schematic(BufReader::new(SCHEMATIC.as_bytes()));

        let mut gear_ratios = find_nums_gear_ratios(&schematic);
        println!("{:?}", gear_ratios);
        let mut expected_gear_ratios = [467 * 35, 7556 * 598];

        assert_eq!(gear_ratios.len(), expected_gear_ratios.len());
        gear_ratios.sort();
        expected_gear_ratios.sort();
        assert_eq!(gear_ratios, expected_gear_ratios);
    }
}
