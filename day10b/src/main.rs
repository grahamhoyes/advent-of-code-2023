use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::*;
use std::iter::Iterator;
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord(i32, i32);

impl Coord {
    fn is_valid(&self) -> bool {
        self.0 >= 0 && self.1 >= 0
    }

    // Get a vector of the 8 neighbors of this cell.
    // This should really return an iterator rather than allocating
    // a vector, but I don't feel like creating the wrapper type to do that.
    fn neighbors(&self) -> Vec<Coord> {
        let mut neighbors = Vec::with_capacity(8);

        for x in [-1, 0, 1] {
            for y in [-1, 0, 1] {
                if x == 0 && y == 0 {
                    continue;
                }

                neighbors.push(*self + (x, y));
            }
        }

        neighbors
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<(i32, i32)> for Coord {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as i32, value.1 as i32)
    }
}

#[derive(Clone)]
struct Pipe {
    symbol: char,
    position: Coord,
    // Connected coordinates, in no particular order
    connected: [Coord; 2],
}

impl Pipe {
    /// Get the coordinates of the next pipe, given the one we came from
    fn next_from(&self, prev: &Pipe) -> Coord {
        if prev.position == self.connected[0] {
            self.connected[1]
        } else if prev.position == self.connected[1] {
            self.connected[0]
        } else {
            panic!(
                "Previous position {:?} is not connected to pipe at {:?}",
                prev.position, self.position
            );
        }
    }

    fn is_connected_to(&self, other: Coord) -> bool {
        self.connected.iter().any(|c| c == &other)
    }
}

enum Cell {
    Empty,
    Visited,
    Pipe(Pipe),
}

impl Cell {
    // Get the contained pipe, panicking if the cell doesn't contain a pipe
    fn pipe(&self) -> &Pipe {
        match self {
            Self::Empty | Self::Visited => panic!("Cell is not a pipe"),
            Self::Pipe(p) => p,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Visited => write!(f, "x"),
            Cell::Pipe(pipe) => {
                let c = match pipe.symbol {
                    'F' => '┌',
                    '7' => '┐',
                    'L' => '└',
                    'J' => '┘',
                    _ => pipe.symbol,
                };
                write!(f, "{}", c)
            }
        }
    }
}

struct Field {
    inner: Vec<Vec<Cell>>,
    start: Coord,
}

impl Field {
    fn from_input(input: &str) -> Self {
        // Position of the starting pipe, we'll come back and fill
        // this in later
        let mut start = Coord(-1, -1);

        let mut field: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let here: Coord = (x, y).into();

                        let connected = match c {
                            '.' => {
                                return Cell::Empty;
                            }
                            'S' => {
                                start = here;
                                // This will get set to a pipe later
                                return Cell::Empty;
                            }
                            '|' => [here - (0, 1), here + (0, 1)],
                            '-' => [here - (1, 0), here + (1, 0)],
                            'L' => [here - (0, 1), here + (1, 0)],
                            'J' => [here - (0, 1), here - (1, 0)],
                            '7' => [here - (1, 0), here + (0, 1)],
                            'F' => [here + (1, 0), here + (0, 1)],
                            _ => unreachable!(),
                        };

                        Cell::Pipe(Pipe {
                            symbol: c,
                            position: here,
                            connected,
                        })
                    })
                    .collect()
            })
            .collect();

        // From the start, we have to find the connected pipes to start our search
        let mut start_connected: Vec<Coord> = Vec::with_capacity(2);

        for coord in start.neighbors() {
            if !coord.is_valid() {
                continue;
            }

            if let Cell::Pipe(pipe) = &field[coord.1 as usize][coord.0 as usize] {
                if pipe.is_connected_to(start) {
                    start_connected.push(pipe.position);
                }
            }
        }

        assert_eq!(
            start_connected.len(),
            2,
            "Found {} neighbors for the starting pipe, should be exactly 2",
            start_connected.len()
        );

        // Insert the starting pipe into the playing field
        field[start.1 as usize][start.0 as usize] = Cell::Pipe(Pipe {
            symbol: 'S',
            position: start,
            connected: [start_connected[0], start_connected[1]],
        });

        Self {
            inner: field,
            start,
        }
    }

    fn get(&self, coord: &Coord) -> Option<&Cell> {
        if !coord.is_valid() {
            // Guards against casting negative integers to usize
            return None;
        }

        self.inner
            .get(coord.1 as usize)
            .and_then(|row| row.get(coord.0 as usize))
    }

    fn set(&mut self, coord: &Coord, cell: Cell) {
        if !coord.is_valid() {
            panic!("Tried to set at invalid coordinate {:?}", coord);
        }

        self.inner[coord.1 as usize][coord.0 as usize] = cell;
    }

    /// The field can have pipes that aren't part of the loop. Traverse
    /// the loop once to figure out what belongs, and delete anything
    /// that doesn't.
    fn clear_junk(&mut self) {
        let mut prev = self.get(&self.start).unwrap().pipe().clone();
        // Arbitrarily pick one of the next pipes to go to to set our direction
        let mut cur = self.get(&prev.connected[0]).unwrap().pipe().clone();

        let mut visited: HashSet<Coord> = HashSet::new();

        visited.insert(self.start);

        while cur.position != self.start {
            visited.insert(cur.position);
            let next_coord = cur.next_from(&prev);
            prev = cur;
            cur = self.get(&next_coord).unwrap().pipe().clone();
        }

        // Now, go over the board and delete anything that wasn't visited
        for (y, row) in self.inner.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if !visited.contains(&(x, y).into()) {
                    *cell = Cell::Empty;
                }
            }
        }
    }

    /// Perform a search for empty cells from a starting coordinate, in the process
    /// marking visited cells as visited. Returns the number of cells visited
    ///
    /// If the search encounters the edge of the board, returns a None variant.
    /// Otherwise, returns a Some variant with the number of empty cells visited
    /// (which will be marked as visited and never visited again).
    fn search_from(&mut self, coord: Coord) -> Option<usize> {
        // println!("=== Starting search from {:?} ===", coord);

        let mut frontier: Vec<Coord> = vec![coord];

        let mut count = 0;

        while let Some(coord) = frontier.pop() {
            // print!("\n{}", self);
            // println!("Frontier: {:?}", frontier);

            // Checks if the cell is on the board
            if let Some(cell) = self.get(&coord) {
                match cell {
                    Cell::Visited | Cell::Pipe(_) => continue,
                    Cell::Empty => {
                        count += 1;

                        // Enqueue all of this cell's neighbors (potentially ones
                        // we just came from, that's fine)
                        let neighbors = coord.neighbors();
                        // println!("Adding neighbors: {:?}", neighbors);
                        frontier.extend(neighbors);

                        // Mark the current cell as visited so we don't count it again
                        self.set(&coord, Cell::Visited);
                    }
                }
            } else {
                // Encountering a cell off the board means we got here
                // from one that was not bounded by the pipe. Hence, this whole
                // search is unbounded and can be discarded.
                return None;
            }
        }

        Some(count)
    }

    fn print_step(&self, current: &Coord, direction: &Coord) {
        for (y, row) in self.inner.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if &Coord(x as i32, y as i32) == current {
                    let c = match direction {
                        Coord(0, -1) => '^',
                        Coord(0, 1) => 'v',
                        Coord(1, 0) => '>',
                        Coord(-1, 0) => '<',
                        _ => unreachable!(),
                    };
                    print!("{}", c);
                } else {
                    print!("{}", cell);
                }
            }

            println!();
        }

        println!();
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.inner.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?
            }
            writeln!(f)?
        }

        writeln!(f)
    }
}

fn solution(input: &str) -> usize {
    let mut field = Field::from_input(input);

    field.clear_junk();

    // Since we're traversing the pipe in a fixed direction, either the "left"
    // or "right" of our traversal path will be the inside. To figure that out,
    // observe that the inside direction will never reach the outside of the board.
    // The outside direction will reach the outside at some point, but not
    // necessarily from every position.
    //
    // As we move around the pipe, we'll do a search for empty cells on its "left" and "right"
    // (depending on which direction we came from). Once we encounter the wall, we know that
    // side is the outside and can stop checking it.

    let mut contained_right = Some(0);
    let mut contained_left = Some(0);

    let mut prev = field.get(&field.start).unwrap().pipe().clone();
    // Arbitrarily pick one of the next pipes to go to to set our direction
    let mut cur = field.get(&prev.connected[0]).unwrap().pipe().clone();

    while cur.position != field.start {
        let dir = cur.position - prev.position;

        let delta_right = if dir.0 == 0 {
            Coord(-dir.1, dir.0)
        } else {
            Coord(dir.1, dir.0)
        };

        if let Some(count) = contained_right {
            let right_neighbor = cur.position + delta_right;

            if let Some(visited) = field.search_from(right_neighbor) {
                contained_right = Some(count + visited);
            } else {
                // Reached the edge of the board, so permanently end this count
                contained_right = None
            }
        }

        if let Some(count) = contained_left {
            let left_neighbor = cur.position - delta_right;

            if let Some(visited) = field.search_from(left_neighbor) {
                contained_left = Some(count + visited);
            } else {
                contained_left = None
            }
        }

        // field.print_step(&cur.position, &dir);
        // println!(
        //     "Current: {:?} | Contained right: {:?} | Contained left: {:?}",
        //     cur.position, contained_right, contained_left
        // );
        //
        // let mut buf = String::new();
        // io::stdin().read_line(&mut buf).unwrap();

        let next_coord = cur.next_from(&prev);
        prev = cur;
        cur = field.get(&next_coord).unwrap().pipe().clone();
    }

    println!("{}", field);

    println!(
        "Contained right: {:?}, contained left: {:?}",
        contained_right, contained_left
    );

    contained_right.or(contained_left).unwrap()
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
    fn test_example_1() {
        let input = include_str!("../example_1.txt");
        let res = solution(input);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("../example_2.txt");
        let res = solution(input);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_larger_example() {
        let input = include_str!("../example_3.txt");
        let res = solution(input);

        assert_eq!(res, 8);
    }

    #[test]
    fn test_larger_example_with_random_junk() {
        let input = include_str!("../example_4.txt");
        let res = solution(input);

        assert_eq!(res, 10);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 6842);
    }
}
