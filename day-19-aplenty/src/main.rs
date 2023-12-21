use std::{collections::HashMap, fs};

enum Order {
    Less,
    Greater,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn parse(s: &str) -> Self {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => panic!("Unknown category: {}", s),
        }
    }
}

enum Destination {
    Accepted,
    Rejected,
    Workflow(String),
}

impl Destination {
    fn parse(s: &str) -> Self {
        match s {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            _ => Destination::Workflow(s.to_string()),
        }
    }
}

struct Rule {
    category: Category,
    order: Order,
    threshold: i64,
    destination: Destination,
}

impl Rule {
    fn parse(s: &str) -> Self {
        let order;
        let mut order_split;
        if s.contains("<") {
            order = Order::Less;
            order_split = s.split("<");
        } else if s.contains(">") {
            order = Order::Greater;
            order_split = s.split(">");
        } else {
            panic!("Cannot parse rule: {}", s);
        }

        let category = Category::parse(order_split.next().unwrap());

        let mut colon_split = order_split.next().unwrap().split(":");
        let threshold = colon_split.next().unwrap().parse().unwrap();
        let destination = Destination::parse(colon_split.next().unwrap());

        Self {
            category,
            order,
            threshold,
            destination,
        }
    }

    fn satisfies(&self, part: &Part) -> bool {
        let value = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };

        match self.order {
            Order::Less => value < self.threshold,
            Order::Greater => value > self.threshold,
        }
    }

    fn split_interval(&self, part: &IntervalPart) -> (Option<IntervalPart>, Option<IntervalPart>) {
        match self.order {
            Order::Less => {
                let (lower, higher) = part[&self.category].clone().split(self.threshold - 1);

                let accepted = lower.map(|lower| new_interval_part(part, lower, &self.category));
                let rejected: Option<HashMap<Category, Interval>> =
                    higher.map(|higher| new_interval_part(part, higher, &self.category));

                return (rejected, accepted);
            }
            Order::Greater => {
                let (lower, higher) = part[&self.category].clone().split(self.threshold);

                let accepted = higher.map(|higher| new_interval_part(part, higher, &self.category));
                let rejected: Option<HashMap<Category, Interval>> =
                    lower.map(|lower| new_interval_part(part, lower, &self.category));

                return (rejected, accepted);
            }
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
    final_destination: Destination,
}

impl Workflow {
    fn parse(s: &str) -> Self {
        let split = s.split(",").collect::<Vec<_>>();
        let (last, rules) = split.split_last().unwrap();

        Self {
            rules: rules.iter().map(|rule| Rule::parse(&rule)).collect(),
            final_destination: Destination::parse(&last),
        }
    }

    fn process_part<'a>(&'a self, part: &Part) -> &'a Destination {
        for rule in &self.rules {
            if rule.satisfies(&part) {
                return &rule.destination;
            }
        }

        &self.final_destination
    }

    fn process_interval<'a>(
        &'a self,
        mut part: IntervalPart,
    ) -> Vec<(IntervalPart, &'a Destination)> {
        let mut result = Vec::new();
        let mut all_satisfied = false;

        for rule in &self.rules {
            let (rejected, accepted) = rule.split_interval(&part);

            if let Some(accepted) = accepted {
                result.push((accepted, &rule.destination));
            }

            if let Some(rejected) = rejected {
                part = rejected;
            } else {
                all_satisfied = true;
                break;
            }
        }

        if !all_satisfied {
            result.push((part, &self.final_destination));
        }

        result
    }
}

struct WorkflowMap {
    map: HashMap<String, Workflow>,
    accepted: Vec<Part>,
    rejected: Vec<Part>,
    accepted_intervals: Vec<IntervalPart>,
}

impl WorkflowMap {
    fn parse(s: &str) -> Self {
        let mut result = WorkflowMap {
            map: HashMap::new(),
            accepted: Vec::new(),
            rejected: Vec::new(),
            accepted_intervals: Vec::new(),
        };

        for line in s.lines() {
            let mut split = line.split("{");
            let name = split.next().unwrap();
            let workflow = split.next().unwrap().strip_suffix("}").unwrap();

            result
                .map
                .insert(name.to_string(), Workflow::parse(workflow));
        }

        return result;
    }

    fn process_part(&mut self, part: Part) {
        let mut workflow = &self.map["in"];

        while let Destination::Workflow(workflow_name) = workflow.process_part(&part) {
            workflow = &self.map[workflow_name];
        }

        match workflow.process_part(&part) {
            Destination::Accepted => self.accepted.push(part),
            Destination::Rejected => self.rejected.push(part),
            Destination::Workflow(_) => (),
        };
    }

    fn process_interval(&mut self, part: IntervalPart) {
        let mut stack = vec![(part, &self.map["in"])];

        while let Some((part, workflow)) = stack.pop() {
            for (new_part, new_destination) in workflow.process_interval(part) {
                match new_destination {
                    Destination::Accepted => self.accepted_intervals.push(new_part),
                    Destination::Rejected => (),
                    Destination::Workflow(name) => stack.push((new_part, &self.map[name])),
                }
            }
        }
    }

    fn sum_accepted(&self) -> i64 {
        self.accepted
            .iter()
            .map(|part| part.x + part.m + part.a + part.s)
            .sum()
    }

    fn combinations(&self) -> i64 {
        self.accepted_intervals
            .iter()
            .map(|part| {
                part[&Category::X].values()
                    * part[&Category::M].values()
                    * part[&Category::A].values()
                    * part[&Category::S].values()
            })
            .sum()
    }
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn parse_value(s: &str, category_name: &str) -> i64 {
        s.strip_prefix(category_name).unwrap().parse().unwrap()
    }

    fn parse(s: &str) -> Self {
        let s = s.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
        let mut split = s.split(",");

        Self {
            x: Part::parse_value(split.next().unwrap(), "x="),
            m: Part::parse_value(split.next().unwrap(), "m="),
            a: Part::parse_value(split.next().unwrap(), "a="),
            s: Part::parse_value(split.next().unwrap(), "s="),
        }
    }
}

fn parse_parts(s: &str) -> Vec<Part> {
    s.lines().map(|line| Part::parse(line)).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Self {
        return Interval { start, end };
    }

    fn split(self, mid: i64) -> (Option<Interval>, Option<Interval>) {
        if self.start > mid {
            return (None, Some(self));
        }

        if self.end <= mid {
            return (Some(self), None);
        }

        return (
            Some(Interval::new(self.start, mid)),
            Some(Interval::new(mid + 1, self.end)),
        );
    }

    fn values(&self) -> i64 {
        return self.end - self.start + 1;
    }
}

type IntervalPart = HashMap<Category, Interval>;

fn new_interval_part(part: &IntervalPart, interval: Interval, category: &Category) -> IntervalPart {
    let mut result = HashMap::new();

    for (key, value) in part {
        result.insert(key.clone(), value.clone());
    }

    result.insert(category.clone(), interval);

    return result;
}

//167409079868000

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut split = input.split("\n\n");
    let mut wmap = WorkflowMap::parse(split.next().unwrap());
    let parts = parse_parts(split.next().unwrap());

    for part in parts {
        wmap.process_part(part);
    }

    let mut interval_part = IntervalPart::new();
    interval_part.insert(Category::X, Interval::new(1, 4000));
    interval_part.insert(Category::M, Interval::new(1, 4000));
    interval_part.insert(Category::A, Interval::new(1, 4000));
    interval_part.insert(Category::S, Interval::new(1, 4000));

    wmap.process_interval(interval_part);

    println!("{}", wmap.combinations());
}

#[cfg(test)]
mod tests {
    use crate::Interval;

    #[test]
    fn split_interval() {
        let i = Interval::new(5, 10);

        assert_eq!(
            i.clone().split(7),
            (Some(Interval::new(5, 7)), Some(Interval::new(8, 10)))
        );

        assert_eq!(
            i.clone().split(5),
            (Some(Interval::new(5, 5)), Some(Interval::new(6, 10)))
        );

        assert_eq!(
            i.clone().split(9),
            (Some(Interval::new(5, 9)), Some(Interval::new(10, 10)))
        );

        assert_eq!(i.clone().split(4), (None, Some(Interval::new(5, 10))));

        assert_eq!(i.clone().split(10), (Some(Interval::new(5, 10)), None));
    }
}
