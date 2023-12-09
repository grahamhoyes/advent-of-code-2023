fn solution(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let mut history: Vec<Vec<i32>> = Vec::new();

            history.push(l.split(' ').map(|x| x.parse::<i32>().unwrap()).collect());

            loop {
                let last = history.last().unwrap();
                let differences = last
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(i, val)| val - last[i - 1])
                    .collect::<Vec<_>>();

                let all_zeros = differences.iter().all(|x| *x == 0);

                history.push(differences);

                if all_zeros {
                    break;
                }
            }

            // Add a zero to the last history item, then work our way back up
            history.last_mut().unwrap().push(0);

            for i in (0..=history.len() - 2).rev() {
                let next_val = history[i].last().unwrap() + history[i + 1].last().unwrap();
                history[i].push(next_val);
            }

            *history.first().unwrap().last().unwrap()
        })
        .sum()
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

        assert_eq!(res, 114);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1938731307);
    }
}
