// We need to implement Dijkstra's algorithm, with the caveat that we can't
// move straight more than 3 times. In other words, the cost of moving straight
// after already having moved straight thrice is infinite.
//
// I don't see a way to add that condition into the interfaces exposed
// by the `pathfinding` crate (since the neighbors function doesn't get given the
// path so far), so we do it ourselves.
//
// Note that the property of not re-visiting a node is maintained. If we were to
// cross a straight path, we could have made a turn there originally with less cost.
// Performing a loop to cross an angled path is also longer than pathing to the next
// node directly.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;

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

const DIRECTIONS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

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

    /// Get the valid neighboring edge connections of a coordinate
    fn neighbors(&self, from: &Coord) -> Vec<Edge> {
        DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let target = from + dir;
                let next = self.get(&target)?;

                Some(Edge {
                    target,
                    direction: *dir,
                    cost: next,
                })
            })
            .collect()
    }
}

#[derive(Eq, PartialEq)]
struct Edge {
    /// Coordinate of the target node
    target: Coord,
    /// Direction travelled from the previous node
    direction: Dir,
    /// Cost incurred by this transition
    cost: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Visit {
    vertex: Coord,
    /// Total heat lost travelling from the start through this vertex
    heat_lost: usize,
}

// The binary heap we use to track edges depends on `Ord`. Explicitly implementing
// this trait with the comparison backwards so that we get a min-heap, rather than
// a max-heap.
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
    let mut board = Board::from_input(input);

    // This implementation of Dijkstra's algorithm is mostly from
    // https://codereview.stackexchange.com/a/202879
    let mut costs: HashMap<Coord, usize> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    costs.insert(Coord(0, 0), 0);
    to_visit.push(Visit {
        vertex: Coord(0, 0),
        heat_lost: 0,
    });

    while let Some(Visit { vertex, heat_lost }) = to_visit.pop() {
        if !visited.insert(vertex) {
            // Already been here
            continue;
        }

        for neighbor in board.neighbors(&vertex) {
            let new_cost = heat_lost + neighbor.cost;

            // TODO: If direction results in us going the same way 3 times, abort

            let is_cheaper = costs
                .get(&neighbor.target)
                .map_or(true, |&current| new_cost < current);

            if is_cheaper {
                costs.insert(neighbor.target, new_cost);
                to_visit.push(Visit {
                    vertex: neighbor.target,
                    heat_lost: new_cost,
                });
            }
        }
    }

    let (rows, cols) = board.size();

    costs[&Coord(rows as i32 - 1, cols as i32 - 1)]
}

fn main() {
    let input = include_str!("../example.txt");
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

        // This is what's returned by regular Dijkstra's as a reference point
        assert_eq!(res, 78);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 0);
    }
}
