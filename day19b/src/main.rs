use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
enum Op {
    Gt,
    Lt,
}

impl Op {
    fn cmp(&self, lhs: &u16, rhs: &u16) -> bool {
        match self {
            Op::Gt => lhs > rhs,
            Op::Lt => lhs < rhs,
        }
    }
}

#[derive(Debug)]
enum Dest {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug)]
enum Rule {
    Conditional {
        category: String,
        op: Op,
        val: u16,
        dest: Dest,
    },
    Unconditional(Dest),
}

/// Apply workflows until the part is accepted or reject. Returns true if the
/// part is accepted, otherwise false.
fn apply_workflows(workflows: &HashMap<String, Vec<Rule>>, part: &HashMap<String, u16>) -> bool {
    let mut flow = &workflows["in"];

    loop {
        for rule in flow {
            match rule {
                Rule::Unconditional(dest) => match dest {
                    Dest::Accept => return true,
                    Dest::Reject => return false,
                    Dest::Goto(f) => {
                        flow = &workflows[f];
                        break;
                    }
                },
                Rule::Conditional {
                    category,
                    op,
                    val,
                    dest,
                } => {
                    if op.cmp(&part[category], val) {
                        match dest {
                            Dest::Accept => return true,
                            Dest::Reject => return false,
                            Dest::Goto(f) => {
                                flow = &workflows[f];
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn solution(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let workflow_re = Regex::new(r"(?<name>[a-z]+)\{(?<rules>.+)}").unwrap();
    let comp_rule_re =
        Regex::new(r"(?<category>[xmas])(?<op>[<>])(?<val>\d+):(?<dest>[a-zA-Z]+)").unwrap();

    let workflows: HashMap<String, Vec<Rule>> = workflows
        .lines()
        .map(|l| {
            let caps = workflow_re.captures(l).expect("Found no matches");

            let name = caps["name"].to_string();
            let rules = caps["rules"]
                .split(',')
                .map(|rule| {
                    if let Some(caps) = comp_rule_re.captures(rule) {
                        let category = caps["category"].to_string();
                        let op = if &caps["op"] == ">" { Op::Gt } else { Op::Lt };
                        let val: u16 = caps["val"].parse().unwrap();
                        let dest = match &caps["dest"] {
                            "A" => Dest::Accept,
                            "R" => Dest::Reject,
                            d => Dest::Goto(d.to_string()),
                        };

                        Rule::Conditional {
                            category,
                            op,
                            val,
                            dest,
                        }
                    } else if rule == "A" {
                        Rule::Unconditional(Dest::Accept)
                    } else if rule == "R" {
                        Rule::Unconditional(Dest::Reject)
                    } else {
                        Rule::Unconditional(Dest::Goto(rule.to_string()))
                    }
                })
                .collect();

            (name, rules)
        })
        .collect();

    let mut valid: usize = 0;

    for x in 1..=4000 {
        println!(".");
        for m in 1..=4000 {
            for a in 1..=4000 {
                for s in 1..=4000 {
                    let part: HashMap<String, u16> = [("x", x), ("m", m), ("a", a), ("s", s)]
                        .into_iter()
                        .map(|(key, val)| (key.to_string(), val))
                        .collect();

                    if apply_workflows(&workflows, &part) {
                        valid += 1;
                    }
                }
            }
        }
    }

    valid
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
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 19114);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 376008);
    }
}
