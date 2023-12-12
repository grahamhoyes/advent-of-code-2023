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
    current: Option<char>,
    pattern: &str,
    runs: &[usize],
    current_run_count: usize,
) -> usize {
    if current.is_none() {
        return if runs.is_empty() {
            1
        } else {
            // No more patterns but more recorded counts means this branch
            // is impossible
            0
        };
    }

    let current = current.unwrap();

    println!("{}", pattern);
    let next = pattern.chars().next();
    let rest = &pattern[1..];

    match current {
        '#' => num_arrangements(next, rest, runs, current_run_count + 1),
        '.' => {
            if current_run_count == runs[0] {
                // We finished observing a run of broken hot springs and it matched
                // what we expected, so is valid.
                num_arrangements(next, rest, &runs[1..], 0)
            } else if current_run_count == 0 {
                // Haven't started tracking a run yet, move along
                num_arrangements(next, rest, runs, 0)
            } else {
                // We have finished observing a run, but it's length wasn't
                // compatible with the known run lengths. Invalid path.
                0
            }
        }
        '?' => {
            num_arrangements(Some('.'), pattern, runs, current_run_count)
                + num_arrangements(Some('#'), pattern, runs, current_run_count)
        }
        _ => unreachable!(),
    }
}

fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (pattern, runs) = l.split_once(' ').unwrap();

            let runs: Vec<usize> = runs.split(',').map(|x| x.parse().unwrap()).collect();

            let first_char = pattern.chars().next().unwrap();

            num_arrangements(Some(first_char), &pattern[1..], &runs, 0)
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
