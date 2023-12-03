use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut sum_part_one = 0;
    let mut sum_part_two = 0;

    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        sum_part_one += line
            .as_ref()
            .unwrap()
            .chars()
            .find_map(|c| c.to_digit(10))
            .unwrap()
            * 10
            + line
                .as_ref()
                .unwrap()
                .chars()
                .rev()
                .find_map(|c| c.to_digit(10))
                .unwrap();

        sum_part_two += callibration_number(&line.unwrap());
    }

    println!("{}", sum_part_one);
    println!("{}", sum_part_two);
}

fn callibration_number(line: &str) -> u32 {
    let digit_words: Vec<(String, usize)> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, w)| (String::from(w), i))
    .collect();

    let first_digit = find_digit(line.chars().collect(), &digit_words);

    let reversed_words = digit_words
        .into_iter()
        .map(|(w, i)| (w.chars().rev().collect::<String>(), i))
        .collect();

    let last_digit = find_digit(line.chars().rev().collect(), &reversed_words);

    return first_digit * 10 + last_digit;
}

fn find_digit(line: Vec<char>, digit_words: &Vec<(String, usize)>) -> u32 {
    for i in 0..line.len() {
        let chr = line[i];
        if chr.is_numeric() {
            return chr.to_digit(10).unwrap();
        }

        for word in digit_words {
            let mut found = true;
            for (j, word_char) in word.0.chars().enumerate() {
                if line[i + j] != word_char {
                    found = false;
                    break;
                }
            }

            if found {
                return word.1.try_into().unwrap();
            }
        }
    }

    return 0;
}

#[cfg(test)]

mod tests {
    use crate::callibration_number;

    #[test]
    fn callibration_number_test() {
        assert_eq!(callibration_number("12"), 12);
        assert_eq!(callibration_number("zero4"), 4);
        assert_eq!(callibration_number("one"), 11);
        assert_eq!(callibration_number("twothree"), 23);
        assert_eq!(callibration_number("fourfive"), 45);
        assert_eq!(callibration_number("xdssix1df2fds3sevenasd"), 67);
        assert_eq!(callibration_number("xxxnineightxxx"), 98);
        assert_eq!(callibration_number("p7oneasd23asdftwo5dsf"), 75);
        assert_eq!(callibration_number("6fivefourthreezero"), 60);
    }
}
