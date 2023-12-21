use std::collections::{HashMap, HashSet, VecDeque};
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

impl Add<(i32, i32)> for &Coord {
    type Output = Coord;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
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
    fn get(&self, c: &Coord) -> Plot {
        let (rows, cols) = self.size();
        let mut row = (c.0 + self.start.0) % (rows as i32);
        let mut col = (c.1 + self.start.1) % (cols as i32);

        if row < 0 {
            row += rows as i32;
        }

        if col < 0 {
            col += cols as i32;
        }

        *self
            .inner
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .unwrap()
    }

    // Visualize the board with a given frontier. x and y are the number
    // of extra boards to the left and right to print.
    #[cfg(feature = "interactive")]
    fn visualize(&self, x: i32, y: i32, frontier: &HashSet<Coord>) {
        // Move cursor to 0, 0
        print!("\x1B[0;0H");

        let (rows, cols) = self.size();
        let rows = rows as i32;
        let cols = cols as i32;

        for y_copy in -y..=y {
            for row in 0..rows {
                for x_copy in -x..=x {
                    let origin = Coord(y_copy * rows - self.start.0, x_copy * cols - self.start.1);
                    for col in 0..cols {
                        let cell = &origin + (row, col);

                        if frontier.contains(&cell) {
                            print!("O")
                        } else {
                            let c = match self.get(&cell) {
                                Plot::Rock => '#',
                                Plot::Garden => '.',
                            };
                            print!("{}", c);
                        }
                    }
                    // print!("  ")
                }
                println!();
            }
            // println!();
        }

        // println!();
    }
}

/// Wait for an enter press
#[cfg(feature = "interactive")]
fn wait() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

/// Walk around the garden for a given number of steps. Returns a map
/// of coordinates to the step number that we reached that position at.
fn walk_around(input: &str, steps: usize) -> HashMap<Coord, usize> {
    let board = Board::from_input(input);

    let mut frontier: VecDeque<Coord> = VecDeque::new();
    let mut visited: HashMap<Coord, usize> = HashMap::new();

    let start = Coord(0, 0);
    frontier.push_back(start);
    visited.insert(start, 0);

    for step in 1..=steps {
        for _ in 0..frontier.len() {
            let coord = frontier.pop_front().unwrap();

            for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                let neighbor = &coord + dir;

                if visited.contains_key(&neighbor) {
                    continue;
                }

                if let Plot::Garden = board.get(&neighbor) {
                    frontier.push_back(neighbor);
                    visited.insert(neighbor, step);
                }
            }
        }

        #[cfg(feature = "interactive")]
        if (step - 65) % 131 == 0 {
            print!("\x1B[2J");
            board.visualize(2, 2, &HashSet::from_iter(visited.keys().cloned()));
            println!("Step {}", step);
            println!("Frontier length: {}", frontier.len());
            wait();
        }
    }

    visited
}

fn solution(input: &str, steps: usize) -> usize {
    let visited = walk_around(input, steps);

    let parity = (steps % 2) as i32;

    visited
        .keys()
        .filter(|coord| ((coord.0 + coord.1) % 2).abs() == parity)
        .count()
}

fn main() {
    // Clear the screen
    #[cfg(feature = "interactive")]
    print!("\x1B[2J");

    let input = include_str!("../input.txt");

    // See README.md - we use this result to compute the actual answer
    for i in 0..3 {
        let res = solution(input, i * 131 + 65);
        println!("{}: {}", i, res);
    }
    // let res = solution(input, 1000);

    // println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input, 100);

        assert_eq!(res, 6536);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input, 64);

        assert_eq!(res, 3816);
    }
}
