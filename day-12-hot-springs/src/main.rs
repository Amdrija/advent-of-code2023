use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

fn count_arrangements_line(
    line: &mut Vec<Condition>,
    groups: &Vec<u32>,
    spring: usize,
    mut current_group: usize,
    mut current_group_size: u32,
    cache: &mut HashMap<(usize, usize, u32), u64>,
) -> u64 {
    if spring == line.len() {
        if current_group == groups.len() - 1 && current_group_size != groups[current_group] {
            return 0;
        }

        if current_group < groups.len() - 1 {
            return 0;
        }

        return 1;
    }

    let key = (spring, current_group, current_group_size);
    if let Some(count) = cache.get(&key) {
        return *count;
    }

    let count = match line[spring] {
        Condition::Operational => {
            if spring > 0 && line[spring - 1] == Condition::Damaged {
                if groups[current_group] != current_group_size {
                    cache.insert(key, 0);
                    return 0;
                }

                current_group += 1;
                current_group_size = 0;
            }

            count_arrangements_line(
                line,
                groups,
                spring + 1,
                current_group,
                current_group_size,
                cache,
            )
        }
        Condition::Damaged => {
            if current_group >= groups.len() {
                cache.insert(key, 0);
                return 0;
            }

            current_group_size += 1;
            if current_group_size > groups[current_group] {
                cache.insert(key, 0);
                return 0;
            }

            count_arrangements_line(
                line,
                groups,
                spring + 1,
                current_group,
                current_group_size,
                cache,
            )
        }
        Condition::Unknown => {
            line[spring] = Condition::Damaged;
            let mut arrangements = 0;
            if current_group < groups.len() && current_group_size + 1 <= groups[current_group] {
                arrangements += count_arrangements_line(
                    line,
                    groups,
                    spring + 1,
                    current_group,
                    current_group_size + 1,
                    cache,
                );
            }
            line[spring] = Condition::Operational;
            if spring > 0 && line[spring - 1] == Condition::Damaged {
                if groups[current_group] != current_group_size {
                    arrangements += 0;
                } else {
                    arrangements += count_arrangements_line(
                        line,
                        groups,
                        spring + 1,
                        current_group + 1,
                        0,
                        cache,
                    );
                }
            } else {
                arrangements += count_arrangements_line(
                    line,
                    groups,
                    spring + 1,
                    current_group,
                    current_group_size,
                    cache,
                );
            }
            line[spring] = Condition::Unknown;

            arrangements
        }
    };

    cache.insert(key, count);
    return count;
}

fn parse_line(s: &str) -> Vec<Condition> {
    return s
        .chars()
        .map(|ch| match ch {
            '#' => Condition::Damaged,
            '.' => Condition::Operational,
            '?' => Condition::Unknown,
            _ => {
                panic!("Wrong input.")
            }
        })
        .collect();
}

fn multiply_line(line: Vec<Condition>) -> Vec<Condition> {
    let mut result = Vec::new();
    for _ in 0..5 {
        result.append(&mut line.clone());
        result.push(Condition::Unknown);
    }

    result.pop();

    return result;
}

fn multiply_group(group: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    for _ in 0..5 {
        result.append(&mut group.clone());
    }

    return result;
}

fn line_to_string(line: &Vec<Condition>) -> String {
    return line
        .iter()
        .map(|condition| match condition {
            Condition::Operational => '.',
            Condition::Damaged => '#',
            Condition::Unknown => '?',
        })
        .collect();
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let sum: u64 = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut split = line.split(" ");

            let mut line = parse_line(split.next().unwrap());
            line = multiply_line(line);

            let mut groups: Vec<u32> = split
                .next()
                .unwrap()
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            groups = multiply_group(groups);
            let now = Instant::now();
            let count = count_arrangements_line(&mut line, &groups, 0, 0, 0, &mut HashMap::new());
            println!("{i}: {:.2}", now.elapsed().as_secs_f32());
            return count;
        })
        .sum();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        count_arrangements_line, line_to_string, multiply_group, multiply_line, parse_line,
        Condition,
    };

    #[test]
    fn test_parse_line() {
        let line = parse_line("#.#.??#");

        assert_eq!(
            line,
            vec![
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Damaged
            ]
        );
    }

    #[test]
    fn test_count_arrangements_line() {
        let mut line = parse_line("#.#.###");

        assert_eq!(
            count_arrangements_line(&mut line, &vec![1, 1, 3], 0, 0, 0, &mut HashMap::new()),
            1
        );

        line = parse_line("???.###");
        assert_eq!(
            count_arrangements_line(&mut line, &vec![1, 1, 3], 0, 0, 0, &mut HashMap::new()),
            1
        );

        line = parse_line(".??..#....###.");
        assert_eq!(
            count_arrangements_line(&mut line, &vec![1, 1, 3], 0, 0, 0, &mut HashMap::new()),
            2
        );

        line = parse_line(".??..??...?##.");
        assert_eq!(
            count_arrangements_line(&mut line, &vec![1, 1, 3], 0, 0, 0, &mut HashMap::new()),
            4
        );

        line = parse_line("?#?#?#?#?#?#?#?");
        assert_eq!(
            count_arrangements_line(&mut line, &vec![1, 3, 1, 6], 0, 0, 0, &mut HashMap::new()),
            1
        );

        line = parse_line("?###????????");
        assert_eq!(
            count_arrangements_line(&mut line, &vec![3, 2, 1], 0, 0, 0, &mut HashMap::new()),
            10
        );

        line = parse_line(".##.?#??.#.?#");
        assert_eq!(
            count_arrangements_line(&mut line, &vec![2, 1, 1, 1], 0, 0, 0, &mut HashMap::new()),
            1
        );
    }

    #[test]
    fn test_multiply_group() {
        let group = multiply_group(vec![1, 2]);
        assert_eq!(group, [1, 2, 1, 2, 1, 2, 1, 2, 1, 2]);
    }

    #[test]
    fn test_multiply_line() {
        let line = multiply_line(parse_line("???.###"));

        assert_eq!(
            line_to_string(&line),
            "???.###????.###????.###????.###????.###"
        );
    }
}
