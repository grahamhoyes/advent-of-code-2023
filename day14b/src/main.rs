fn solution(input: &str) -> usize {
    let mut board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Tilt the board to the north
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'O' {
                // Progressively swap with elements above this one until we hit
                // another item
                for k in (0..i).rev() {
                    if board[k][j] != '.' {
                        break;
                    }

                    board[k + 1][j] = '.';
                    board[k][j] = 'O';
                }
            }
        }
    }

    board
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (i + 1))
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

        assert_eq!(res, 136);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 105249);
    }
}
