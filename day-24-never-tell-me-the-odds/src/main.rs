use std::fs;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn parse(s: &str) -> Self {
        let split = s.split(", ").collect::<Vec<_>>();

        Point {
            x: split[0].trim().parse::<i64>().unwrap() as f64,
            y: split[1].trim().parse::<i64>().unwrap() as f64,
            z: split[2].trim().parse::<i64>().unwrap() as f64,
        }
    }
}

struct Hail {
    position: Point,
    velocity: Point,
}

impl Hail {
    fn parse(s: &str) -> Self {
        let mut split = s.split(" @ ");

        Self {
            position: Point::parse(split.next().unwrap()),
            velocity: Point::parse(split.next().unwrap()),
        }
    }

    fn intersect(&self, other: &Hail) -> Option<(f64, f64)> {
        let determinant = self.velocity.y * other.velocity.x - self.velocity.x * other.velocity.y;
        if determinant == 0. {
            return None;
        }

        let b1 = other.position.x - self.position.x;
        let b2 = other.position.y - self.position.y;

        let t1 = (other.velocity.x * b2 - other.velocity.y * b1) / determinant;
        let t2 = (self.velocity.x * b2 - self.velocity.y * b1) / determinant;

        if t1 < 0. || t2 < 0. {
            return None;
        }

        return Some((
            self.position.x + t1 * self.velocity.x,
            self.position.y + t1 * self.velocity.y,
        ));
    }
}

fn count_intersect(hails: &Vec<Hail>, min: f64, max: f64) -> u64 {
    let mut count = 0;
    for i in 0..hails.len() - 1 {
        for j in i + 1..hails.len() {
            if let Some((x, y)) = hails[i].intersect(&hails[j]) {
                if x >= min && x <= max && y >= min && y <= max {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let hails = input
        .lines()
        .map(|line| Hail::parse(&line))
        .collect::<Vec<_>>();

    println!(
        "{}",
        count_intersect(&hails, 200000000000000., 400000000000000.)
    );

    //Part 2 solved by hand (using Octave)
    //In general, it is quite easy,
    //We can just pick any 3 points and try to solve the linear equation
    //Set up the 9 linear equations which and try to solve them
    //If the solution is unique then we solved the problem, which was
    //the case for the first 3 points. Otherwise, we could try one
    //of the infinite number of solutions and if that doesn't work
    //it must be that the solution is unique. Then, it boils down
    //to just finding points which would gives us an independent
    //system of 9 equations.
    // In my case I just used Octave online and did this with
    //the symbolic package.
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{count_intersect, Hail, Point};

    #[test]
    fn intersect() {
        let hail1 = Hail {
            position: Point {
                x: 20.,
                y: 25.,
                z: 34.,
            },
            velocity: Point {
                x: -2.,
                y: -2.,
                z: -4.,
            },
        };

        let hail2 = Hail {
            position: Point {
                x: 12.,
                y: 31.,
                z: 28.,
            },
            velocity: Point {
                x: -1.,
                y: -2.,
                z: -1.,
            },
        };

        let intersect = hail1.intersect(&hail2);

        assert!(intersect.is_some());
        assert_eq!(intersect.unwrap(), (-2., 3.));

        let hail3 = Hail {
            position: Point {
                x: 18.,
                y: 19.,
                z: 22.,
            },
            velocity: Point {
                x: -1.,
                y: -1.,
                z: -2.,
            },
        };

        //These two never intersect
        let intersect = hail1.intersect(&hail3);
        assert!(intersect.is_none());

        let hail4 = Hail {
            position: Point {
                x: 20.,
                y: 19.,
                z: 15.,
            },
            velocity: Point {
                x: 1.,
                y: -5.,
                z: 3.,
            },
        };
        //These two intersect in the past for both
        let intersect = hail3.intersect(&hail4);
        assert!(intersect.is_none());

        // These two intersect in the past for hail4
        let intersect = hail1.intersect(&hail4);
        assert!(intersect.is_none());

        let hail5 = Hail {
            position: Point {
                x: 19.,
                y: 13.,
                z: 30.,
            },
            velocity: Point {
                x: -2.,
                y: 1.,
                z: -2.,
            },
        };
        //These two intersect in the past for hail 4
        let intersect = hail4.intersect(&hail5);
        assert!(intersect.is_none())
    }

    #[test]
    fn test_count_intersect() {
        let input = fs::read_to_string("test.txt").unwrap();
        let hails = input
            .lines()
            .map(|line| Hail::parse(&line))
            .collect::<Vec<_>>();

        assert_eq!(count_intersect(&hails, 7., 27.), 2);
    }
}
