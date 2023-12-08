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

    let mut current_nodes = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();
    let num_paths = current_nodes.len();

    println!("Number of paths to follow: {}", num_paths);

    let mut steps = 0;
    let mut num_complete = 0;

    // This is far to slow to run in any reasonable amount of time
    // while !current_nodes.iter().all(|node| node.ends_with('Z')) {
    while num_complete < num_paths {
        steps += 1;
        num_complete = 0;

        let dir = sequence.next().unwrap();

        for node in current_nodes.iter_mut() {
            let next = nodes.get(*node).unwrap();

            match dir {
                'L' => *node = &next.0,
                'R' => *node = &next.1,
                _ => unreachable!(),
            }

            if node.ends_with('Z') {
                num_complete += 1
            }
        }

        if num_complete > 2 {
            println!("Number complete: {num_complete}");
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
