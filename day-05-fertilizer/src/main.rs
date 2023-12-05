use std::{
    fs::{self},
    num::ParseIntError,
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug)]
struct IntervalMapping {
    interval: Interval,
    offset: i64,
}

impl IntervalMapping {
    fn new(source: i64, destination: i64, length: i64) -> IntervalMapping {
        return IntervalMapping {
            interval: Interval {
                start: source,
                end: source + length - 1,
            },
            offset: destination - source,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn split(&self, point: i64) -> (Interval, Interval) {
        return (
            Interval {
                start: self.start,
                end: point,
            },
            Interval {
                start: point + 1,
                end: self.end,
            },
        );
    }
}

impl IntervalMapping {
    fn get_mapping(&self, source: i64) -> Option<i64> {
        if source >= self.interval.start && source <= self.interval.end {
            return Some(source + self.offset);
        }

        return None;
    }

    fn map_interval(&self, interval: &Interval) -> MappedInterval {
        let mut interval = interval.clone();
        let mut remaining = Vec::new();

        if interval.end < self.interval.start {
            return MappedInterval {
                mapped: None,
                remaining: vec![interval.clone()],
            };
        }

        if interval.start > self.interval.end {
            return MappedInterval {
                mapped: None,
                remaining: vec![interval.clone()],
            };
        }

        if interval.start < self.interval.start && self.interval.start <= interval.end {
            let splitted = interval.split(self.interval.start - 1);
            interval = splitted.1;
            remaining.push(splitted.0);
        }

        if interval.end > self.interval.end && self.interval.end >= interval.start {
            let splitted = interval.split(self.interval.end);
            remaining.push(splitted.1);
            interval = splitted.0;
        }

        //this has to get mapped;
        let mut mapped = None;
        if interval.start != interval.end {
            mapped = Some(Interval {
                start: interval.start + self.offset,
                end: interval.end + self.offset,
            });
        }

        return MappedInterval { mapped, remaining };
    }
}

#[derive(Debug)]
struct MappedInterval {
    mapped: Option<Interval>,
    remaining: Vec<Interval>,
}

#[derive(Debug)]
struct IntervalParsingError;

impl FromStr for IntervalMapping {
    type Err = IntervalParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_ascii_whitespace()
            .map(|part| part.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>()
            .map_err(|_| IntervalParsingError)?;

        if values.len() != 3 {
            return Err(IntervalParsingError);
        }

        return Ok(IntervalMapping::new(values[1], values[0], values[2]));
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
    fn get_mapping(&self, source: i64) -> i64;
    fn map_interval(&self, interval: &Interval) -> Vec<Interval>;
    fn map_intervals(&self, intervals: &Vec<Interval>) -> Vec<Interval>;
}

impl Mapping for Vec<IntervalMapping> {
    fn get_mapping(&self, source: i64) -> i64 {
        return self
            .iter()
            .filter_map(|int| int.get_mapping(source))
            .next()
            .unwrap_or(source);
    }

    fn map_interval(&self, interval: &Interval) -> Vec<Interval> {
        let mut result = Vec::new();
        let mut to_map = vec![interval.clone()];

        for mapping in self {
            let current_to_map = to_map.clone();
            let mut next_to_map = Vec::new();
            for int in current_to_map {
                let mut mapped = mapping.map_interval(&int);

                if let Some(mapped_interval) = mapped.mapped {
                    result.push(mapped_interval);
                }

                if !mapped.remaining.is_empty() {
                    next_to_map.append(&mut mapped.remaining);
                }
            }

            to_map = next_to_map;
        }

        result.append(&mut to_map);

        return result;
    }

    fn map_intervals(&self, intervals: &Vec<Interval>) -> Vec<Interval> {
        return intervals
            .iter()
            .map(|int| self.map_interval(int))
            .flatten()
            .collect();
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
    fn get_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.get_mapping(seed);
        let fertilizer = self.soil_to_fertilizer.get_mapping(soil);
        let water = self.fertilizer_to_water.get_mapping(fertilizer);
        let light = self.water_to_light.get_mapping(water);
        let temperature = self.light_to_temperature.get_mapping(light);
        let humidity = self.temperature_to_humidity.get_mapping(temperature);

        return self.humidity_to_location.get_mapping(humidity);
    }

    fn interval_locations(&self, interval: Interval) -> Vec<Interval> {
        let soil = self.seed_to_soil.map_interval(&interval);
        let fertilizer = self.soil_to_fertilizer.map_intervals(&soil);
        let water = self.fertilizer_to_water.map_intervals(&fertilizer);
        let light = self.water_to_light.map_intervals(&water);
        let temperature = self.light_to_temperature.map_intervals(&light);
        let humidity = self.temperature_to_humidity.map_intervals(&temperature);

        return self.humidity_to_location.map_intervals(&humidity);
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

fn parse_seeds(input: &str) -> Result<Vec<i64>, ParseIntError> {
    return input
        .split("\n\n")
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>())
        .collect();
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let seeds: Vec<i64> = parse_seeds(&input).unwrap();

    let seed_intervals: Vec<Interval> = seeds
        .chunks(2)
        .map(|chunk| Interval {
            start: chunk[0],
            end: chunk[0] + chunk[1] - 1,
        })
        .collect();
    println!("{:?}", seed_intervals);

    let mappings: Mappings = input.parse().unwrap();

    println!(
        "{}",
        seeds
            .iter()
            .map(|seed| mappings.get_location(*seed))
            .min()
            .unwrap()
    );

    let seed_intervals = seed_intervals
        .iter()
        .flat_map(|int| mappings.interval_locations(int.clone()))
        .collect::<Vec<Interval>>();
    println!("{:?}", seed_intervals);

    println!(
        "{}",
        seed_intervals.iter().map(|int| int.start).min().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use crate::{parse_seeds, Interval, IntervalMapping, Mapping, Mappings};

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

        assert_eq!(int, IntervalMapping::new(98, 50, 2));
    }

    #[test]
    fn test_mapping() {
        let int = IntervalMapping::new(98, 50, 2);
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
            IntervalMapping::new(98, 50, 2),
            IntervalMapping::new(50, 52, 48),
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

        let seeds: Vec<i64> = vec![79, 14, 55, 13];

        assert_eq!(
            seeds
                .iter()
                .map(|seed| mappings.get_location(*seed))
                .collect::<Vec<i64>>(),
            vec![82, 43, 86, 35]
        );
    }

    #[test]
    fn test_interval_mapping() {
        let mapping = IntervalMapping::new(98, 50, 2);

        let mapped = mapping.map_interval(&Interval {
            start: 95,
            end: 104,
        });

        assert!(mapped.mapped.is_some());
        assert_eq!(mapped.mapped.unwrap(), Interval { start: 50, end: 51 });
        assert_eq!(
            mapped.remaining,
            vec![
                Interval { start: 95, end: 97 },
                Interval {
                    start: 100,
                    end: 104
                },
            ]
        );

        let mapped = mapping.map_interval(&Interval {
            start: 100,
            end: 103,
        });

        assert!(mapped.mapped.is_none());
        assert_eq!(
            mapped.remaining,
            vec![Interval {
                start: 100,
                end: 103
            }]
        );

        let mapped = mapping.map_interval(&Interval { start: 98, end: 99 });

        assert!(mapped.mapped.is_some());
        assert_eq!(mapped.mapped.unwrap(), Interval { start: 50, end: 51 });
        assert!(mapped.remaining.is_empty());

        let interval = Interval {
            start: 100,
            end: 109,
        };
        let mapped = mapping.map_interval(&interval);
        assert!(mapped.mapped.is_none());
        assert_eq!(mapped.remaining, vec![interval]);
    }

    #[test]
    fn test_mapping_map_interval() {
        let mapping: Vec<IntervalMapping> = vec![
            IntervalMapping::new(98, 50, 2),
            IntervalMapping::new(50, 52, 48),
        ];
        let interval = Interval { start: 79, end: 92 };

        assert_eq!(
            mapping.map_interval(&interval),
            vec![Interval { start: 81, end: 94 }]
        );

        let interval = Interval {
            start: 40,
            end: 109,
        };

        assert_eq!(
            mapping.map_interval(&interval),
            vec![
                Interval { start: 50, end: 51 },
                Interval { start: 52, end: 99 },
                Interval { start: 40, end: 49 },
                Interval {
                    start: 100,
                    end: 109
                }
            ]
        )
    }

    #[test]
    fn get_min_location() {
        let mappings: Result<Mappings, _> = MAPPINGS.parse();

        assert!(mappings.is_ok());

        let mappings = mappings.unwrap();

        let seed_intervals: Vec<Interval> = vec![
            Interval { start: 79, end: 92 },
            Interval { start: 55, end: 67 },
        ];

        let seed_intervals = seed_intervals
            .iter()
            .flat_map(|int| mappings.interval_locations(int.clone()))
            .collect::<Vec<Interval>>();

        println!("{:?}", seed_intervals);
        assert_eq!(
            seed_intervals.iter().map(|int| int.start).min().unwrap(),
            46
        );
    }
}
