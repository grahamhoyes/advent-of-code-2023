use std::collections::HashSet;

// Same solution as part 1, we just multiply the number of offset rows by
// an expansion factor. Removed the board printing functionality, since we
// aren't going to print a board this big.

fn solution(input: &str, expansion_factor: usize) -> usize {
    // Parse the input into a sparse vector of (row, column) tuples representing
    // galaxies in the observed coordinates
    let mut galaxies: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '.' { None } else { Some((y, x)) })
        })
        .collect();

    // Figure out the original dimensions of the board to help us
    let num_rows: usize = input.lines().fold(0, |accum, _| accum + 1);
    let num_cols: usize = input.lines().next().unwrap().len();

    // Determine which rows and columns don't have any galaxies, so we can expand
    // the universe
    let mut empty_rows: HashSet<_> = (0..num_rows).collect();
    let mut empty_cols: HashSet<_> = (0..num_cols).collect();

    for (row, col) in galaxies.iter() {
        empty_rows.remove(row);
        empty_cols.remove(col);
    }

    // Perform the expansion, using the same sparse structure for galaxies
    galaxies.iter_mut().for_each(|(row, col)| {
        let empty_rows_before = empty_rows
            .iter()
            .filter(|r| *r < row)
            .collect::<Vec<_>>()
            .len();
        let empty_cols_before = empty_cols
            .iter()
            .filter(|c| *c < col)
            .collect::<Vec<_>>()
            .len();

        // The only change from part 1
        *row += empty_rows_before * (expansion_factor - 1);
        *col += empty_cols_before * (expansion_factor - 1);
    });

    // Finally, pair off galaxies and compute the Manhattan distance between each.
    let mut total_distance = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let a = galaxies[i];
            let b = galaxies[j];

            total_distance += a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
        }
    }

    total_distance
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input, 1_000_000);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input, 100);

        assert_eq!(res, 8410);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input, 1_000_000);

        assert_eq!(res, 622120986954);
    }
}
