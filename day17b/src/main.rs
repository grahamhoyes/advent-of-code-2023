use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

struct Board {
    inner: Vec<Vec<usize>>,
}

impl Board {
    fn from_input(input: &str) -> Self {
        Self {
            inner: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        }
    }

    /// Return the number of rows and columns in the board
    fn size(&self) -> (usize, usize) {
        (self.inner.len(), self.inner[0].len())
    }

    /// Get the value of the board at the given coordinate, or None if the coordinates
    /// are off the board.
    fn get(&self, c: &Coord) -> Option<usize> {
        if c.0 < 0 || c.1 < 0 {
            return None;
        }

        self.inner
            .get(c.0 as usize)
            .and_then(|row| row.get(c.1 as usize))
            .cloned()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Visit {
    coord: Coord,
    /// Total heat lost travelling from the start through this vertex
    heat_lost: usize,
    /// The direction we came from
    direction: Dir,
}

// The binary heap we use to track edges depends on `Ord`. Explicitly implementing
// this trait with the comparison backwards so that we get a min-heap, rather than
// a max-heap. The only thing we order on is heat lost (the cost).
impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_lost.cmp(&self.heat_lost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solution(input: &str) -> usize {
    let board = Board::from_input(input);

    let (rows, cols) = board.size();
    let goal = Coord(rows as i32 - 1, cols as i32 - 1);

    let mut costs: HashMap<(Coord, Dir), usize> = HashMap::new();
    let mut visited: HashSet<(Coord, Dir)> = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    // We could travel either direction out of the starting cell
    for dir in [Dir::South, Dir::East] {
        costs.insert((Coord(0, 0), dir), 0);
        to_visit.push(Visit {
            coord: Coord(0, 0),
            heat_lost: 0,
            direction: dir,
        });
    }

    while let Some(Visit {
        coord,
        heat_lost,
        direction,
    }) = to_visit.pop()
    {
        if !visited.insert((coord, direction)) {
            // Already been here
            continue;
        }

        if coord == goal {
            return heat_lost;
        }

        let (y, x) = coord.into();

        // This prevents us from going in the same direction or backwards
        for new_direction in match direction {
            Dir::North | Dir::South => [Dir::East, Dir::West],
            Dir::East | Dir::West => [Dir::North, Dir::South],
        } {
            let mut new_cost = heat_lost;

            // Rather than letting us travel straight as a separate iteration,
            // enqueue all of the legal straight moves after going in new_direction at once.
            for steps in 1..=3 {
                let c = match new_direction {
                    Dir::North => Coord(y - steps, x),
                    Dir::East => Coord(y, x + steps),
                    Dir::South => Coord(y + steps, x),
                    Dir::West => Coord(y, x - steps),
                };

                if let Some(extra_cost) = board.get(&c) {
                    new_cost += extra_cost;

                    let is_cheaper = costs
                        .get(&(c, new_direction))
                        .map_or(true, |&current| new_cost < current);

                    if is_cheaper {
                        costs.insert((c, new_direction), new_cost);
                        to_visit.push(Visit {
                            coord: c,
                            heat_lost: new_cost,
                            direction: new_direction,
                        })
                    }
                }
            }
        }
    }

    panic!("Could not find target node")
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

        assert_eq!(res, 102);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1128);
    }
}
