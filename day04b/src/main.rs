use std::collections::HashSet;

fn solution(input: &str) -> usize {
    // cards[i] is a tuple of how many wins card i+1 had, and how many copies
    // of that card we get. Copies starts at 1, and is incremented below.
    let mut cards: Vec<(usize, usize)> = input
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

            // Set of winning numbers
            let winning: HashSet<u32> = split_row[0].iter().copied().collect();

            // Number of matches
            let num_won = split_row[1]
                .iter()
                .filter(|num| winning.contains(num))
                .cloned()
                .collect::<Vec<_>>()
                .len();

            (num_won, 1)
        })
        .collect();

    for card in 0..cards.len() {
        let (num_won, this_num_copies) = cards[card];

        if num_won > 0 {
            for i in 1..=num_won {
                if let Some((_, num_copies)) = cards.get_mut(card + i) {
                    *num_copies += this_num_copies;
                }
            }
        }
    }

    cards.iter().map(|c| c.1).sum()
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

    assert_eq!(res, 30);
}

#[test]
fn test_input() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    assert_eq!(res, 5037841);
}
