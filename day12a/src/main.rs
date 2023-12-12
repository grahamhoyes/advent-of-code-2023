/// Recursively count the number of possible arrangements for pattern
/// of observed hot springs with unknowns and known run lengths of the correct
/// observations.
///
///     `pattern`: A string array of '.', '?', and '#'
///     `runs`: An array of numbers storing the correct length of each '#' run
///     `current_run_count`: The number of '#' that immediately precede the pattern
fn num_arrangements(pattern: &str, runs: &[usize], current_run_count: usize) -> usize {
    if pattern.is_empty() {
        #[allow(clippy::if_same_then_else)]
        return if runs.len() == 1 && runs[0] == current_run_count {
            // Processed the entire pattern, and the run we were tracking matches
            // the last one we needed
            1
        } else if runs.is_empty() && current_run_count == 0 {
            // Processed the entire pattern, and we already found all
            // the matches we need
            1
        } else {
            // Processed the entire pattern, but there is still at least
            // 1 unmatched run or we had a run that was too long
            0
        };
    }

    let current = pattern.chars().next().unwrap();
    let rest = pattern.get(1..).unwrap();

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
            match c {
                '#' => num_arrangements(rest, runs, current_run_count + 1),
                '.' => {
                    let first_run = runs.first();

                    if first_run.map_or(false, |length| current_run_count == *length) {
                        // We finished observing a run of broken hot springs and it matched
                        // what we expected, so is valid.
                        num_arrangements(rest, runs.get(1..).unwrap(), 0)
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

            num_arrangements(pattern, &runs, 0)
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

        assert_eq!(res, 21);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 7771);
    }
}
