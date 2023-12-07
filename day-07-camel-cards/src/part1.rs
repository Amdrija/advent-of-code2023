use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
struct CardParsingError;

impl FromStr for Card {
    type Err = CardParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(CardParsingError),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    strength: Strength,
}

impl Hand {
    fn get_strength(cards: &[Card; 5]) -> Strength {
        let counts = cards.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            return map;
        });

        return match counts.len() {
            1 => Strength::FiveKind,
            2 => {
                let max_counts = *counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
                if max_counts == 4 {
                    return Strength::FourKind;
                }

                return Strength::FullHouse;
            }
            3 => {
                let max_counts = *counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
                if max_counts == 3 {
                    return Strength::ThreeKind;
                }

                return Strength::TwoPair;
            }
            4 => Strength::OnePair,
            5 => Strength::HighCard,
            _ => panic!("An array of 5 elements converted to a map must have 1 to 5 keys!"),
        };
    }

    fn new(cards: [Card; 5], bid: u64) -> Hand {
        let strength = Hand::get_strength(&cards);

        return Hand {
            cards,
            bid,
            strength,
        };
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return match self.strength.cmp(&other.strength) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => {
                for (card_self, card_other) in self.cards.iter().zip(&other.cards) {
                    match card_self.cmp(card_other) {
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Equal => (),
                        Ordering::Greater => return Some(Ordering::Greater),
                    }
                }
                Some(Ordering::Equal)
            }
            Ordering::Greater => Some(Ordering::Greater),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

#[derive(Debug)]
struct HandParsingError;

impl FromStr for Hand {
    type Err = HandParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        let cards: Vec<Card> = split
            .next()
            .ok_or(HandParsingError)?
            .chars()
            .map(|card| {
                card.to_string()
                    .parse::<Card>()
                    .map_err(|_| HandParsingError)
            })
            .collect::<Result<Vec<Card>, HandParsingError>>()?;

        let bid: u64 = split
            .next()
            .ok_or(HandParsingError)?
            .parse()
            .map_err(|_| HandParsingError)?;

        let cards = cards.try_into().map_err(|_| HandParsingError)?;

        return Ok(Hand::new(cards, bid));
    }
}

fn main() {
    let file = File::open("input").unwrap();

    let mut hands: Vec<Hand> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<Hand>().unwrap())
        .collect();

    hands.sort();

    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1) * hand.bid)
            .sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{Card, Hand, Strength};

    #[test]
    fn test_get_strength() {
        assert_eq!(
            Hand::get_strength(&[Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
            Strength::FiveKind
        );
        assert_eq!(
            Hand::get_strength(&[Card::Nine, Card::Nine, Card::Eight, Card::Nine, Card::Nine]),
            Strength::FourKind
        );
        assert_eq!(
            Hand::get_strength(&[Card::Two, Card::Three, Card::Three, Card::Two, Card::Three]),
            Strength::FullHouse
        );
        assert_eq!(
            Hand::get_strength(&[Card::Ten, Card::Nine, Card::Ten, Card::Eight, Card::Ten]),
            Strength::ThreeKind
        );
        assert_eq!(
            Hand::get_strength(&[Card::Two, Card::Three, Card::Jack, Card::Three, Card::Two]),
            Strength::TwoPair
        );
        assert_eq!(
            Hand::get_strength(&[Card::Queen, Card::Two, Card::Three, Card::Queen, Card::King]),
            Strength::OnePair
        );
        assert_eq!(
            Hand::get_strength(&[Card::Six, Card::Three, Card::Four, Card::Five, Card::King]),
            Strength::HighCard
        );
    }

    #[test]
    fn test_hand_order() {
        // A five kind of the same card is equal
        assert_eq!(
            Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace], 100).cmp(
                &Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace], 150)
            ),
            Ordering::Equal
        );

        // A hand with a five kind of a greater first card beats a hand of a five kind
        // of smaller first card
        assert_eq!(
            Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace], 100).cmp(
                &Hand::new(
                    [Card::Nine, Card::Nine, Card::Nine, Card::Nine, Card::Nine],
                    150
                )
            ),
            Ordering::Greater
        );

        //A five kind beats a four kind
        assert_eq!(
            Hand::new(
                [Card::Nine, Card::Nine, Card::Nine, Card::Nine, Card::Nine],
                100
            )
            .cmp(&Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Nine, Card::Nine],
                150
            )),
            Ordering::Greater
        );

        // A full house loses to a four kind
        assert_eq!(
            Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Eight, Card::Nine],
                100
            )
            .cmp(&Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Nine, Card::Nine],
                150
            )),
            Ordering::Less
        );

        // A three kind loses to a full house
        assert_eq!(
            Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Seven, Card::Nine],
                100
            )
            .cmp(&Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Eight, Card::Nine],
                150
            )),
            Ordering::Less
        );

        // A three kind beats a two pair
        assert_eq!(
            Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Seven, Card::Nine],
                100
            )
            .cmp(&Hand::new(
                [Card::Ace, Card::Two, Card::Eight, Card::Eight, Card::Ace],
                150
            )),
            Ordering::Greater
        );

        // A one pair loses to a two pair
        assert_eq!(
            Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Ten, Card::Jack],
                100
            )
            .cmp(&Hand::new(
                [Card::Ace, Card::Two, Card::Eight, Card::Eight, Card::Ace],
                150
            )),
            Ordering::Less
        );

        // A one pair beats a high card
        assert_eq!(
            Hand::new(
                [Card::Nine, Card::Nine, Card::Eight, Card::Ten, Card::Jack],
                100
            )
            .cmp(&Hand::new(
                [Card::Ace, Card::Two, Card::Eight, Card::Three, Card::King],
                150
            )),
            Ordering::Greater
        );

        // A high card with a higher 5th card beats a high card with a lower 5th card
        assert_eq!(
            Hand::new(
                [Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten],
                100
            )
            .cmp(&Hand::new(
                [Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Nine],
                150
            )),
            Ordering::Greater
        );

        // Two hands are equal if they have equal cards in equal orders
        assert_eq!(
            Hand::new(
                [Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten],
                100
            )
            .cmp(&Hand::new(
                [Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten],
                150
            )),
            Ordering::Equal
        );
    }

    #[test]
    fn test_hand_parsing() {
        let hand = "32T4K 765".parse::<Hand>();
        assert!(hand.is_ok());
        assert_eq!(
            hand.unwrap(),
            Hand::new(
                [Card::Three, Card::Two, Card::Ten, Card::Four, Card::King],
                765
            )
        );

        let hand = "75896 134".parse::<Hand>();
        assert!(hand.is_ok());
        assert_eq!(
            hand.unwrap(),
            Hand::new(
                [Card::Seven, Card::Five, Card::Eight, Card::Nine, Card::Six],
                134
            )
        );

        let hand = "JAQJA 666".parse::<Hand>();
        assert!(hand.is_ok());
        assert_eq!(
            hand.unwrap(),
            Hand::new(
                [Card::Jack, Card::Ace, Card::Queen, Card::Jack, Card::Ace],
                666
            )
        );
    }
}
