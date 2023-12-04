use std::{
    cmp::min,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
    u32,
};

struct Card {
    winning_numbers: Vec<u32>,
    scratched: Vec<u32>,
}

impl Card {
    fn new(winning_numbers: Vec<u32>, scratched: Vec<u32>) -> Card {
        return Card {
            winning_numbers,
            scratched,
        };
    }

    fn get_points(&self) -> u32 {
        let winning_count = self.winning_count();

        if winning_count == 0 {
            return 0;
        }

        return 1 << (winning_count - 1);
    }

    fn winning_count(&self) -> usize {
        let set: HashSet<u32> = self
            .winning_numbers
            .iter()
            .fold(HashSet::new(), |mut set, n| {
                set.insert(*n);
                set
            });

        return self.scratched.iter().filter(|n| set.contains(n)).count();
    }
}

#[derive(Debug)]
struct CardParsingError;

impl FromStr for Card {
    type Err = CardParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(": ").nth(1).ok_or(CardParsingError)?.split(" | ");

        let winning_numbers: Vec<u32> = splits
            .next()
            .ok_or(CardParsingError)?
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .map_err(|_| CardParsingError)?;

        let scratched: Vec<u32> = splits
            .next()
            .ok_or(CardParsingError)?
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .map_err(|_| CardParsingError)?;

        return Ok(Card::new(winning_numbers, scratched));
    }
}

struct CardInstances<'a> {
    card: &'a Card,
    count: usize,
}

fn get_won_cards(cards: &Vec<Card>) -> usize {
    let mut instances: Vec<CardInstances> = cards
        .iter()
        .map(|card| CardInstances { card, count: 1 })
        .collect();

    let mut won_count = 0;
    for i in 0..instances.len() {
        won_count += instances[i].count;
        for j in i + 1..min(i + 1 + instances[i].card.winning_count(), instances.len()) {
            instances[j].count += instances[i].count;
        }
    }

    return won_count;
}

fn parse_cards(buf: impl BufRead) -> Vec<Card> {
    return buf
        .lines()
        .map(|line| line.map_err(|_| CardParsingError)?.parse::<Card>())
        .collect::<Result<Vec<Card>, CardParsingError>>()
        .unwrap();
}

fn main() {
    let file = File::open("input").unwrap();

    let cards = parse_cards(BufReader::new(file));
    let sum = cards
        .iter()
        .map(|c| c.get_points())
        .fold(0, |sum, points| sum + points);

    println!("{}", sum);

    println!("{}", get_won_cards(&cards));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::{get_won_cards, parse_cards, Card};

    const CARD: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    const CARDS: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn test_winning_count() {
        let card: Card = CARD.parse().unwrap();
        assert_eq!(card.winning_count(), 4);
    }

    #[test]
    fn test_parse() {
        let card: Card = CARD.parse().unwrap();
        let expected_card = Card::new(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]);

        assert_eq!(
            card.winning_numbers.len(),
            expected_card.winning_numbers.len()
        );
        assert!(card
            .winning_numbers
            .iter()
            .all(|wn| expected_card.winning_numbers.contains(wn)));
        assert!(expected_card
            .winning_numbers
            .iter()
            .all(|wn| card.winning_numbers.contains(wn)));

        assert_eq!(
            card.winning_numbers.len(),
            expected_card.winning_numbers.len()
        );
        assert!(card
            .scratched
            .iter()
            .all(|wn| expected_card.scratched.contains(wn)));
        assert!(expected_card
            .scratched
            .iter()
            .all(|wn| card.scratched.contains(wn)));
    }

    #[test]
    fn test_get_points() {
        assert_eq!(
            Card::new(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]).get_points(),
            8
        );

        assert_eq!(
            Card::new(
                vec![13, 32, 20, 16, 61],
                vec![61, 30, 68, 82, 17, 32, 24, 19]
            )
            .get_points(),
            2
        );

        assert_eq!(
            Card::new(
                vec![41, 92, 73, 84, 69],
                vec![59, 84, 76, 51, 58, 5, 54, 83]
            )
            .get_points(),
            1
        );

        assert_eq!(
            Card::new(
                vec![31, 18, 13, 56, 72],
                vec![74, 77, 10, 23, 35, 67, 36, 11]
            )
            .get_points(),
            0
        );
    }

    #[test]
    fn test_get_won_counts() {
        let cards = parse_cards(BufReader::new(CARDS.as_bytes()));

        assert_eq!(get_won_cards(&cards), 30);
    }
}
