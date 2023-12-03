use std::collections::HashMap;

struct Part {
    /// Line the part is on
    line: i32,
    /// Column the part starts on
    col: i32,
    /// Length of the part
    length: usize,
    /// Part value (parsed)
    val: u32,
}

fn solution(input: &str) -> u32 {
    let schematic: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut parts: Vec<Part> = Vec::new();

    // Find all numbers in the schematic, don't worry if they're parts yet
    for (lineno, line) in schematic.iter().enumerate() {
        let mut colno = 0;
        while colno < line.len() {
            if !char::is_numeric(line[colno]) {
                colno += 1;
                continue;
            }

            let start = colno;

            // Found the start of a part number, advance until past the number
            while colno < line.len() && char::is_numeric(line[colno]) {
                colno += 1;
            }

            let val = line[start..colno]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            let p = Part {
                line: lineno as i32,
                col: start as i32,
                length: colno - start,
                val,
            };

            parts.push(p);
        }
    }

    // Map from position -> (count of adjacent neighbors, product of neighbors)
    let mut gears: HashMap<(i32, i32), (usize, u32)> = HashMap::new();

    // Now that we have all hypothetical part numbers, figure out which ones are adjacent
    // to a symbol
    for part in parts.iter() {
        // Iterate over the box surrounding the number. This will also iterate over the
        // part number itself which is redundant, but that's fine.
        for row in part.line - 1..=part.line + 1 {
            for col in part.col - 1..=part.col + (part.length as i32) {
                if let Some(char) = schematic
                    .get(row as usize)
                    .and_then(|r| r.get(col as usize))
                {
                    if *char == '*' {
                        if let Some(gear) = gears.get_mut(&(row, col)) {
                            gear.0 += 1;
                            gear.1 *= part.val;
                        } else {
                            gears.insert((row, col), (1, part.val));
                        }
                    }
                }
            }
        }
    }

    gears
        .iter()
        .filter_map(
            |(_coords, (neighbors, ratio))| {
                if *neighbors == 2 {
                    Some(ratio)
                } else {
                    None
                }
            },
        )
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");
    let res = solution(input);

    assert_eq!(res, 467835);
}

#[test]
fn test_input() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    assert_eq!(res, 82824352);
}
