use std::{
    fs::{self, File},
    io::{BufRead, Read},
    num::ParseIntError,
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug)]
struct IntervalMapping {
    source: u64,
    destination: u64,
    length: u64,
}

impl IntervalMapping {
    fn get_mapping(&self, source: u64) -> Option<u64> {
        if source >= self.source && source < self.source + self.length {
            return Some(self.destination + source - self.source);
        }

        return None;
    }
}

#[derive(Debug)]
struct IntervalParsingError;

impl FromStr for IntervalMapping {
    type Err = IntervalParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_ascii_whitespace()
            .map(|part| part.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()
            .map_err(|_| IntervalParsingError)?;

        if values.len() != 3 {
            return Err(IntervalParsingError);
        }

        return Ok(IntervalMapping {
            destination: values[0],
            source: values[1],
            length: values[2],
        });
    }
}

fn parse_mapping_set(s: &str) -> Result<Vec<IntervalMapping>, IntervalParsingError> {
    return s.lines().map(|line| line.parse()).collect();
}

fn extract_part<'a>(s: &'a str, name: &str) -> Result<&'a str, IntervalParsingError> {
    return s
        .split(name)
        .nth(1)
        .ok_or(IntervalParsingError)?
        .split("\n\n")
        .nth(0)
        .ok_or(IntervalParsingError);
}

trait Mapping {
    fn get_mapping(&self, source: u64) -> u64;
}

impl Mapping for Vec<IntervalMapping> {
    fn get_mapping(&self, source: u64) -> u64 {
        return self
            .iter()
            .filter_map(|int| int.get_mapping(source))
            .next()
            .unwrap_or(source);
    }
}

struct Mappings {
    seed_to_soil: Box<dyn Mapping>,
    soil_to_fertilizer: Box<dyn Mapping>,
    fertilizer_to_water: Box<dyn Mapping>,
    water_to_light: Box<dyn Mapping>,
    light_to_temperature: Box<dyn Mapping>,
    temperature_to_humidity: Box<dyn Mapping>,
    humidity_to_location: Box<dyn Mapping>,
}

impl Mappings {
    fn get_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get_mapping(seed);
        let fertilizer = self.soil_to_fertilizer.get_mapping(soil);
        let water = self.fertilizer_to_water.get_mapping(fertilizer);
        let light = self.water_to_light.get_mapping(water);
        let temperature = self.light_to_temperature.get_mapping(light);
        let humidity = self.temperature_to_humidity.get_mapping(temperature);

        return self.humidity_to_location.get_mapping(humidity);
    }
}

impl FromStr for Mappings {
    type Err = IntervalParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Mappings {
            seed_to_soil: Box::new(parse_mapping_set(extract_part(s, "seed-to-soil map:\n")?)?),
            soil_to_fertilizer: Box::new(parse_mapping_set(extract_part(
                s,
                "soil-to-fertilizer map:\n",
            )?)?),
            fertilizer_to_water: Box::new(parse_mapping_set(extract_part(
                s,
                "fertilizer-to-water map:\n",
            )?)?),
            water_to_light: Box::new(parse_mapping_set(extract_part(
                s,
                "water-to-light map:\n",
            )?)?),
            light_to_temperature: Box::new(parse_mapping_set(extract_part(
                s,
                "light-to-temperature map:\n",
            )?)?),
            temperature_to_humidity: Box::new(parse_mapping_set(extract_part(
                s,
                "temperature-to-humidity map:\n",
            )?)?),
            humidity_to_location: Box::new(parse_mapping_set(extract_part(
                s,
                "humidity-to-location map:\n",
            )?)?),
        });
    }
}

fn parse_seeds(input: &str) -> Result<Vec<u64>, ParseIntError> {
    return input
        .split("\n\n")
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>())
        .collect();
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let seeds: Vec<u64> = parse_seeds(&input).unwrap();
    println!("{:?}", seeds);
    let mappings: Mappings = input.parse().unwrap();

    println!(
        "{}",
        seeds
            .iter()
            .map(|seed| mappings.get_location(*seed))
            .min()
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use crate::{parse_seeds, IntervalMapping, Mapping, Mappings};

    const MAPPINGS: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn test_interval_parsing() {
        let int: IntervalMapping = "50 98 2".parse().unwrap();

        assert_eq!(
            int,
            IntervalMapping {
                destination: 50,
                source: 98,
                length: 2
            }
        );
    }

    #[test]
    fn test_interval_mapping() {
        let int = IntervalMapping {
            destination: 50,
            source: 98,
            length: 2,
        };
        assert_eq!(int.get_mapping(98).unwrap(), 50);
        assert_eq!(int.get_mapping(99).unwrap(), 51);
        assert!(int.get_mapping(100).is_none());
        assert!(int.get_mapping(97).is_none());
    }

    #[test]
    fn test_vec_interval_mapping() {
        //         50 98 2
        // 52 50 48
        let mappings = vec![
            IntervalMapping {
                destination: 50,
                source: 98,
                length: 2,
            },
            IntervalMapping {
                destination: 52,
                source: 50,
                length: 48,
            },
        ];

        assert_eq!(mappings.get_mapping(98), 50);
        assert_eq!(mappings.get_mapping(99), 51);
        assert_eq!(mappings.get_mapping(100), 100);
        assert_eq!(mappings.get_mapping(50), 52);
        assert_eq!(mappings.get_mapping(55), 57);
        assert_eq!(mappings.get_mapping(97), 99);
        assert_eq!(mappings.get_mapping(49), 49);
        assert_eq!(mappings.get_mapping(10), 10);
    }

    #[test]
    fn test_parse_seeds() {
        let seeds = parse_seeds(MAPPINGS);

        assert!(seeds.is_ok());
        //Here I guess we don't really need to test the ordering
        //but I cannot be bothered to write the test that compares
        //the vectors as sets
        assert_eq!(seeds.unwrap(), vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_mappings() {
        let mappings: Result<Mappings, _> = MAPPINGS.parse();

        assert!(mappings.is_ok());

        let mappings = mappings.unwrap();

        let seeds: Vec<u64> = vec![79, 14, 55, 13];

        assert_eq!(
            seeds
                .iter()
                .map(|seed| mappings.get_location(*seed))
                .collect::<Vec<u64>>(),
            vec![82, 43, 86, 35]
        );
    }
}
