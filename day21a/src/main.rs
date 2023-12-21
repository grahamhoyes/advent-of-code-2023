use std::collections::HashSet;
use std::ops::Add;

/// A (row, col) coordinate pair or vector. Using i32 so that we can subtract
/// when needed, but only positive values are valid.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord(i32, i32);

impl From<Coord> for (i32, i32) {
    fn from(value: Coord) -> Self {
        (value.0, value.1)
    }
}

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Add<Dir> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::North => Coord(self.0 - 1, self.1),
            Dir::East => Coord(self.0, self.1 + 1),
            Dir::South => Coord(self.0 + 1, self.1),
            Dir::West => Coord(self.0, self.1 - 1),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
enum Plot {
    Rock,
    Garden,
}

#[derive(Debug)]
struct Board {
    inner: Vec<Vec<Plot>>,
    start: Coord,
}

impl Board {
    fn from_input(input: &str) -> Self {
        let mut start = Coord(0, 0);

        Self {
            inner: input
                .lines()
                .enumerate()
                .map(|(row, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(col, c)| match c {
                            '#' => Plot::Rock,
                            '.' => Plot::Garden,
                            'S' => {
                                start = Coord(row as i32, col as i32);
                                Plot::Garden
                            }
                            x => panic!("Unrecognized symbol {}", x),
                        })
                        .collect()
                })
                .collect(),
            start,
        }
    }

    /// Return the number of rows and columns in the board
    fn size(&self) -> (usize, usize) {
        (self.inner.len(), self.inner[0].len())
    }

    /// Get the value of the board at the given coordinate, or None if the coordinates
    /// are off the board.
    fn get(&self, c: &Coord) -> Option<Plot> {
        if c.0 < 0 || c.1 < 0 {
            return None;
        }

        self.inner
            .get(c.0 as usize)
            .and_then(|row| row.get(c.1 as usize))
            .cloned()
    }
}

fn solution(input: &str, steps: usize) -> usize {
    let board = Board::from_input(input);

    // To figure out the number of places we could be after the given
    // number of steps, we just need to do BFS and clearing the visited
    // set on each turn.
    let mut frontier: HashSet<Coord> = HashSet::new();
    frontier.insert(board.start);

    for _ in 0..steps {
        let mut next_frontier: HashSet<Coord> = HashSet::new();

        for coord in frontier.iter() {
            for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                let neighbor = coord + dir;
                if let Some(Plot::Garden) = board.get(&neighbor) {
                    next_frontier.insert(neighbor);
                }
            }
        }

        frontier = next_frontier;
    }

    frontier.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input, 64);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input, 6);

        assert_eq!(res, 16);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input, 64);

        assert_eq!(res, 3816);
    }
}
