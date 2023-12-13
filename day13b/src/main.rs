/// Detect a palindrome that is offset to the right, and return the
/// number of elements to the left of the axis of symmetry.
///
/// To detect a left offset, reverse the input.
fn detect_offset_palindrome(
    items: &[usize],
    allow_smudges: bool,
    ignore_num: usize,
) -> Option<usize> {
    // How many smudges we can use
    let mut smudge_count = if allow_smudges { 1 } else { 0 };

    // An offset palindrome will always reach the end of one side
    // of the array, so start there until we find a match

    let mut i_start = 0;
    let mut j = items.len() - 1;

    // Advance the left pointer until it matches the last element (maybe smudged)
    while (items[i_start] ^ items[j]).count_ones() > smudge_count && i_start < j {
        i_start += 1;
    }

    if i_start == j {
        // Went through the whole list and didn't find a match
        return None;
    }

    while i_start < j {
        let mut i = i_start;

        while (items[i] ^ items[j]).count_ones() <= smudge_count && i < j {
            if (items[i] ^ items[j]).count_ones() == 1 {
                smudge_count = 0
            }

            i += 1;
            j -= 1;
        }

        // This problem in particular is for even number palindromes - the
        // axis of symmetry must be between two elements, not on one. For a
        // normal palindrome, this should be i >= j.
        if i > j && (!allow_smudges || (ignore_num == 0 || i != ignore_num)) {
            return Some(i);
        }

        // Start the search again, from one more element further from the left
        j = items.len() - 1;
        i_start += 1;
        smudge_count = if allow_smudges { 1 } else { 0 }
    }

    None
}

#[derive(Eq, PartialEq, Debug)]
enum DetectedBy {
    Rows,
    RowsReversed,
    Cols,
    ColsReversed,
}

fn solution(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|block| {
            // Hash each row / column into an integer by treating it as a binary sequence

            let rows: Vec<usize> = block
                .lines()
                .map(|l| {
                    l.chars()
                        .fold(0, |accum, c| (accum << 1) + if c == '#' { 1 } else { 0 })
                })
                .collect();

            let num_cols = block.lines().nth(1).unwrap().len();
            let num_rows = rows.len();

            // Iterating over the columns is slightly less elegant, but the same thing
            let cols: Vec<usize> = (0..num_cols)
                .map(|j| {
                    (0..num_rows)
                        // Need (num_cols + 1) here to account for the \n character ending each line
                        .map(move |i| block.as_bytes()[i * (num_cols + 1) + j])
                        .fold(0, |accum, c| (accum << 1) + if c == b'#' { 1 } else { 0 })
                })
                .collect();

            (rows, cols)
        })
        .map(|(rows, cols)| {
            // Detect palindromes. detect_offset_palindrome only detects right-offset palindromes,
            // so the input is reversed to check for left offsets.
            let rows_reversed: Vec<_> = rows.iter().rev().cloned().collect();
            let cols_reversed: Vec<_> = cols.iter().rev().cloned().collect();

            let to_check = [
                (&rows, DetectedBy::Rows),
                (&rows_reversed, DetectedBy::RowsReversed),
                (&cols, DetectedBy::Cols),
                (&cols_reversed, DetectedBy::ColsReversed),
            ];

            // Find the smudgeless reflection line
            let mut check_iter = to_check.iter();
            let (smudgeless_reflection, smudgeless_detected_by) = loop {
                let (data, detected_by) =
                    check_iter.next().expect("Failed to detect any palindrome");

                if let Some(num) = detect_offset_palindrome(data, false, 0) {
                    break (num, detected_by);
                }
            };

            // Find the smudged reflection by, which should be a different value from above
            let mut check_iter = to_check.iter();
            loop {
                let (data, detected_by) =
                    check_iter.next().expect("Failed to detect any palindrome");

                let value_to_ignore = if detected_by == smudgeless_detected_by {
                    smudgeless_reflection
                } else {
                    0
                };

                if let Some(num) = detect_offset_palindrome(data, true, value_to_ignore) {
                    if detected_by == smudgeless_detected_by && num == smudgeless_reflection {
                        continue;
                    }

                    break match detected_by {
                        DetectedBy::Rows => num * 100,
                        DetectedBy::RowsReversed => (data.len() - num) * 100,
                        DetectedBy::Cols => num,
                        DetectedBy::ColsReversed => data.len() - num,
                    };
                }
            }
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

        assert_eq!(res, 400);
    }

    #[test]
    fn test_normal_palindrome() {
        let a = [3, 2, 2, 3];
        let res = detect_offset_palindrome(&a, false, 0);

        assert_eq!(res, Some(2));
    }

    #[test]
    fn test_offset_palindrome() {
        let a = [3, 2, 3, 2, 2, 3];
        let res = detect_offset_palindrome(&a, false, 0);

        assert_eq!(res, Some(4));
    }

    #[test]
    fn test_no_palindrome() {
        let a = [3, 2, 3, 2, 3];
        let res = detect_offset_palindrome(&a, false, 0);

        assert_eq!(res, None);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 44615);
    }
}
