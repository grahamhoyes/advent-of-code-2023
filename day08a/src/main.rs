use regex::Regex;
use std::collections::HashMap;

fn solution(input: &str) -> usize {
    let mut lines = input.lines();

    // Infinite cyclic iterator over the sequence of moves
    let mut sequence = lines.next().unwrap().trim().chars().cycle();

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

    let mut current_node = "AAA";
    let mut steps = 0;

    while current_node != "ZZZ" {
        let n = nodes.get(current_node).unwrap();
        steps += 1;

        current_node = match sequence.next().unwrap() {
            'L' => n.0.as_str(),
            'R' => n.1.as_str(),
            _ => unreachable!(),
        }
    }

    steps
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

        assert_eq!(res, 11309);
    }
}
