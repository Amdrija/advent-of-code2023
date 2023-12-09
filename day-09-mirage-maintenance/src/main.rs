use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn predict_next(sequence: Vec<i64>) -> (i64, i64) {
    if sequence.iter().all(|el| *el == 0) {
        return (0, 0);
    }

    let mut diffs = Vec::new();
    for i in 1..sequence.len() {
        diffs.push(sequence[i] - sequence[i - 1]);
    }

    let next_diff = predict_next(diffs);

    return (
        sequence.first().unwrap() - next_diff.0,
        sequence.last().unwrap() + next_diff.1,
    );
}

fn main() {
    let file = File::open("input").unwrap();

    let sum: (i64, i64) = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|sequence| predict_next(sequence))
        .fold((0, 0), |sums, predicted| {
            (sums.0 + predicted.0, sums.1 + predicted.1)
        });

    println!("Previous sum: {}, Next sum: {}", sum.0, sum.1);
}

#[cfg(test)]
mod tests {
    use crate::predict_next;

    #[test]
    fn test_predict_next() {
        assert_eq!(predict_next(vec![0, 3, 6, 9, 12, 15]), (-3, 18));
        assert_eq!(predict_next(vec![1, 3, 6, 10, 15, 21]), (0, 28));
        assert_eq!(predict_next(vec![10, 13, 16, 21, 30, 45]), (5, 68));
    }
}
