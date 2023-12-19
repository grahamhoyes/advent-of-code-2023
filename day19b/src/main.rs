use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
enum Op {
    Gt,
    Lt,
}

#[derive(Debug)]
enum Dest {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug, Clone)]
struct Condition {
    key: String,
    op: Op,
    val: u16,
}

impl Condition {
    /// Get the logical complement of this condition
    fn complement(self) -> Self {
        let (op, val) = match self.op {
            Op::Lt => (Op::Gt, self.val - 1),
            Op::Gt => (Op::Lt, self.val + 1),
        };

        Self {
            key: self.key,
            op,
            val,
        }
    }
}

#[derive(Debug)]
enum Rule {
    Conditional { condition: Condition, dest: Dest },
    Unconditional(Dest),
}

/// Find the conditions that result in a part being accepted.
///
/// Conjunctions (ANDs) of conditions needed to accept a part are added to
/// accepted_runs when one is found. accepted_runs as a whole is a disjunction (OR)
/// of acceptable conditions.
fn find_accept_conditions(
    workflows: &HashMap<String, Vec<Rule>>,
    start: &str,
    mut this_run: Vec<Condition>,
    accepted_runs: &mut Vec<Vec<Condition>>,
) {
    let flow = &workflows[start];

    let mut conditions: Vec<Condition> = Vec::new();

    for rule in flow {
        match rule {
            Rule::Unconditional(dest) => {
                // Unconditional rules are always the end of a rule set,
                // so these must all return
                match dest {
                    Dest::Accept => {
                        // When we hit an accept, save the history that led here
                        this_run.extend_from_slice(&conditions);
                        accepted_runs.push(this_run);
                    }
                    Dest::Reject => {
                        // When we hit a reject, discard the history
                    }
                    Dest::Goto(f) => {
                        // When we hit an unconditional goto, add conditions from this
                        // rule set and proceed from there.
                        this_run.extend_from_slice(&conditions);
                        find_accept_conditions(workflows, f, this_run, accepted_runs);
                    }
                }
                return;
            }
            Rule::Conditional { condition, dest } => {
                // Fork on the condition
                let mut forked_run = this_run.clone();
                let mut forked_conditions = conditions.clone();
                forked_conditions.push(condition.clone());
                forked_run.extend_from_slice(&forked_conditions);

                match dest {
                    Dest::Accept => {
                        accepted_runs.push(forked_run);
                    }
                    Dest::Reject => {}
                    Dest::Goto(f) => {
                        find_accept_conditions(workflows, f, forked_run, accepted_runs);
                    }
                }

                // Rules will never end in a conditional case. We push the complement
                // of this condition since it must be false to reach the next rule.
                conditions.push(condition.clone().complement())
            }
        };
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
                            condition: Condition {
                                key: category,
                                op,
                                val,
                            },
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

    // Brute forcing doesn't work, there are over 2^47 combinations to check.
    // The workflows form a tree, starting with the `in` node (let's hope it's
    // non-cyclic). By doing a depth first search, we can get a list of all
    // conditions that result in a part being accepted. Build bounded regions,
    // calculate their 4D volumes.
    let mut all_runs = Vec::new();
    let this_run = Vec::new();
    find_accept_conditions(&workflows, "in", this_run, &mut all_runs);

    for ruleset in all_runs {
        println!("{:?}", ruleset);
        println!();
    }

    0
}

fn main() {
    let input = include_str!("../example.txt");
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
