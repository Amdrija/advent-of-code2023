use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    u32,
};

#[derive(Debug, PartialEq, Eq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn new(red: u32, green: u32, blue: u32) -> Draw {
        return Draw { red, green, blue };
    }

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        return self.red <= red && self.green <= green && self.blue <= blue;
    }

    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

#[derive(Debug)]
struct DrawParsingError;

impl FromStr for Draw {
    type Err = DrawParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for color in s.split(", ") {
            let mut split = color.split(" ");
            let number = split
                .next()
                .unwrap()
                .parse::<u32>()
                .or(Err(DrawParsingError))?;

            let color_str = split.next().ok_or(DrawParsingError)?;

            if color_str.contains("red") {
                red = number;
            } else if color_str.contains("blue") {
                blue = number;
            } else if color_str.contains("green") {
                green = number;
            } else {
                return Err(DrawParsingError);
            }
        }

        return Ok(Draw::new(red, green, blue));
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: u32, draws: Vec<Draw>) -> Game {
        return Game { id, draws };
    }

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        return self
            .draws
            .iter()
            .all(|draw| draw.is_possible(red, green, blue));
    }

    fn get_minimum(&self) -> Draw {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for draw in &self.draws {
            red_max = max(red_max, draw.red);
            green_max = max(green_max, draw.green);
            blue_max = max(blue_max, draw.blue);
        }

        return Draw::new(red_max, green_max, blue_max);
    }
}

#[derive(Debug)]
struct GameParsingError;

impl FromStr for Game {
    type Err = GameParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");

        let id = split
            .next()
            .unwrap()
            .split("Game ")
            .nth(1)
            .ok_or(GameParsingError)?
            .parse::<u32>()
            .map_err(|_| GameParsingError)?;

        let draws: Vec<Draw> = split
            .next()
            .ok_or(GameParsingError)?
            .split("; ")
            .map(|str| str.parse::<Draw>())
            .collect::<Result<Vec<Draw>, DrawParsingError>>()
            .map_err(|_| GameParsingError)?;

        return Ok(Game::new(id, draws));
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();
    let games: Vec<Game> = lines
        .map(|line| line.unwrap().parse::<Game>().unwrap())
        .collect();

    let sum_ids = games
        .iter()
        .filter(|g| g.is_possible(12, 13, 14))
        .fold(0, |sum, game| sum + game.id);

    println!("{}", sum_ids);

    let sum_power_mins = games
        .iter()
        .fold(0, |sum, game| sum + game.get_minimum().power());

    println!("{}", sum_power_mins);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{Draw, Game};

    #[test]
    fn test_draws() {
        assert!(Draw::new(5, 6, 7).is_possible(10, 10, 10));
        assert!(Draw::new(0, 0, 0).is_possible(0, 0, 0));
        assert!(!Draw::new(1, 5, 5).is_possible(0, 10, 10));
        assert!(!Draw::new(5, 1, 5).is_possible(10, 0, 10));
        assert!(!Draw::new(5, 5, 1).is_possible(10, 10, 0));
        assert!(!Draw::new(1, 1, 1).is_possible(0, 0, 0));
    }

    #[test]
    fn test_games() {
        assert!(Game::new(1, vec![Draw::new(5, 6, 7), Draw::new(8, 8, 8)]).is_possible(9, 9, 9));
        assert!(!Game::new(
            1,
            vec![
                Draw::new(0, 0, 0),
                Draw::new(2, 1, 0),
                Draw::new(3, 3, 3),
                Draw::new(5, 6, 8)
            ]
        )
        .is_possible(3, 3, 3));
    }

    #[test]
    fn test_parse_draw() {
        assert_eq!(
            "7 green, 4 blue, 3 red".parse::<Draw>().unwrap(),
            Draw::new(3, 7, 4)
        );
        assert_eq!("23 red".parse::<Draw>().unwrap(), Draw::new(23, 0, 0));
        assert_eq!("887 green".parse::<Draw>().unwrap(), Draw::new(0, 887, 0));
        assert_eq!("1 blue".parse::<Draw>().unwrap(), Draw::new(0, 0, 1));

        assert!("1 blu2e".parse::<Draw>().is_err());
        assert!("".parse::<Draw>().is_err());
        assert!("blue".parse::<Draw>().is_err());
    }

    #[test]
    fn test_draw_power() {
        assert_eq!(Draw::new(2, 5, 8).power(), 80);
    }

    #[test]
    fn test_game_min() {
        assert_eq!(
            Game::new(
                1,
                vec![Draw::new(20, 8, 6), Draw::new(4, 13, 5), Draw::new(1, 5, 0)]
            )
            .get_minimum(),
            Draw::new(20, 13, 6)
        )
    }
}
