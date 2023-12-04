use std::collections::HashSet;

fn solution(input: &str) -> u32 {
    input
        .lines()
        // Remove the Card X: part
        .map(|l| l.split(':').nth(1).unwrap().trim())
        .map(|l| {
            let split_row = l
                .splitn(2, '|')
                // Convert each part of the line to a vector of numbers
                .map(|text| {
                    text.trim()
                        .split(' ')
                        .filter_map(|char| char.parse::<u32>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let winning: HashSet<u32> = split_row[0].iter().copied().collect();

            let num_won = split_row[1]
                .iter()
                .filter(|num| winning.contains(num))
                .collect::<Vec<_>>()
                .len() as u32;

            if num_won > 0 {
                2u32.pow(num_won - 1)
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");
    let res = solution(input);

    assert_eq!(res, 13);
}

#[test]
fn test_input() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    assert_eq!(res, 32001);
}
