/// Recursively count the number of possible arrangements for pattern
/// of observed hot springs with unknowns and known run lengths of the correct
/// observations.
///
///     `current`: The current byte at the beginning of the pattern
///     `pattern`: A byte array of '.', '?', and '#', the rest of the pattern after `current`
///     `runs`: An array of numbers storing the correct length of each '#' run
///     `current_run_count`: The number of '#' that immediately precede the pattern
///
/// We pass `current` as a separate parameter from `pattern` to make recursive checking easier,
/// without needing to allocate new strings.
fn num_arrangements(
    pattern: Option<&str>,
    runs: Option<&[usize]>,
    current_run_count: usize,
) -> usize {
    if pattern.is_none() || pattern.unwrap().is_empty() {
        return if (runs.is_some()
            && runs.unwrap().len() == 1
            && runs.unwrap()[0] == current_run_count)
            || (runs.is_none() || runs.unwrap().is_empty())
        {
            1
        } else {
            // No more patterns but more recorded counts means this branch
            // is impossible
            0
        };
    }
    // else if runs.is_none() || runs.unwrap().is_empty() {
    //     // Not a valid base case, we might run out of runs before the rest of the
    //     // pattern has been validated
    //     return 0;
    // }

    let pattern = pattern.unwrap();

    // println!("{:?}", pattern);
    let current = pattern.chars().next().unwrap();
    let rest = pattern.get(1..);

    let values_to_check = if current == '?' {
        ['.', '#']
    } else {
        // Rust wants to allocate an array of two elements on the stack,
        // so use a space character as a dummy one we can skip
        [current, ' ']
    };

    values_to_check
        .iter()
        .map(|c| {
            println!(
                "{}{}{}",
                (0..10 - rest.map_or(0, |r| r.len()))
                    .map(|_| ' ')
                    .collect::<String>(),
                c,
                rest.map_or("", |x| x)
            );
            match c {
                '#' => num_arrangements(rest, runs, current_run_count + 1),
                '.' => {
                    let first_run = runs.and_then(|r| r.first());

                    // if current_run_count > 0 && first_run.is_none() {
                    //     // We are tracking a run of broken hot springs, but expect no more
                    //     0
                    // } else
                    if first_run.map_or(false, |length| current_run_count == *length) {
                        // We finished observing a run of broken hot springs and it matched
                        // what we expected, so is valid.
                        num_arrangements(rest, runs.unwrap().get(1..), 0)
                    } else if current_run_count == 0 {
                        // Haven't started tracking a run yet, move along
                        num_arrangements(rest, runs, 0)
                    } else {
                        // We have finished observing a run, but it's length wasn't
                        // compatible with the known run lengths. Invalid path.
                        0
                    }
                }
                _ => 0,
            }
        })
        .sum()
}

fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (pattern, runs) = l.split_once(' ').unwrap();

            let runs: Vec<usize> = runs.split(',').map(|x| x.parse().unwrap()).collect();

            let arrangements = num_arrangements(Some(pattern), Some(&runs), 0);

            println!("{}: {}", pattern, arrangements);
            arrangements
        })
        .sum()
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

        assert_eq!(res, 0);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 0);
    }
}
