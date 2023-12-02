const REPLACEMENTS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            // Need to account for things like "eightwo", which should be considered "82"

            // Tuple is (position, value)
            let mut numbers: Vec<(usize, u32)> = vec![];

            for (word, val) in REPLACEMENTS {
                // Find the first and last spelled out digits, if there are any
                if let Some(idx) = l.find(word) {
                    numbers.push((idx, *val));
                }

                if let Some(idx) = l.rfind(word) {
                    numbers.push((idx, *val));
                }
            }

            // Find regular digits
            if let Some(idx) = l.find(char::is_numeric) {
                let num = l.chars().nth(idx).unwrap().to_digit(10).unwrap();
                numbers.push((idx, num));
            }

            if let Some(idx) = l.rfind(char::is_numeric) {
                let num = l.chars().nth(idx).unwrap().to_digit(10).unwrap();
                numbers.push((idx, num));
            }

            // Sort the numbers by index
            numbers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            let first_num = numbers.first().unwrap().1;
            let last_num = numbers.last().unwrap().1;

            first_num * 10 + last_num
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[test]
fn test_solution() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    assert_eq!(res, 55701);
}
