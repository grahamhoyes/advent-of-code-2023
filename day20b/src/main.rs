use std::collections::HashMap;

#[derive(Debug)]
struct FlipFlop {
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new(outputs: Vec<String>) -> Self {
        Self { outputs }
    }
}

#[derive(Debug)]
struct Conjunction {
    outputs: Vec<String>,
}

// An active-low AND gate
impl Conjunction {
    fn new(outputs: Vec<String>) -> Self {
        Self { outputs }
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

    let mut new_outputs: Vec<(String, Module)> = Vec::new();

    for (_, module) in modules.iter() {
        if let Module::FlipFlop(FlipFlop { outputs, .. })
        | Module::Conjunction(Conjunction { outputs, .. }) = module
        {
            for o in outputs {
                if !modules.contains_key(o) {
                    new_outputs.push((o.clone(), Module::Output));
                }
            }
        }
    }

    modules.extend(new_outputs);

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
