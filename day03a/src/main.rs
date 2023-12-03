fn is_symbol(c: &char) -> bool {
    *c != '.' && !char::is_numeric(*c) && !char::is_alphabetic(*c)
}

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

    // Now that we have all hypothetical part numbers, figure out which ones are adjacent
    // to a symbol
    parts.retain(|part| {
        // Iterate over the box surrounding the number. This will also iterate over the
        // part number itself which is redundant, but that's fine.
        for row in part.line - 1..=part.line + 1 {
            for col in part.col - 1..=part.col + (part.length as i32) {
                if let Some(char) = schematic
                    .get(row as usize)
                    .and_then(|r| r.get(col as usize))
                {
                    if is_symbol(char) {
                        return true;
                    }
                }
            }
        }

        false
    });

    parts.iter().map(|p| p.val).sum()
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

    assert_eq!(res, 4361);
}

#[test]
fn test_input() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    assert_eq!(res, 556057);
}
