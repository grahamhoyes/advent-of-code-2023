use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    fn is_valid(&self) -> bool {
        self.0 >= 0 && self.1 >= 0
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<(i32, i32)> for Coord {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

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
                        let here = Coord(x as i32, y as i32);

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

        for x in [-1, 0, 1] {
            for y in [-1, 0, 1] {
                let coord = start + (x, y);

                if !coord.is_valid() {
                    continue;
                }

                if let Cell::Pipe(pipe) = &field[coord.1 as usize][coord.0 as usize] {
                    if pipe.is_connected_to(start) {
                        start_connected.push(pipe.position);
                    }
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
            return None;
        }

        Some(&self.inner[coord.1 as usize][coord.0 as usize])
    }

    fn get_start(&self) -> &Pipe {
        self.inner[self.start.1 as usize][self.start.0 as usize].pipe()
    }
}

fn solution(input: &str) -> usize {
    let field = Field::from_input(input);

    // To figure out the number of tiles enclosed by the pipe, we could traverse
    // in a known direction, say clockwise, and do breadth first search on empty
    // tiles to the "right" of each - or | pipe, counting the ones we haven't seen
    // before.

    // Figure out the length of the pipe by traversing it
    let mut len = 1;
    let mut prev = field.get(&field.start).unwrap().pipe();

    // Arbitrarily pick one of the next pipes to go to to set our direction
    let mut cur = field.get(&field.get_start().connected[1]).unwrap().pipe();

    while cur.position != field.start {
        let next_coord = cur.next_from(prev);
        prev = cur;
        cur = field.get(&next_coord).unwrap().pipe();
        len += 1;
    }

    len / 2
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

        assert_eq!(res, 8);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 6842);
    }
}
