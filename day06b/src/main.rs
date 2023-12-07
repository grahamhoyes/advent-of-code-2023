fn solution(input: &str) -> usize {
    let mut data = input.lines().map(|line| {
        // Line 1 starts with "Time:", line 2 with "Distance:". We can just slice
        // the first 9 characters off of both lines then parse out the numbers.
        line[9..].replace(' ', "").parse::<usize>().unwrap()
    });

    let time = data.next().unwrap();
    let distance = data.next().unwrap();

    // Number of ways to win this race
    (0..=time)
        .filter_map(|hold_duration| {
            let distance_travelled = (time - hold_duration) * hold_duration;

            if distance_travelled > distance {
                Some(1)
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 71503);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 26499773);
    }
}
