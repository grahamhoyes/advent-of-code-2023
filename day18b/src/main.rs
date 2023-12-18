use std::ops::{Add, Mul};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::North,
            "R" => Self::East,
            "D" => Self::South,
            "L" => Self::West,
            v => panic!("Unrecognized direction {}", v),
        }
    }
}

impl Add<Dir> for Coord {
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

// A direction stepped in a given direction
#[derive(Debug, Clone, Copy)]
struct SteppedDir {
    dir: Dir,
    step: usize,
}

impl From<Dir> for SteppedDir {
    fn from(dir: Dir) -> Self {
        Self { dir, step: 1 }
    }
}

impl Mul<usize> for SteppedDir {
    type Output = SteppedDir;

    fn mul(self, step: usize) -> Self::Output {
        Self {
            dir: self.dir,
            step: self.step * step,
        }
    }
}

impl Mul<usize> for Dir {
    type Output = SteppedDir;

    fn mul(self, step: usize) -> Self::Output {
        Self::Output { dir: self, step }
    }
}

impl Add<SteppedDir> for Coord {
    type Output = Coord;

    fn add(self, rhs: SteppedDir) -> Self::Output {
        let step = rhs.step as i32;
        match rhs.dir {
            Dir::North => Coord(self.0 - step, self.1),
            Dir::East => Coord(self.0, self.1 + step),
            Dir::South => Coord(self.0 + step, self.1),
            Dir::West => Coord(self.0, self.1 - step),
        }
    }
}

fn solution(input: &str) -> u64 {
    let directions: Vec<SteppedDir> = input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let dir = parts.next().unwrap();
            let step = parts.next().unwrap().parse::<usize>().unwrap();

            Dir::from(dir) * step
        })
        .collect();

    let perimeter = directions
        .iter()
        .map(|stepped_dir| stepped_dir.step)
        .sum::<usize>() as u64;

    let mut points = directions
        .iter()
        // Convert the directions into a list of coordinates
        .fold(vec![Coord(0, 0)], |mut points, stepped_dir| {
            let next = *points.last().unwrap() + *stepped_dir;
            points.push(next);

            points
        });

    // Since the directions form an enclosed region, the last point should coincide
    // with the first.
    assert_eq!(points.first(), points.last());
    points.pop();

    // Use the shoelace formula on pairs of coordinates to compute the enclosed area
    let shoelace_area = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
        .map(|(p0, p1)| (p0.0 + p1.0) as i64 * (p0.1 - p1.1) as i64)
        .sum::<i64>()
        .unsigned_abs()
        / 2;

    // Since our perimeter is "thick", we can imagine that one half of it (say the top and
    // right sides) are enclosed in the computed area, and the other half (bottom and left
    // sides) aren't. To get the total area including the perimeter, we need to add the
    // missing half back in. The +1 is probably to account for a missing corner by similar
    // logic (counted 2, added 1 back from our half a perimeter, need one more if the
    // polygon were a rectangle), but I just found it was needed by inspection.
    shoelace_area + (perimeter / 2) + 1
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

        assert_eq!(res, 62);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 36679);
    }
}
