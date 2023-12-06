use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_ways(&self) -> u64 {
        let time = self.time as i128;
        let distance = self.distance as i128;
        let determinant: i128 = time * time - 4 * distance;

        //because the equation is
        // x^2 - time * x + distance <= 0
        // if the determinant is less than 0,
        // it means that the quadratic function
        // doesn't intersect the x-axis
        // meaning that the function value
        // is always greater than 0
        if determinant < 0 {
            return 0;
        }

        let lower_bound = (((self.time as f64) - (determinant as f64).sqrt()) / 2.0).ceil() as u64;
        let upper_bound = (((self.time as f64) + (determinant as f64).sqrt()) / 2.0).floor() as u64;

        return upper_bound - lower_bound + 1;
    }
}

#[derive(Debug)]
struct ParseRaceError;

fn parse_races(s: &str) -> Result<Vec<Race>, ParseRaceError> {
    let mut lines = s.lines();
    let times = lines
        .next()
        .ok_or(ParseRaceError)?
        .strip_prefix("Time:")
        .ok_or(ParseRaceError)?
        .trim_start()
        .split_ascii_whitespace()
        .map(|time| time.parse::<u64>().map_err(|_| ParseRaceError));

    let distances = lines
        .next()
        .ok_or(ParseRaceError)?
        .strip_prefix("Distance:")
        .ok_or(ParseRaceError)?
        .trim_start()
        .split_ascii_whitespace()
        .map(|time| time.parse::<u64>().map_err(|_| ParseRaceError));

    return times
        .zip(distances)
        .map(|(time, distance)| {
            if time.is_err() || distance.is_err() {
                return Err(ParseRaceError);
            }

            return Ok(Race {
                time: time.unwrap(),
                distance: distance.unwrap(),
            });
        })
        .collect();
}

fn parse_race(s: &str) -> Result<Race, ParseRaceError> {
    let mut lines = s.lines();
    let times = lines
        .next()
        .ok_or(ParseRaceError)?
        .strip_prefix("Time:")
        .ok_or(ParseRaceError)?
        .trim_start()
        .split_ascii_whitespace()
        .fold(String::from(""), |fstr, time| fstr + time);

    let distances = lines
        .next()
        .ok_or(ParseRaceError)?
        .strip_prefix("Distance:")
        .ok_or(ParseRaceError)?
        .trim_start()
        .split_ascii_whitespace()
        .fold(String::from(""), |fstr, distance| fstr + distance);

    let time = times.parse::<u64>().map_err(|_| ParseRaceError)?;
    let distance = distances.parse::<u64>().map_err(|_| ParseRaceError)?;

    return Ok(Race { time, distance });
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let races = parse_races(&input).unwrap();

    println!(
        "{}",
        races.iter().map(|race| race.winning_ways()).sum::<u64>()
    );

    let race = parse_race(&input).unwrap();
    println!("{}", race.winning_ways());
}

#[cfg(test)]
mod tests {
    use crate::{parse_race, parse_races, Race};

    const RACES: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse_races() {
        let races = parse_races(RACES);

        assert!(races.is_ok());

        let races = races.unwrap();
        assert_eq!(
            races,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ]
        );
    }

    #[test]
    fn test_winning_ways() {
        assert_eq!(
            Race {
                time: 7,
                distance: 9
            }
            .winning_ways(),
            4
        );

        assert_eq!(
            Race {
                time: 6,
                distance: 9
            }
            .winning_ways(),
            1
        );

        assert_eq!(
            Race {
                time: 6,
                distance: 10
            }
            .winning_ways(),
            0
        );
    }

    #[test]
    fn test_parse_race() {
        let race = parse_race(RACES);

        assert!(race.is_ok());

        let race = race.unwrap();

        assert_eq!(
            race,
            Race {
                time: 71530,
                distance: 940200
            }
        );
    }
}
