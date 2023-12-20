use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
    None,
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    outputs: Vec<String>,
    state: State,
}

impl FlipFlop {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Self {
            name,
            outputs,
            state: State::Off,
        }
    }

    fn process(&mut self, input: Pulse) -> Pulse {
        match (input, &self.state) {
            (Pulse::High, _) => Pulse::None,
            (Pulse::Low, &State::On) => {
                self.state = State::Off;
                Pulse::Low
            }
            (Pulse::Low, &State::Off) => {
                self.state = State::On;
                Pulse::High
            }
            (Pulse::None, _) => Pulse::None,
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    outputs: Vec<String>,
    memory: HashMap<String, State>,
}

impl Conjunction {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Self {
            name,
            outputs,
            memory: HashMap::new(),
        }
    }

    fn add_inputs(&mut self, inputs: Vec<String>) {
        for input in inputs {
            self.memory.insert(input, State::Off);
        }
    }

    fn process(&mut self, from: &str, input: Pulse) -> Pulse {
        match input {
            Pulse::None => return Pulse::None,
            Pulse::High => {
                *self.memory.get_mut(from).unwrap() = State::On;
            }
            Pulse::Low => {
                *self.memory.get_mut(from).unwrap() = State::Off;
            }
        }

        if self.memory.iter().all(|(_, state)| state == &State::On) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    /// There should only be one broadcaster module
    Broadcaster {
        outputs: Vec<String>,
    },
    /// Outputs are the end of a chain
    Output,
}

fn load_modules(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<String> = outputs.split(", ").map(String::from).collect();

        if let Some(name) = name.strip_prefix('%') {
            modules.insert(
                name.to_string(),
                Module::FlipFlop(FlipFlop::new(name.to_string(), outputs)),
            );
        } else if let Some(name) = name.strip_prefix('&') {
            modules.insert(
                name.to_string(),
                Module::Conjunction(Conjunction::new(name.to_string(), outputs)),
            );
        } else if name == "broadcaster" {
            modules.insert(name.to_string(), Module::Broadcaster { outputs });
        } else {
            panic!("Unrecognized module {name}")
        }
    }

    // With modules loaded, we need to do a pass through to set up the inputs
    // for the conjunctions and any output-only modules. Because rust, we need
    // to do this as two separate loops to avoid mutating modules while
    // iterating over it
    let mut new_outputs: Vec<(String, Module)> = Vec::new();
    let mut module_inputs: HashMap<String, Vec<String>> = HashMap::new();

    for (name, module) in modules.iter() {
        if let Module::FlipFlop(FlipFlop { outputs, .. })
        | Module::Conjunction(Conjunction { outputs, .. }) = module
        {
            for o in outputs {
                if !modules.contains_key(o) {
                    new_outputs.push((o.clone(), Module::Output));
                }
                module_inputs
                    .entry(o.clone())
                    .or_default()
                    .push(name.clone());
            }
        }
    }

    modules.extend(new_outputs);

    // Go through and set the inputs for the conjunction modules
    for (name, inputs) in module_inputs {
        if let Module::Conjunction(c) = modules
            .get_mut(&name)
            .unwrap_or_else(|| panic!("Unknown module {}", name))
        {
            c.add_inputs(inputs);
        }
    }

    modules
}

/// Simulate a button press on the modules. Returns the number
/// of low and high pulses that were sent.
fn simulate_button_press(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut pulses_sent = (1, 0); // The button press is one low pulse

    // Tuples of (from, to, pulse)
    let mut signals: VecDeque<(String, String, Pulse)> = VecDeque::new();

    // The button press sends a low signal through the broadcaster
    if let Module::Broadcaster { outputs } = modules.get("broadcaster").unwrap() {
        for output in outputs {
            signals.push_back(("broadcaster".to_string(), output.clone(), Pulse::Low));
            pulses_sent.0 += 1
        }
    } else {
        panic!("Failed to find broadcaster module");
    }

    while let Some((source, dest, pulse)) = signals.pop_front() {
        match modules.get_mut(&dest).unwrap() {
            Module::FlipFlop(f) => match f.process(pulse) {
                Pulse::None => continue,
                Pulse::Low => {
                    for output in f.outputs.iter() {
                        // The destination of this signal is the source of the next one
                        signals.push_back((dest.clone(), output.clone(), Pulse::Low));
                        pulses_sent.0 += 1;
                    }
                }
                Pulse::High => {
                    for output in f.outputs.iter() {
                        // The destination of this signal is the source of the next one
                        signals.push_back((dest.clone(), output.clone(), Pulse::High));
                        pulses_sent.1 += 1;
                    }
                }
            },
            Module::Conjunction(c) => match c.process(&source, pulse) {
                Pulse::None => continue,
                Pulse::Low => {
                    for output in c.outputs.iter() {
                        signals.push_back((dest.clone(), output.clone(), Pulse::Low));
                        pulses_sent.0 += 1;
                    }
                }
                Pulse::High => {
                    for output in c.outputs.iter() {
                        signals.push_back((dest.clone(), output.clone(), Pulse::High));
                        pulses_sent.1 += 1;
                    }
                }
            },
            _ => {}
        }
    }

    pulses_sent
}

fn solution(input: &str) -> usize {
    let mut modules = load_modules(input);

    println!("{:?}", modules);

    let mut low_sent = 0;
    let mut high_sent = 0;

    for i in 0..1000 {
        let res = simulate_button_press(&mut modules);
        low_sent += res.0;
        high_sent += res.1;
    }

    low_sent * high_sent
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = include_str!("../example_1.txt");
        let res = solution(input);

        assert_eq!(res, 32000000);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../example_2.txt");
        let res = solution(input);

        assert_eq!(res, 11687500);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 818649769);
    }
}
