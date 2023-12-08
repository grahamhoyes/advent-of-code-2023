use num::Integer; // Integer trait gives us the lcm method
use regex::Regex;
use std::collections::HashMap;

fn solution(input: &str) -> usize {
    let mut lines = input.lines();

    let raw_sequence = lines.next().unwrap().trim();

    // Skip the next blank line
    lines.next().unwrap();

    let re = Regex::new(r"(?<node>\w{3}) = \((?<l>\w{3}), (?<r>\w{3})\)").unwrap();

    let nodes: HashMap<String, (String, String)> = lines
        .map(|line| {
            let caps = re.captures(line).unwrap();

            (
                caps["node"].to_string(),
                (caps["l"].to_string(), caps["r"].to_string()),
            )
        })
        .collect();

    let mut current_nodes = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    // Actually computing all paths in parallel until they converge would take way,
    // way too long. However, there's a trick in the input: The paths are cyclic.
    // When you reach an ending node, the next step in the sequence will take you
    // back to the start of that path. Therefore, we just need to figure out how many
    // steps each sequence takes on its own, and compute the LCM of those.
    let mut lcm: usize = 1;

    for node in current_nodes.iter_mut() {
        let mut num_steps: usize = 0;

        // Restart the sequence for this path
        let mut sequence = raw_sequence.chars().cycle();

        while !node.ends_with('Z') {
            num_steps += 1;
            let next = nodes.get(*node).unwrap();

            *node = match sequence.next().unwrap() {
                'L' => &next.0,
                'R' => &next.1,
                _ => unreachable!(),
            }
        }

        lcm = lcm.lcm(&num_steps);
    }

    lcm
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

        assert_eq!(res, 6);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 13740108158591);
    }
}
