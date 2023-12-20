use std::collections::HashMap;

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
    outputs: Vec<String>,
    state: State,
}

impl FlipFlop {
    fn new(outputs: Vec<String>) -> Self {
        Self {
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
    outputs: Vec<String>,
    memory: HashMap<String, State>,
}

// An active-low AND gate
impl Conjunction {
    fn new(outputs: Vec<String>) -> Self {
        Self {
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
struct Output {
    // An output is enabled when it has been sent a low pulse
    enabled: bool,
}

impl Output {
    fn new() -> Self {
        Self { enabled: false }
    }

    fn process(&mut self, input: Pulse) {
        if let Pulse::Low = input {
            println!("Output got a low pulse");
            self.enabled = true;
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
    Output(Output),
}

impl Module {
    /// Return the contained output variant, panicking if this is
    /// not an output
    fn output(&self) -> &Output {
        match self {
            Module::Output(o) => o,
            _ => panic!("Not an output"),
        }
    }
}

fn load_modules(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<String> = outputs.split(", ").map(String::from).collect();

        if let Some(name) = name.strip_prefix('%') {
            modules.insert(name.to_string(), Module::FlipFlop(FlipFlop::new(outputs)));
        } else if let Some(name) = name.strip_prefix('&') {
            modules.insert(
                name.to_string(),
                Module::Conjunction(Conjunction::new(outputs)),
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
                    new_outputs.push((o.clone(), Module::Output(Output::new())));
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

fn print_mermaid_diagram(modules: &HashMap<String, Module>) {
    println!("flowchart TB");
    println!("    broadcaster[[broadcaster]]");

    let mut cons = Vec::new();
    let mut flops = Vec::new();
    let mut connections = Vec::new();

    for (name, module) in modules.iter() {
        match module {
            Module::FlipFlop(f) => {
                flops.push((name, f));
                for o in f.outputs.iter() {
                    connections.push(format!("    {}-->{}", name, o));
                }
            }
            Module::Conjunction(c) => {
                cons.push((name, c));
                for o in c.outputs.iter() {
                    connections.push(format!("    {}-->{}", name, o));
                }
            }
            Module::Broadcaster { outputs } => {
                for o in outputs {
                    connections.push(format!("    {}-->{}", name, o));
                }
            }
            _ => {}
        }
    }

    // Use a stable order for outputting the nodes so that the diagram
    // is consistent.
    cons.sort_by_key(|(name, _)| *name);
    flops.sort_by_key(|(name, _)| *name);

    for (name, _) in cons.iter() {
        println!("    {}([&{}])", name, name);
    }

    for (name, _) in flops.iter() {
        // Lots of braces needed to escape double braces in the output
        println!("    {}{{{{%{}}}}}", name, name);
    }

    println!("    rx[[rx]]");

    // With the nodes in the right places, print the connections
    for c in connections {
        println!("{}", c);
    }
}

/// No computed solution for this part, see the readme for a proof by inspection.
fn main() {
    let input = include_str!("../input.txt");
    let modules = load_modules(input);
    print_mermaid_diagram(&modules);
}
