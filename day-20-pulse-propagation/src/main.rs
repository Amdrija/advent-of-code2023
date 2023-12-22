use std::{
    collections::{HashMap, VecDeque},
    fs,
    sync::Arc,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

struct Signal {
    from: String,
    to: String,
    pulse: Pulse,
}

trait Module {
    fn process(&mut self, signal: Signal) -> Vec<Signal>;

    fn add_receiver(&mut self, receiver: &str);
}

struct Sender {
    name: String,
    receivers: Vec<String>,
}

impl Sender {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            receivers: Vec::new(),
        }
    }

    fn send(&self, pulse: Pulse) -> Vec<Signal> {
        self.receivers
            .iter()
            .map(|receiver| Signal {
                from: self.name.clone(),
                to: receiver.clone(),
                pulse: pulse,
            })
            .collect()
    }

    fn add_receiver(&mut self, receiver: &str) {
        self.receivers.push(receiver.to_string());
    }
}

struct Broadcaster {
    sender: Sender,
}

impl Broadcaster {
    fn start(&self) -> Vec<Signal> {
        self.sender.send(Pulse::Low)
    }
}

impl Module for Broadcaster {
    fn process(&mut self, signal: Signal) -> Vec<Signal> {
        self.sender.send(signal.pulse)
    }

    fn add_receiver(&mut self, receiver: &str) {
        self.sender.add_receiver(receiver);
    }
}

struct FlipFlop {
    on: bool,
    sender: Sender,
}

impl FlipFlop {
    fn new(name: &str) -> Self {
        Self {
            on: false,
            sender: Sender::new(name),
        }
    }
}

impl Module for FlipFlop {
    fn process(&mut self, signal: Signal) -> Vec<Signal> {
        match signal.pulse {
            Pulse::Low => {
                self.on = !self.on;

                let pulse;
                if self.on {
                    pulse = Pulse::High
                } else {
                    pulse = Pulse::Low
                }

                self.sender.send(pulse)
            }
            Pulse::High => Vec::new(),
        }
    }

    fn add_receiver(&mut self, receiver: &str) {
        self.sender.add_receiver(receiver);
    }
}

struct Conjunction {
    last_received: HashMap<String, Pulse>,
    sender: Sender,
}

impl Conjunction {
    fn new(name: &str) -> Self {
        Self {
            last_received: HashMap::new(),
            sender: Sender::new(name),
        }
    }
}

impl Module for Conjunction {
    fn process(&mut self, signal: Signal) -> Vec<Signal> {
        self.last_received.insert(signal.from.clone(), signal.pulse);

        let pulse;

        if self
            .last_received
            .values()
            .all(|pulse| *pulse == Pulse::High)
        {
            pulse = Pulse::Low;
        } else {
            pulse = Pulse::High;
        }

        self.sender.send(pulse)
    }

    fn add_receiver(&mut self, receiver: &str) {
        self.sender.add_receiver(receiver);
    }
}

enum ModuleType {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

struct Modules {
    map: HashMap<String, ModuleType>,
}

impl Modules {
    fn parse(s: &str) -> Self {
        let mut modules = HashMap::new();
        let broadcaster = Broadcaster {
            sender: Sender::new("broadcaster"),
        };

        modules.insert(
            String::from("broadcaster"),
            ModuleType::Broadcaster(broadcaster),
        );

        for line in s.lines() {
            let name = line.split(" -> ").next().unwrap();

            if let Some(name) = name.strip_prefix("%") {
                modules.insert(name.to_string(), ModuleType::FlipFlop(FlipFlop::new(name)));
            } else if let Some(name) = name.strip_prefix("&") {
                modules.insert(
                    name.to_string(),
                    ModuleType::Conjunction(Conjunction::new(name)),
                );
            }
        }

        for line in s.lines() {
            let mut split = line.split(" -> ");
            let mut name = split.next().unwrap();
            if name != "broadcaster" {
                name = &name[1..];
            }

            for receiver_name in split.next().unwrap().split(", ") {
                let receiver = modules.get_mut(receiver_name);
                if receiver.is_some() {
                    if let ModuleType::Conjunction(conjunction) = receiver.unwrap() {
                        conjunction
                            .last_received
                            .insert(name.to_string(), Pulse::Low);
                    }
                }

                let module = modules.get_mut(name);

                if module.is_none() {
                    continue;
                }

                match module.unwrap() {
                    ModuleType::Broadcaster(b) => b.add_receiver(receiver_name),
                    ModuleType::FlipFlop(f) => f.add_receiver(receiver_name),
                    ModuleType::Conjunction(c) => c.add_receiver(receiver_name),
                }
            }
        }

        return Self { map: modules };
    }

    fn start(&mut self) -> (i64, i64, bool) {
        let mut q = VecDeque::new();
        if let ModuleType::Broadcaster(b) = self.map.get_mut("broadcaster").unwrap() {
            Modules::add_signals(&mut q, b.start());
        }

        let mut high_count = 0;
        //we count the first signal from button to broadcaster
        let mut low_count = 1;
        let mut sent_to_rx = false;
        while let Some(signal) = q.pop_front() {
            match signal.pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }

            if signal.to == "rx" && signal.pulse == Pulse::Low {
                sent_to_rx = true;
            }

            let to = self.map.get_mut(&signal.to);
            if to.is_none() {
                continue;
            }

            let signals = match to.unwrap() {
                ModuleType::Broadcaster(bc) => bc.process(signal),
                ModuleType::FlipFlop(ff) => ff.process(signal),
                ModuleType::Conjunction(cj) => cj.process(signal),
            };

            Modules::add_signals(&mut q, signals);
        }

        (low_count, high_count, sent_to_rx)
    }

    fn add_signals(q: &mut VecDeque<Signal>, signals: Vec<Signal>) {
        for signal in signals {
            q.push_back(signal);
        }
    }
}

fn main() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let mut modules = Modules::parse(&input);

    let mut low = 0;
    let mut high = 0;
    let mut button_presses = 0;
    loop {
        button_presses += 1;
        let (low_count, high_count, sent_to_rx) = modules.start();

        if sent_to_rx {
            break;
        }

        low += low_count;
        high += high_count;
    }

    //Solved part 2 by hand in the end, because it was a pain
    //to make it generic :(
    println!("{} {} {}", low, high, low * high);
    println!("{}", button_presses);
}
