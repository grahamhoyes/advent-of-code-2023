use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

enum Dir {
    North,
    South,
    East,
    West,
}

type Board = Vec<Vec<char>>;
type I = Box<dyn Iterator<Item = usize>>;

fn rotate(board: &mut Board, dir: Dir) {
    let rows = board.len();
    let cols = board[0].len();

    let row_iter: I = match dir {
        Dir::South => Box::new((0..rows).rev()),
        _ => Box::new(0..rows),
    };

    for i in row_iter {
        let col_iter: I = match dir {
            Dir::East => Box::new((0..cols).rev()),
            _ => Box::new(0..cols),
        };

        for j in col_iter {
            if board[i][j] == 'O' {
                let swap_iter: I = match dir {
                    Dir::North => Box::new((0..i).rev()),
                    Dir::South => Box::new((i + 1)..rows),
                    Dir::East => Box::new((j + 1)..cols),
                    Dir::West => Box::new((0..j).rev()),
                };

                let source_offset: i32 = match dir {
                    Dir::North | Dir::West => 1,
                    Dir::South | Dir::East => -1,
                };

                for k in swap_iter {
                    let (target_row, target_col) = match dir {
                        Dir::North | Dir::South => (k, j),
                        Dir::East | Dir::West => (i, k),
                    };

                    if board[target_row][target_col] != '.' {
                        break;
                    }

                    let (source_row, source_col) = match dir {
                        Dir::North | Dir::South => ((k as i32 + source_offset) as usize, j),
                        Dir::East | Dir::West => (i, (k as i32 + source_offset) as usize),
                    };

                    board[source_row][source_col] = '.';
                    board[target_row][target_col] = 'O';
                }
            }
        }
    }
}

fn rotate_cycle(board: &mut Board) {
    rotate(board, Dir::North);
    rotate(board, Dir::West);
    rotate(board, Dir::South);
    rotate(board, Dir::East);
}

fn hash_board(board: &Board) -> u64 {
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    hasher.finish()
}

const ITERATIONS: usize = 1_000_000_000;

fn solution(input: &str) -> usize {
    let mut board: Board = input.lines().map(|l| l.chars().collect()).collect();

    // Map of hash to the last iteration we saw it in, used to detect
    // cycles.
    let mut hash_states: HashMap<u64, usize> = HashMap::new();

    let mut i = 0;
    let mut cycle_length = usize::MAX;

    while i < ITERATIONS {
        rotate_cycle(&mut board);

        let hash = hash_board(&board);

        if let Some(state) = hash_states.get(&hash) {
            cycle_length = i - state;
            break;
        }

        hash_states.insert(hash, i);
        i += 1;
    }

    // Jump ahead past all the cycles, then perform the last few
    // iterations
    i += ((ITERATIONS - i) / cycle_length) * cycle_length + 1;
    while i < ITERATIONS {
        rotate_cycle(&mut board);
        i += 1;
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

        assert_eq!(res, 64);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 88680);
    }
}
