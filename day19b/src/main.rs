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
    Conditional {
        category: String,
        condition: Condition,
        dest: Dest,
    },
    Unconditional(Dest),
}

// Do some stuff. Return the rule streams that lead to an accept condition.
fn parse_rule_stream(
    workflows: &HashMap<String, Vec<Rule>>,
    start: &str,
    mut this_run: Vec<Condition>,
    all_runs: &mut Vec<Vec<Condition>>,
) {
    let flow = &workflows[start];

    let mut conditions: Vec<Condition> = Vec::new();

    for rule in flow {
        match rule {
            Rule::Unconditional(dest) => match dest {
                // Unconditional rules are always the end of a rule set,
                // so these must all return
                Dest::Accept => {
                    // When we hit an accept, save the history that led here
                    this_run.extend_from_slice(&conditions);
                    all_runs.push(this_run);
                    return;
                }
                Dest::Reject => {
                    // When we hit a reject, discard the history
                    return;
                }
                Dest::Goto(f) => {
                    // When we hit a fork, clone the history so far and proceed
                    // with that
                    this_run.extend_from_slice(&conditions);
                    parse_rule_stream(workflows, f, this_run, all_runs);
                    return;
                }
            },
            Rule::Conditional {
                condition,
                category,
                dest,
            } => {
                // Fork on the condition
                match dest {
                    Dest::Accept => {
                        conditions.push(condition.clone());
                        this_run.extend_from_slice(&conditions);
                        all_runs.push(this_run);
                        return;
                    }
                    Dest::Reject => {
                        return;
                    }
                    Dest::Goto(f) => {
                        let mut forked_run = this_run.clone();
                        let mut forked_conditions = conditions.clone();
                        forked_conditions.push(condition.clone());
                        forked_run.extend_from_slice(&forked_conditions);
                        parse_rule_stream(workflows, f, forked_run, all_runs);
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
                            category: category.clone(),
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
    parse_rule_stream(&workflows, "in", this_run, &mut all_runs);

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
