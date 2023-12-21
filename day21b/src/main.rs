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
        let mut row = c.0 % (rows as i32);
        let mut col = c.1 % (cols as i32);

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
                    let origin = Coord(y_copy * rows, x_copy * cols);
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
                }
                println!();
            }
        }
    }
}

/// Wait for an enter press
#[cfg(feature = "interactive")]
fn wait() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn solution(input: &str, steps: usize) -> usize {
    let board = Board::from_input(input);

    // To figure out the number of places we could be after the given
    // number of steps, we just need to do BFS and clearing the visited
    // set on each turn.
    let mut frontier: HashSet<Coord> = HashSet::new();
    frontier.insert(board.start);

    for i in 0..steps {
        if i % 10 == 0 {
            println!("{}", i);
        }
        let mut next_frontier: HashSet<Coord> = HashSet::new();

        for coord in frontier.iter() {
            for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                let neighbor = coord + dir;
                if let Plot::Garden = board.get(&neighbor) {
                    next_frontier.insert(neighbor);
                }
            }
        }

        frontier = next_frontier;

        #[cfg(feature = "interactive")]
        {
            board.visualize(2, 2, &frontier);
            wait();
        }
    }

    0
}

fn main() {
    // Clear the screen
    #[cfg(feature = "interactive")]
    print!("\x1B[2J");

    let input = include_str!("../example.txt");
    let res = solution(input, 5000);

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
