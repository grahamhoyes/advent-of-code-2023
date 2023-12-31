use std::collections::HashSet;
use std::ops::Add;

// There are a lot of similarities between these types and what we did in day 10
struct Board {
    inner: Vec<Vec<char>>,
}

impl Board {
    fn from_input(input: &str) -> Self {
        Self {
            inner: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn get(&self, c: &Coord) -> Option<char> {
        if c.0 < 0 || c.1 < 0 {
            return None;
        }

        self.inner
            .get(c.0 as usize)
            .and_then(|row| row.get(c.1 as usize))
            .cloned()
    }

    #[cfg(feature = "interactive")]
    fn print_at(&self, coord: &Coord, dir: &Dir) {
        // Move cursor to 0, 0
        print!("\x1B[0;0H");

        let direction_symbol = match dir {
            Dir::North => '^',
            Dir::East => '>',
            Dir::South => 'v',
            Dir::West => '<',
        };

        for (i, row) in self.inner.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if &Coord(i as i32, j as i32) == coord {
                    print!("{}", direction_symbol)
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

/// A (row, col) coordinate pair or vector. Using i32 so that we can subtract
/// when needed, but only positive values are valid.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord(i32, i32);

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Add<&Dir> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Dir) -> Self::Output {
        match rhs {
            Dir::North => Coord(self.0 - 1, self.1),
            Dir::East => Coord(self.0, self.1 + 1),
            Dir::South => Coord(self.0 + 1, self.1),
            Dir::West => Coord(self.0, self.1 - 1),
        }
    }
}

impl Add<&Dir> for Coord {
    type Output = Self;

    #[allow(clippy::op_ref)]
    fn add(self, rhs: &Dir) -> Self::Output {
        &self + rhs
    }
}

/// Wait for an enter press
#[cfg(feature = "interactive")]
fn wait() {
    use std::io::stdin;
    stdin().read_line(&mut String::new()).unwrap();
}

fn solution(input: &str) -> usize {
    // Set of coordinates and directions of illuminated cells, used to detect
    // loops.
    let mut illuminated: HashSet<(Coord, Dir)> = HashSet::new();

    let board = Board::from_input(input);

    // Forked beams we have to keep track of, from their starting coordinate and direction
    let mut beams: Vec<(Coord, Dir)> = vec![(Coord(0, 0), Dir::East)];

    while let Some((start, dir_start)) = beams.pop() {
        let mut coord = start;
        let mut dir = dir_start;

        while let Some(char) = board.get(&coord) {
            let key = (coord, dir);
            if illuminated.contains(&key) {
                break;
            } else {
                illuminated.insert(key);
            }

            #[cfg(feature = "interactive")]
            {
                board.print_at(&coord, &dir);
                println!("Beams: {:?}", beams);
                crate::wait();
            }

            // dir is the direction we're going, not the direction we're coming from
            dir = match (char, &dir) {
                ('.', _) => dir,
                ('/', Dir::North) => Dir::East,
                ('/', Dir::East) => Dir::North,
                ('/', Dir::South) => Dir::West,
                ('/', Dir::West) => Dir::South,
                ('\\', Dir::North) => Dir::West,
                ('\\', Dir::East) => Dir::South,
                ('\\', Dir::South) => Dir::East,
                ('\\', Dir::West) => Dir::North,
                ('-', Dir::East) | ('-', Dir::West) => dir,
                ('-', Dir::North) | ('-', Dir::South) => {
                    // Enqueue the west direction, proceed with east
                    beams.push((&coord + &Dir::West, Dir::West));
                    Dir::East
                }
                ('|', Dir::North) | ('|', Dir::South) => dir,
                ('|', Dir::East) | ('|', Dir::West) => {
                    // Enqueue the south direction, proceed with north
                    beams.push((&coord + &Dir::South, Dir::South));
                    Dir::North
                }
                (c, d) => panic!("Unrecognized combination ({}, {:?})", c, d),
            };

            coord = coord + &dir;
        }
    }

    illuminated
        .iter()
        .map(|(coord, _)| coord)
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    // Clear the screen
    #[cfg(feature = "interactive")]
    print!("\x1B[2J");

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

        assert_eq!(res, 46);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 7199);
    }
}
