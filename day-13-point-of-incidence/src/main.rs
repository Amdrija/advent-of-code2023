use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Ash,
    Rock,
}

#[derive(Debug)]
struct ParseSubPatternError;

fn parse_line(s: &str) -> Result<Vec<Element>, ParseSubPatternError> {
    return s
        .chars()
        .map(|ch| match ch {
            '.' => Ok(Element::Ash),
            '#' => Ok(Element::Rock),
            _ => Err(ParseSubPatternError),
        })
        .collect::<Result<Vec<Element>, ParseSubPatternError>>();
}

fn parse_pattern(input: &str) -> Result<Vec<Vec<Element>>, ParseSubPatternError> {
    return input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Result<Vec<Vec<Element>>, ParseSubPatternError>>();
}

fn transpose(pattern: &Vec<Vec<Element>>) -> Vec<Vec<Element>> {
    let mut transposed = vec![vec![Element::Ash; pattern.len()]; pattern[0].len()];

    for i in 0..pattern.len() {
        for j in 0..pattern[i].len() {
            transposed[j][i] = pattern[i][j].clone();
        }
    }

    return transposed;
}

fn check_expand(
    pattern: &Vec<Vec<Element>>,
    left: i64,
    right: usize,
    error_threshold: usize,
) -> usize {
    let mut left = left;
    let mut right = right;

    let mut errors = 0;
    while left >= 0 && right < pattern.len() {
        for j in 0..pattern[0].len() {
            if pattern[left as usize][j] != pattern[right][j] {
                errors += 1;
            }
        }

        if errors > error_threshold {
            break;
        }

        left -= 1;
        right += 1;
    }

    return errors;
}

fn get_indexes_from_middle(length: usize) -> Vec<usize> {
    let i = length / 2;
    let mut rows_to_check: Vec<usize> = Vec::new();
    rows_to_check.push(i);
    let mut step = 1;

    while rows_to_check.len() != length {
        if (i as i64 - step as i64) >= 0 {
            rows_to_check.push(i - step);
        }

        if i + step < length {
            rows_to_check.push(i + step);
        }

        step += 1;
    }

    return rows_to_check;
}

fn get_mirror(pattern: &Vec<Vec<Element>>, error_threshold: usize) -> usize {
    for i in get_indexes_from_middle(pattern.len()) {
        if i == pattern.len() - 1 {
            continue;
        }

        let errors = check_expand(pattern, i as i64, i + 1, error_threshold);
        if errors == error_threshold {
            return i + 1;
        }
    }

    return 0;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for pattern_str in input.split("\n\n") {
        let pattern = parse_pattern(pattern_str).unwrap();

        let rows = get_mirror(&pattern, 1);

        if rows == 0 {
            let columns = get_mirror(&transpose(&pattern), 1);
            sum += columns;
        } else {
            sum += rows * 100;
        }
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{get_indexes_from_middle, get_mirror, parse_pattern, transpose, Element};

    fn get_pattern(file: &str) -> Vec<Vec<Element>> {
        let input = fs::read_to_string(file).unwrap();
        let pattern = parse_pattern(&input);

        assert!(pattern.is_ok());

        return pattern.unwrap();
    }

    #[test]
    fn test_get_indexes_from_middle() {
        assert_eq!(get_indexes_from_middle(5), vec![2, 1, 3, 0, 4]);
        assert_eq!(get_indexes_from_middle(6), vec![3, 2, 4, 1, 5, 0]);
    }

    #[test]
    fn test_transpose() {
        let pattern = vec![
            vec![Element::Ash, Element::Rock],
            vec![Element::Ash, Element::Rock],
            vec![Element::Rock, Element::Ash],
        ];

        let expected_transposed = vec![
            vec![Element::Ash, Element::Ash, Element::Rock],
            vec![Element::Rock, Element::Rock, Element::Ash],
        ];

        assert_eq!(transpose(&pattern), expected_transposed);
    }

    #[test]
    fn test_get_mirror() {
        assert_eq!(get_mirror(&get_pattern("test_row_1.txt"), 0), 4);
        assert_eq!(get_mirror(&get_pattern("test_row_2.txt"), 0), 14);
        assert_eq!(get_mirror(&get_pattern("test_column_1.txt"), 0), 0);
        assert_eq!(get_mirror(&get_pattern("buggy.txt"), 0), 1);

        assert_eq!(
            get_mirror(&transpose(&get_pattern("test_column_1.txt")), 0),
            5
        );
        assert_eq!(
            get_mirror(&transpose(&get_pattern("test_column_2.txt")), 0),
            10
        );
        assert_eq!(get_mirror(&transpose(&get_pattern("test_row_1.txt")), 0), 0);
    }
}
