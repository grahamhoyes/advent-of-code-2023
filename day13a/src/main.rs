/// Detect a palindrome that is offset to the right, and return the
/// number of elements to the left of the axis of symmetry.
///
/// To detect a left offset, reverse the input.
fn detect_offset_palindrome(items: &[usize]) -> Option<usize> {
    // An offset palindrome will always reach the end of one side
    // of the array, so start there until we find a match

    let mut i_start = 0;
    let mut j = items.len() - 1;

    // Advance the left pointer until it matches the last element
    while items[i_start] != items[j] && i_start < j {
        i_start += 1;
    }

    if i_start == j {
        // Went through the whole list and didn't find a match
        return None;
    }

    // There is a case where the left side happened to match the end,
    // but not because it's in the palindrome. For example:
    //   3 2 3 2 1 1 2 3
    // The left 3 2 aren't part of the palindrome, so we need to
    // progressively ignore them.

    while i_start < j {
        let mut i = i_start;

        while items[i] == items[j] && i < j {
            i += 1;
            j -= 1;
        }

        // This problem in particular is for even number palindromes - the
        // axis of symmetry must be between two elements, not on one. For a
        // normal palindrome, this should be i >= j.
        if i > j {
            return Some(i);
        }

        // Start the search again, from one more element further from the left
        j = items.len() - 1;
        i_start += 1;
    }

    None
}

fn solution(input: &str) -> usize {
    // We basically need an offset palindrome recognizer. To make it easier,
    // we start by treating each row/column as a binary sequence and converting each
    // to an integer to make later processing easier. This works as long as the
    // inputs are less than 64 wide/tall - any other hashing function would be fine too.
    input
        .split("\n\n")
        .map(|block| {
            // Hash each row / column into an integer

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

            if let Some(num) = detect_offset_palindrome(&rows) {
                num * 100
            } else if let Some(num) =
                detect_offset_palindrome(&rows.iter().rev().cloned().collect::<Vec<_>>())
            {
                (rows.len() - num) * 100
            } else if let Some(num) = detect_offset_palindrome(&cols) {
                num
            } else if let Some(num) =
                detect_offset_palindrome(&cols.iter().rev().cloned().collect::<Vec<_>>())
            {
                cols.len() - num
            } else {
                unreachable!("Failed to detect any palindrome");
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

        assert_eq!(res, 405);
    }

    #[test]
    fn test_normal_palindrome() {
        let a = [3, 2, 2, 3];
        let res = detect_offset_palindrome(&a);

        assert_eq!(res, Some(2));
    }

    #[test]
    fn test_offset_palindrome() {
        let a = [3, 2, 3, 2, 2, 3];
        let res = detect_offset_palindrome(&a);

        assert_eq!(res, Some(4));
    }

    #[test]
    fn test_no_palindrome() {
        let a = [3, 2, 3, 2, 3];
        let res = detect_offset_palindrome(&a);

        assert_eq!(res, None);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 30705);
    }
}
